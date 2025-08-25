use crate::types::card::Card;
use crate::types::hand::Hand;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    pub id: Uuid,
    pub name: String,
    pub chips: u64,
    pub hole_cards: Option<[Card; 2]>,
    pub current_bet: u64,
    pub total_bet_this_hand: u64,
    pub status: PlayerStatus,
    pub position: usize,
    pub is_dealer: bool,
    pub is_small_blind: bool,
    pub is_big_blind: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PlayerStatus {
    Active,
    Folded,
    AllIn,
    SitOut,
    Disconnected,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerAction {
    pub player_id: Uuid,
    pub action_type: ActionType,
    pub amount: u64,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ActionType {
    Fold,
    Check,
    Call,
    Bet(u64),
    Raise(u64),
    AllIn,
}

impl Player {
    pub fn new(id: Uuid, name: String, chips: u64) -> Self {
        Self {
            id,
            name,
            chips,
            hole_cards: None,
            current_bet: 0,
            total_bet_this_hand: 0,
            status: PlayerStatus::Active,
            position: 0,
            is_dealer: false,
            is_small_blind: false,
            is_big_blind: false,
        }
    }

    pub fn can_act(&self) -> bool {
        matches!(self.status, PlayerStatus::Active)
    }

    pub fn set_hole_cards(&mut self, cards: [Card; 2]) {
        self.hole_cards = Some(cards);
    }

    pub fn fold(&mut self) {
        self.status = PlayerStatus::Folded;
        self.current_bet = 0;
    }

    pub fn bet(&mut self, amount: u64) -> Result<(), String> {
        if amount > self.chips {
            return Err("Insufficient chips".to_string());
        }
        
        self.chips -= amount;
        self.current_bet = amount;
        self.total_bet_this_hand += amount;
        
        if self.chips == 0 {
            self.status = PlayerStatus::AllIn;
        }
        
        Ok(())
    }

    pub fn win_chips(&mut self, amount: u64) {
        self.chips += amount;
    }

    pub fn reset_for_new_hand(&mut self) {
        self.hole_cards = None;
        self.current_bet = 0;
        self.total_bet_this_hand = 0;
        if self.status != PlayerStatus::SitOut {
            self.status = PlayerStatus::Active;
        }
        self.is_dealer = false;
        self.is_small_blind = false;
        self.is_big_blind = false;
    }

    pub fn evaluate_hand(&self, community_cards: &[Card]) -> Option<Hand> {
        if let Some(hole_cards) = self.hole_cards {
            let mut all_cards = vec![hole_cards[0], hole_cards[1]];
            all_cards.extend_from_slice(community_cards);
            
            if all_cards.len() == 7 {
                Some(Hand::evaluate(all_cards))
            } else {
                None
            }
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_player_creation() {
        let player_id = Uuid::new_v4();
        let player = Player::new(player_id, "TestPlayer".to_string(), 1000);
        
        assert_eq!(player.id, player_id);
        assert_eq!(player.name, "TestPlayer");
        assert_eq!(player.chips, 1000);
        assert_eq!(player.status, PlayerStatus::Active);
    }

    #[test]
    fn test_player_betting() {
        let mut player = Player::new(Uuid::new_v4(), "TestPlayer".to_string(), 1000);
        
        assert!(player.bet(500).is_ok());
        assert_eq!(player.chips, 500);
        assert_eq!(player.current_bet, 500);
        assert_eq!(player.total_bet_this_hand, 500);
    }

    #[test]
    fn test_player_all_in() {
        let mut player = Player::new(Uuid::new_v4(), "TestPlayer".to_string(), 100);
        
        assert!(player.bet(100).is_ok());
        assert_eq!(player.chips, 0);
        assert_eq!(player.status, PlayerStatus::AllIn);
    }
}
