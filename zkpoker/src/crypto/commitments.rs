use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Commitment {
    pub hash: [u8; 32],
    pub nonce: u64,
}

impl Commitment {
    pub fn new(data: &[u8], nonce: u64) -> Self {
        let mut hasher = Sha256::new();
        hasher.update(data);
        hasher.update(&nonce.to_le_bytes());
        let hash = hasher.finalize().into();
        
        Self { hash, nonce }
    }
}
