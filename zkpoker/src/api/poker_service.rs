use crate::game::PokerEngine;
use crate::types::ActionType;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct PokerService {
    engine: Arc<RwLock<PokerEngine>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTableRequest {
    pub small_blind: u64,
    pub big_blind: u64,
    pub max_players: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JoinTableRequest {
    pub player_name: String,
    pub buy_in: u64,
    pub table_id: Option<Uuid>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ActionRequest {
    pub player_id: Uuid,
    pub game_id: Uuid,
    pub action: String, // "fold", "check", "call", "bet:100", "raise:200", "allin"
    pub player_secret: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProofRequest {
    pub player_id: Uuid,
    pub game_id: Uuid,
    pub player_secret: String,
}

impl PokerService {
    pub fn new() -> Self {
        Self {
            engine: Arc::new(RwLock::new(PokerEngine::new())),
        }
    }

    pub async fn create_table(&self, request: CreateTableRequest) -> Result<Uuid, String> {
        let mut engine = self.engine.write().await;
        let table_id = engine.create_game(request.small_blind, request.big_blind);
        Ok(table_id)
    }

    pub async fn join_table(&self, request: JoinTableRequest) -> Result<(Uuid, Uuid), String> {
        let mut engine = self.engine.write().await;
        let join_request = crate::game::JoinGameRequest {
            player_name: request.player_name,
            buy_in: request.buy_in,
            table_id: request.table_id,
        };
        engine.join_game(join_request)
    }

    pub async fn player_action(&self, request: ActionRequest) -> Result<(), String> {
        let action = self.parse_action(&request.action)?;
        
        // Generate ZK proof if needed for betting actions
        let zk_proof = if matches!(action, ActionType::Bet(_) | ActionType::Raise(_)) {
            if let Some(secret) = request.player_secret {
                let engine = self.engine.read().await;
                Some(engine.generate_hand_proof(
                    request.player_id,
                    request.game_id,
                    secret.into_bytes(),
                )?)
            } else {
                None
            }
        } else {
            None
        };

        let mut engine = self.engine.write().await;
        let action_request = crate::game::PlayerActionRequest {
            player_id: request.player_id,
            game_id: request.game_id,
            action,
            zk_proof,
        };
        
        engine.process_action(action_request)
    }

    pub async fn get_game_state(&self, game_id: Uuid, player_id: Uuid) -> Result<crate::game::GameStateResponse, String> {
        let engine = self.engine.read().await;
        engine.get_game_state(game_id, player_id)
    }

    pub async fn generate_proof(&self, request: ProofRequest) -> Result<crate::crypto::HandProof, String> {
        let engine = self.engine.read().await;
        engine.generate_hand_proof(
            request.player_id,
            request.game_id,
            request.player_secret.into_bytes(),
        )
    }

    pub async fn get_leaderboard(&self) -> Vec<(String, u64)> {
        let engine = self.engine.read().await;
        engine.get_leaderboard()
    }

    pub async fn get_active_tables(&self) -> Vec<TableInfo> {
        let engine = self.engine.read().await;
        engine.games.iter().map(|(id, game)| {
            TableInfo {
                id: *id,
                players_count: game.players.len(),
                pot: game.pot,
                stage: game.stage.clone(),
                small_blind: game.small_blind,
                big_blind: game.big_blind,
            }
        }).collect()
    }

    fn parse_action(&self, action_str: &str) -> Result<ActionType, String> {
        match action_str.to_lowercase().as_str() {
            "fold" => Ok(ActionType::Fold),
            "check" => Ok(ActionType::Check),
            "call" => Ok(ActionType::Call),
            "allin" => Ok(ActionType::AllIn),
            _ => {
                if action_str.starts_with("bet:") {
                    let amount_str = action_str.strip_prefix("bet:").unwrap();
                    let amount = amount_str.parse::<u64>()
                        .map_err(|_| "Invalid bet amount".to_string())?;
                    Ok(ActionType::Bet(amount))
                } else if action_str.starts_with("raise:") {
                    let amount_str = action_str.strip_prefix("raise:").unwrap();
                    let amount = amount_str.parse::<u64>()
                        .map_err(|_| "Invalid raise amount".to_string())?;
                    Ok(ActionType::Raise(amount))
                } else {
                    Err(format!("Unknown action: {}", action_str))
                }
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TableInfo {
    pub id: Uuid,
    pub players_count: usize,
    pub pot: u64,
    pub stage: crate::types::GameStage,
    pub small_blind: u64,
    pub big_blind: u64,
}

impl Default for PokerService {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_poker_service_creation() {
        let service = PokerService::new();
        let tables = service.get_active_tables().await;
        assert!(tables.is_empty());
    }

    #[tokio::test]
    async fn test_create_and_join_table() {
        let service = PokerService::new();
        
        let create_request = CreateTableRequest {
            small_blind: 10,
            big_blind: 20,
            max_players: 6,
        };
        
        let table_id = service.create_table(create_request).await.unwrap();
        
        let join_request = JoinTableRequest {
            player_name: "TestPlayer".to_string(),
            buy_in: 1000,
            table_id: Some(table_id),
        };
        
        let result = service.join_table(join_request).await;
        assert!(result.is_ok());
    }

    #[tokio::test]  
    async fn test_action_parsing() {
        let service = PokerService::new();
        
        assert_eq!(service.parse_action("fold").unwrap(), ActionType::Fold);
        assert_eq!(service.parse_action("check").unwrap(), ActionType::Check);
        assert_eq!(service.parse_action("call").unwrap(), ActionType::Call);
        assert_eq!(service.parse_action("bet:100").unwrap(), ActionType::Bet(100));
        assert_eq!(service.parse_action("raise:200").unwrap(), ActionType::Raise(200));
        assert_eq!(service.parse_action("allin").unwrap(), ActionType::AllIn);
    }
}
