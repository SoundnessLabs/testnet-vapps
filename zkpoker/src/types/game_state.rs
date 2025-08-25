use crate::types::{Card, Player, PlayerAction, ActionType, Hand};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameState {
    pub game_id: Uuid,
    pub players: Vec<Player>,
    pub deck: Vec<Card>,
    pub community_cards: Vec<Card>,
    pub pot: u64,
    pub current_bet: u64,
    pub dealer_position: usize,
    pub current_player: usize,
    pub stage: GameStage,
    pub small_blind: u64,
    pub big_blind: u64,
    pub actions_this_round: Vec<PlayerAction>,
    pub hand_number: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GameStage {
    WaitingForPlayers,
    PreFlop,
    Flop,
    Turn,
    River,
    Showdown,
    HandComplete,
}

impl GameState {
    pub fn new(game_id: Uuid, small_blind: u64, big_blind: u64) -> Self {
        Self {
            game_id,
            players: Vec::new(),
            deck: crate::types::card::create_deck(),
            community_cards: Vec::new(),
            pot: 0,
            current_bet: 0,
            dealer_position: 0,
            current_player: 0,
            stage: GameStage::WaitingForPlayers,
            small_blind,
            big_blind,
            actions_this_round: Vec::new(),
            hand_number: 0,
        }
    }

    pub fn add_player(&mut self, player: Player) -> Result<(), String> {
        if self.players.len() >= 9 {
            return Err("Table is full".to_string());
        }
        
        if self.stage != GameStage::WaitingForPlayers {
            return Err("Cannot join game in progress".to_string());
        }
        
        self.players.push(player);
        Ok(())
    }

    pub fn can_start_hand(&self) -> bool {
        self.players.len() >= 2 && 
        self.players.iter().filter(|p| p.can_act()).count() >= 2
    }

    pub fn start_new_hand(&mut self) -> Result<(), String> {
        if !self.can_start_hand() {
            return Err("Not enough players to start hand".to_string());
        }

        // Reset for new hand
        self.hand_number += 1;
        self.pot = 0;
        self.current_bet = 0;
        self.community_cards.clear();
        self.actions_this_round.clear();
        self.deck = crate::types::card::create_deck();
        
        // Reset players
        for player in &mut self.players {
            player.reset_for_new_hand();
        }

        // Set positions
        self.set_positions();
        
        // Shuffle deck (simplified - would use verifiable shuffle)
        self.shuffle_deck();
        
        // Deal hole cards
        self.deal_hole_cards();
        
        // Post blinds
        self.post_blinds()?;
        
        self.stage = GameStage::PreFlop;
        self.set_current_player_preflop();
        
        Ok(())
    }

    fn set_positions(&mut self) {
        let active_players: Vec<usize> = self.players.iter()
            .enumerate()
            .filter(|(_, p)| p.can_act())
            .map(|(i, _)| i)
            .collect();

        if active_players.len() < 2 {
            return;
        }

        // Move dealer button
        self.dealer_position = (self.dealer_position + 1) % active_players.len();
        let dealer_idx = active_players[self.dealer_position];
        self.players[dealer_idx].is_dealer = true;

        // Set blinds
        if active_players.len() == 2 {
            // Heads up: dealer is small blind
            self.players[dealer_idx].is_small_blind = true;
            let bb_idx = active_players[(self.dealer_position + 1) % active_players.len()];
            self.players[bb_idx].is_big_blind = true;
        } else {
            // Normal play
            let sb_idx = active_players[(self.dealer_position + 1) % active_players.len()];
            self.players[sb_idx].is_small_blind = true;
            let bb_idx = active_players[(self.dealer_position + 2) % active_players.len()];
            self.players[bb_idx].is_big_blind = true;
        }
    }

    fn shuffle_deck(&mut self) {
        use rand::seq::SliceRandom;
        use rand::rngs::StdRng;
        use rand::SeedableRng;
        
        // In production, this would use verifiable random shuffling
        let mut rng = StdRng::from_entropy();
        self.deck.shuffle(&mut rng);
    }

    fn deal_hole_cards(&mut self) {
        let mut card_index = 0;
        
        // Deal 2 cards to each active player
        for _ in 0..2 {
            for player in &mut self.players {
                if player.can_act() && card_index < self.deck.len() {
                    if player.hole_cards.is_none() {
                        player.hole_cards = Some([self.deck[card_index], self.deck[card_index + 1]]);
                        card_index += 2;
                        break;
                    }
                }
            }
        }
        
        // Actually deal properly
        card_index = 0;
        for player in &mut self.players {
            if player.can_act() {
                player.set_hole_cards([self.deck[card_index], self.deck[card_index + 1]]);
                card_index += 2;
            }
        }
        
        // Remove dealt cards from deck
        self.deck.drain(0..card_index);
    }

    fn post_blinds(&mut self) -> Result<(), String> {
        // Find and post small blind
        if let Some(sb_player) = self.players.iter_mut().find(|p| p.is_small_blind) {
            sb_player.bet(self.small_blind)?;
            self.pot += self.small_blind;
        }

        // Find and post big blind
        if let Some(bb_player) = self.players.iter_mut().find(|p| p.is_big_blind) {
            bb_player.bet(self.big_blind)?;
            self.pot += self.big_blind;
            self.current_bet = self.big_blind;
        }

        Ok(())
    }

    fn set_current_player_preflop(&mut self) {
        let active_players: Vec<usize> = self.players.iter()
            .enumerate()
            .filter(|(_, p)| p.can_act())
            .map(|(i, _)| i)
            .collect();

        if active_players.len() >= 3 {
            // First to act is UTG (after big blind)
            let bb_pos = self.players.iter().position(|p| p.is_big_blind).unwrap();
            self.current_player = (bb_pos + 1) % self.players.len();
        } else {
            // Heads up: small blind acts first preflop
            let sb_pos = self.players.iter().position(|p| p.is_small_blind).unwrap();
            self.current_player = sb_pos;
        }
    }

    pub fn process_action(&mut self, player_id: Uuid, action: ActionType) -> Result<(), String> {
        let player_idx = self.players.iter()
            .position(|p| p.id == player_id)
            .ok_or("Player not found")?;

        if player_idx != self.current_player {
            return Err("Not player's turn".to_string());
        }

        let player = &mut self.players[player_idx];
        
        if !player.can_act() {
            return Err("Player cannot act".to_string());
        }

        // Process the action
        match action {
            ActionType::Fold => {
                player.fold();
            },
            ActionType::Check => {
                if self.current_bet > player.current_bet {
                    return Err("Cannot check, must call or fold".to_string());
                }
            },
            ActionType::Call => {
                let call_amount = self.current_bet - player.current_bet;
                player.bet(call_amount)?;
                self.pot += call_amount;
            },
            ActionType::Bet(amount) => {
                if self.current_bet > 0 {
                    return Err("Cannot bet, there's already a bet".to_string());
                }
                player.bet(amount)?;
                self.pot += amount;
                self.current_bet = amount;
            },
            ActionType::Raise(amount) => {
                if amount <= self.current_bet {
                    return Err("Raise amount must be greater than current bet".to_string());
                }
                let total_amount = amount - player.current_bet;
                player.bet(total_amount)?;
                self.pot += total_amount;
                self.current_bet = amount;
            },
            ActionType::AllIn => {
                let all_in_amount = player.chips;
                player.bet(all_in_amount)?;
                self.pot += all_in_amount;
                if player.current_bet > self.current_bet {
                    self.current_bet = player.current_bet;
                }
            },
        }

        // Record the action
        self.actions_this_round.push(PlayerAction {
            player_id,
            action_type: action,
            amount: player.current_bet,
            timestamp: chrono::Utc::now(),
        });

        // Move to next player
        self.advance_to_next_player();

        // Check if betting round is complete
        if self.is_betting_round_complete() {
            self.advance_to_next_stage();
        }

        Ok(())
    }

    fn advance_to_next_player(&mut self) {
        let active_players: Vec<usize> = self.players.iter()
            .enumerate()
            .filter(|(_, p)| p.can_act())
            .map(|(i, _)| i)
            .collect();

        if active_players.is_empty() {
            return;
        }

        let current_pos = active_players.iter()
            .position(|&i| i == self.current_player)
            .unwrap_or(0);
        
        self.current_player = active_players[(current_pos + 1) % active_players.len()];
    }

    fn is_betting_round_complete(&self) -> bool {
        let active_players: Vec<&Player> = self.players.iter()
            .filter(|p| p.can_act())
            .collect();

        if active_players.len() <= 1 {
            return true;
        }

        // Check if all active players have acted and matched the current bet
        active_players.iter().all(|p| {
            p.current_bet == self.current_bet || p.status == crate::types::player::PlayerStatus::AllIn
        })
    }

    fn advance_to_next_stage(&mut self) {
        // Reset current bets for next round
        for player in &mut self.players {
            player.current_bet = 0;
        }
        self.current_bet = 0;
        self.actions_this_round.clear();

        match self.stage {
            GameStage::PreFlop => {
                self.deal_flop();
                self.stage = GameStage::Flop;
            },
            GameStage::Flop => {
                self.deal_turn();
                self.stage = GameStage::Turn;
            },
            GameStage::Turn => {
                self.deal_river();
                self.stage = GameStage::River;
            },
            GameStage::River => {
                self.stage = GameStage::Showdown;
            },
            GameStage::Showdown => {
                self.determine_winners();
                self.stage = GameStage::HandComplete;
            },
            _ => {}
        }

        // Set first to act (small blind or first active player after dealer)
        if matches!(self.stage, GameStage::Flop | GameStage::Turn | GameStage::River) {
            self.set_current_player_postflop();
        }
    }

    fn deal_flop(&mut self) {
        // Burn one card
        if !self.deck.is_empty() {
            self.deck.remove(0);
        }
        
        // Deal 3 community cards
        for _ in 0..3 {
            if !self.deck.is_empty() {
                self.community_cards.push(self.deck.remove(0));
            }
        }
    }

    fn deal_turn(&mut self) {
        // Burn one card
        if !self.deck.is_empty() {
            self.deck.remove(0);
        }
        
        // Deal 1 community card
        if !self.deck.is_empty() {
            self.community_cards.push(self.deck.remove(0));
        }
    }

    fn deal_river(&mut self) {
        // Burn one card
        if !self.deck.is_empty() {
            self.deck.remove(0);
        }
        
        // Deal 1 community card
        if !self.deck.is_empty() {
            self.community_cards.push(self.deck.remove(0));
        }
    }

    fn set_current_player_postflop(&mut self) {
        // First to act post-flop is small blind (or next active after dealer)
        if let Some(sb_pos) = self.players.iter().position(|p| p.is_small_blind && p.can_act()) {
            self.current_player = sb_pos;
        } else {
            // Find first active player after dealer
            let dealer_pos = self.players.iter().position(|p| p.is_dealer).unwrap_or(0);
            for i in 1..self.players.len() {
                let pos = (dealer_pos + i) % self.players.len();
                if self.players[pos].can_act() {
                    self.current_player = pos;
                    break;
                }
            }
        }
    }

    fn determine_winners(&mut self) {
        let active_players: Vec<(usize, Hand)> = self.players.iter()
            .enumerate()
            .filter(|(_, p)| p.status != crate::types::player::PlayerStatus::Folded)
            .filter_map(|(i, p)| {
                p.evaluate_hand(&self.community_cards).map(|hand| (i, hand))
            })
            .collect();

        if active_players.is_empty() {
            return;
        }

        // Find the best hand
        let best_hand = active_players.iter().max_by_key(|(_, hand)| hand).unwrap();
        
        // Find all players with the best hand (ties)
        let winners: Vec<usize> = active_players.iter()
            .filter(|(_, hand)| hand.score == best_hand.1.score)
            .map(|(i, _)| *i)
            .collect();

        // Distribute pot among winners
        let winnings_per_player = self.pot / winners.len() as u64;
        for &winner_idx in &winners {
            self.players[winner_idx].win_chips(winnings_per_player);
        }

        self.pot = 0;
    }

    pub fn get_active_player_count(&self) -> usize {
        self.players.iter().filter(|p| p.can_act()).count()
    }

    pub fn is_hand_complete(&self) -> bool {
        matches!(self.stage, GameStage::HandComplete) || 
        self.get_active_player_count() <= 1
    }
}
