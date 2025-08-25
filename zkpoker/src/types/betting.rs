use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BettingRound {
    pub round_number: u32,
    pub total_pot: u64,
    pub current_bet: u64,
}

impl BettingRound {
    pub fn new() -> Self {
        Self {
            round_number: 1,
            total_pot: 0,
            current_bet: 0,
        }
    }
}
