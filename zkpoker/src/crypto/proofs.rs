use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZkProof {
    pub proof_data: Vec<u8>,
    pub public_inputs: Vec<u8>,
}

impl ZkProof {
    pub fn new(proof_data: Vec<u8>, public_inputs: Vec<u8>) -> Self {
        Self {
            proof_data,
            public_inputs,
        }
    }
}
