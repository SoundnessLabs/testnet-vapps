use crate::types::*;
use crate::crypto::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct PokerEngine {
    pub games: HashMap<Uuid, GameState>,
    pub player_sessions: HashMap<Uuid, Uuid>, // player_id -> game_id
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JoinGameRequest {
    pub player_name: String,
    pub buy_in: u64,
    pub table_id: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerActionRequest {
    pub player_id: Uuid,
    pub game_id: Uuid,
    pub action: ActionType,
    pub zk_proof: Option<HandProof>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameStateResponse {
    pub game_id: Uuid,
    pub stage: GameStage,
    pub pot: u64,
    pub community_cards: Vec<Card>,
    pub current_player: Option<Uuid>,
    pub players: Vec<PublicPlayerInfo>,
    pub your_cards: Option<[Card; 2]>,
    pub can_act: bool,
    pub valid_actions: Vec<ActionType>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublicPlayerInfo {
    pub id: Uuid,
    pub name: String,
    pub chips: u64,
    pub current_bet: u64,
    pub status: PlayerStatus,
    pub position: String,
}

impl PokerEngine {
    pub fn new() -> Self {
        Self {
            games: HashMap::new(),
            player_sessions: HashMap::new(),
        }
    }

    pub fn create_game(&mut self, small_blind: u64, big_blind: u64) -> Uuid {
        let game_id = Uuid::new_v4();
        let game_state = GameState::new(game_id, small_blind, big_blind);
        self.games.insert(game_id, game_state);
        game_id
    }

    pub fn join_game(&mut self, request: JoinGameRequest) -> Result<(Uuid, Uuid), String> {
        let game_id = if let Some(table_id) = request.table_id {
            table_id
        } else {
            // Find available game or create new one
            self.find_available_game().unwrap_or_else(|| self.create_game(10, 20))
        };

        let game = self.games.get_mut(&game_id)
            .ok_or("Game not found")?;

        let player_id = Uuid::new_v4();
        let player = Player::new(player_id, request.player_name, request.buy_in);
        
        game.add_player(player)?;
        self.player_sessions.insert(player_id, game_id);

        // Start hand if enough players
        if game.can_start_hand() && game.stage == GameStage::WaitingForPlayers {
            game.start_new_hand()?;
        }

        Ok((player_id, game_id))
    }

    pub fn process_action(&mut self, request: PlayerActionRequest) -> Result<(), String> {
        let game = self.games.get_mut(&request.game_id)
            .ok_or("Game not found")?;

        // Verify it's the player's turn
        let current_player_id = game.players.get(game.current_player)
            .map(|p| p.id)
            .ok_or("No current player")?;

        if current_player_id != request.player_id {
            return Err("Not your turn".to_string());
        }

        // For betting actions that reveal hand strength, verify ZK proof
        if matches!(request.action, ActionType::Bet(_) | ActionType::Raise(_)) {
            if let Some(proof) = request.zk_proof {
                let is_valid = verify_hand_proof(
                    &proof, 
                    &game.community_cards, 
                    game.game_id
                )?;
                
                if !is_valid {
                    return Err("Invalid ZK proof".to_string());
                }
            }
        }

        game.process_action(request.player_id, request.action)?;

        // Check if hand is complete
        if game.is_hand_complete() {
            // Start new hand if possible
            if game.can_start_hand() {
                game.start_new_hand()?;
            }
        }

        Ok(())
    }

    pub fn get_game_state(&self, game_id: Uuid, player_id: Uuid) -> Result<GameStateResponse, String> {
        let game = self.games.get(&game_id)
            .ok_or("Game not found")?;

        let player_idx = game.players.iter()
            .position(|p| p.id == player_id)
            .ok_or("Player not in game")?;

        let player = &game.players[player_idx];
        let current_player_id = game.players.get(game.current_player).map(|p| p.id);
        let can_act = current_player_id == Some(player_id) && player.can_act();

        let valid_actions = if can_act {
            self.get_valid_actions(game, player_idx)
        } else {
            Vec::new()
        };

        let public_players = game.players.iter().map(|p| {
            let position = if p.is_dealer {
                "Dealer".to_string()
            } else if p.is_small_blind {
                "Small Blind".to_string()
            } else if p.is_big_blind {
                "Big Blind".to_string()
            } else {
                format!("Position {}", p.position)
            };

            PublicPlayerInfo {
                id: p.id,
                name: p.name.clone(),
                chips: p.chips,
                current_bet: p.current_bet,
                status: p.status.clone(),
                position,
            }
        }).collect();

        Ok(GameStateResponse {
            game_id,
            stage: game.stage.clone(),
            pot: game.pot,
            community_cards: game.community_cards.clone(),
            current_player: current_player_id,
            players: public_players,
            your_cards: player.hole_cards,
            can_act,
            valid_actions,
        })
    }

    pub fn generate_hand_proof(&self, player_id: Uuid, game_id: Uuid, player_secret: Vec<u8>) -> Result<HandProof, String> {
        let game = self.games.get(&game_id)
            .ok_or("Game not found")?;

        let player = game.players.iter()
            .find(|p| p.id == player_id)
            .ok_or("Player not found")?;

        let hole_cards = player.hole_cards
            .ok_or("Player has no hole cards")?;

        if game.community_cards.len() < 3 {
            return Err("Not enough community cards for proof generation".to_string());
        }

        let circuit = PokerZkCircuit::new(
            hole_cards,
            game.community_cards.clone(),
            player_secret,
            player_id,
            game_id,
        );

        circuit.generate_hand_proof()
            .map_err(|e| format!("ZK proof generation failed: {}", e))
    }

    fn find_available_game(&self) -> Option<Uuid> {
        self.games.iter()
            .find(|(_, game)| {
                game.players.len() < 9 && 
                matches!(game.stage, GameStage::WaitingForPlayers)
            })
            .map(|(id, _)| *id)
    }

    fn get_valid_actions(&self, game: &GameState, player_idx: usize) -> Vec<ActionType> {
        let player = &game.players[player_idx];
        let mut actions = Vec::new();

        if !player.can_act() {
            return actions;
        }

        // Always can fold (except when all-in)
        if player.status != PlayerStatus::AllIn {
            actions.push(ActionType::Fold);
        }

        let call_amount = game.current_bet.saturating_sub(player.current_bet);

        // Check or call
        if call_amount == 0 {
            actions.push(ActionType::Check);
        } else if call_amount <= player.chips {
            actions.push(ActionType::Call);
        }

        // Betting/Raising
        if game.current_bet == 0 {
            // Can bet
            let min_bet = game.big_blind;
            if player.chips >= min_bet {
                actions.push(ActionType::Bet(min_bet));
            }
        } else {
            // Can raise
            let min_raise = game.current_bet * 2;
            if player.chips >= min_raise {
                actions.push(ActionType::Raise(min_raise));
            }
        }

        // All-in (always available if player has chips)
        if player.chips > 0 {
            actions.push(ActionType::AllIn);
        }

        actions
    }

    pub fn get_leaderboard(&self) -> Vec<(String, u64)> {
        let mut player_chips: HashMap<String, u64> = HashMap::new();
        
        for game in self.games.values() {
            for player in &game.players {
                *player_chips.entry(player.name.clone()).or_insert(0) += player.chips;
            }
        }
        
        let mut leaderboard: Vec<(String, u64)> = player_chips.into_iter().collect();
        leaderboard.sort_by(|a, b| b.1.cmp(&a.1));
        leaderboard
    }
}

impl Default for PokerEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_poker_engine_creation() {
        let engine = PokerEngine::new();
        assert!(engine.games.is_empty());
        assert!(engine.player_sessions.is_empty());
    }

    #[test]
    fn test_create_and_join_game() {
        let mut engine = PokerEngine::new();
        
        let join_request = JoinGameRequest {
            player_name: "TestPlayer".to_string(),
            buy_in: 1000,
            table_id: None,
        };
        
        let result = engine.join_game(join_request);
        assert!(result.is_ok());
        
        let (player_id, game_id) = result.unwrap();
        assert!(engine.games.contains_key(&game_id));
        assert_eq!(engine.player_sessions.get(&player_id), Some(&game_id));
    }

    #[test]
    fn test_game_state_retrieval() {
        let mut engine = PokerEngine::new();
        
        let join_request = JoinGameRequest {
            player_name: "TestPlayer".to_string(),
            buy_in: 1000,
            table_id: None,
        };
        
        let (player_id, game_id) = engine.join_game(join_request).unwrap();
        let game_state = engine.get_game_state(game_id, player_id);
        
        assert!(game_state.is_ok());
        let state = game_state.unwrap();
        assert_eq!(state.game_id, game_id);
        assert_eq!(state.players.len(), 1);
    }
}
