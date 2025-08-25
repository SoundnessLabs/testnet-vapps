//! # zkPoker - Verifiable Card Gaming Platform
//! 
//! A privacy-preserving poker platform built on Soundness Layer that enables
//! verifiable gameplay using zero-knowledge proofs. Players can prove hand
//! strength without revealing their cards.
//! 
//! ## Features
//! 
//! - **Private Card Management**: Cards are kept secret using cryptographic commitments
//! - **Zero-Knowledge Proofs**: Prove hand strength without revealing cards
//! - **Verifiable Shuffling**: Provably fair card distribution
//! - **Anti-Cheating**: Cryptographic verification prevents manipulation
//! - **Tournament Support**: Multi-table tournament management
//! - **Real-time Gaming**: Fast-paced poker with instant verification
//! 
//! ## Quick Start
//! 
//! ```
//! use zkpoker::prelude::*;
//! 
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Create poker service
//!     let service = PokerService::new();
//!     
//!     // Create a table
//!     let table_request = CreateTableRequest {
//!         small_blind: 10,
//!         big_blind: 20,
//!         max_players: 6,
//!     };
//!     let table_id = service.create_table(table_request).await?;
//!     
//!     // Join the table
//!     let join_request = JoinTableRequest {
//!         player_name: "Player1".to_string(),
//!         buy_in: 1000,
//!         table_id: Some(table_id),
//!     };  
//!     let (player_id, _) = service.join_table(join_request).await?;
//!     
//!     // Get game state
//!     let state = service.get_game_state(table_id, player_id).await?;
//!     println!("Game stage: {:?}", state.stage);
//!     
//!     Ok(())
//! }
//! ```

pub mod types;
pub mod game;
pub mod crypto;
pub mod api;

// Re-export commonly used types
pub use types::*;
pub use game::*;
pub use crypto::*;
pub use api::*;

/// Convenient imports for users of this library
pub mod prelude {
    pub use crate::types::*;
    pub use crate::game::{PokerEngine, JoinGameRequest, PlayerActionRequest, GameStateResponse};
    pub use crate::crypto::{PokerZkCircuit, HandProof, verify_hand_proof};
    pub use crate::api::{PokerService, CreateTableRequest, JoinTableRequest, ActionRequest, ProofRequest};
}

/// Library version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Health check function
pub fn health_check() -> bool {
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_library_health() {
        assert!(health_check());
    }

    #[test]
    fn test_version() {
        assert!(!VERSION.is_empty());
    }
}
