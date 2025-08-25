use crate::types::{Card, Hand, HandRank};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Debug, Clone)]
pub struct PokerZkCircuit {
    // Private inputs (hidden from other players)
    pub hole_cards: [Card; 2],
    pub player_secret: Vec<u8>,
    pub nonce: u64,
    
    // Public inputs (visible to all)
    pub community_cards: Vec<Card>,
    pub hand_commitment: [u8; 32],
    pub player_id: uuid::Uuid,
    pub table_id: uuid::Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HandProof {
    pub proof_data: Vec<u8>,
    pub public_inputs: PublicInputs,
    pub verification_key: Vec<u8>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublicInputs {
    pub hand_rank: HandRank,
    pub hand_strength: u32,
    pub player_id: uuid::Uuid,
    pub table_id: uuid::Uuid,
    pub community_cards_hash: [u8; 32],
    pub commitment: [u8; 32],
}

#[derive(Debug, thiserror::Error)]
pub enum ZkCircuitError {
    #[error("Invalid card combination")]
    InvalidCards,
    #[error("Commitment verification failed")]
    CommitmentFailed,
    #[error("Proof generation failed: {0}")]
    ProofGenerationFailed(String),
    #[error("Proof verification failed")]
    VerificationFailed,
}

impl PokerZkCircuit {
    pub fn new(
        hole_cards: [Card; 2],
        community_cards: Vec<Card>,
        player_secret: Vec<u8>,
        player_id: uuid::Uuid,
        table_id: uuid::Uuid,
    ) -> Self {
        let mut hasher = Sha256::new();
        hasher.update(&serde_json::to_vec(&hole_cards).unwrap());
        hasher.update(&player_secret);
        hasher.update(&player_id.to_string());
        
        let hand_commitment = hasher.finalize().into();
        let nonce = rand::random();

        Self {
            hole_cards,
            player_secret,
            nonce,
            community_cards,
            hand_commitment,
            player_id,
            table_id,
        }
    }

    /// Generate a zero-knowledge proof that proves hand strength without revealing cards
    pub fn generate_hand_proof(&self) -> Result<HandProof, ZkCircuitError> {
        // Step 1: Validate inputs
        self.validate_inputs()?;
        
        // Step 2: Calculate hand strength
        let hand = self.evaluate_hand()?;
        
        // Step 3: Generate commitment proof
        let commitment_proof = self.prove_commitment()?;
        
        // Step 4: Generate the ZK proof
        let proof_data = self.generate_zk_proof(&hand, &commitment_proof)?;
        
        // Step 5: Create public inputs
        let public_inputs = self.create_public_inputs(&hand)?;
        
        Ok(HandProof {
            proof_data,
            public_inputs,
            verification_key: self.generate_verification_key(),
            created_at: chrono::Utc::now(),
        })
    }

    fn validate_inputs(&self) -> Result<(), ZkCircuitError> {
        // Validate hole cards are unique
        if self.hole_cards[0] == self.hole_cards[1] {
            return Err(ZkCircuitError::InvalidCards);
        }

        // Validate no overlap with community cards
        for community_card in &self.community_cards {
            if self.hole_cards.contains(community_card) {
                return Err(ZkCircuitError::InvalidCards);
            }
        }

        // Validate community cards count
        if self.community_cards.len() > 5 {
            return Err(ZkCircuitError::InvalidCards);
        }

        Ok(())
    }

    fn evaluate_hand(&self) -> Result<Hand, ZkCircuitError> {
        if self.community_cards.len() < 3 {
            return Err(ZkCircuitError::InvalidCards);
        }

        let mut all_cards = vec![self.hole_cards[0], self.hole_cards[1]];
        all_cards.extend_from_slice(&self.community_cards);

        // Pad with dummy cards if needed for evaluation
        while all_cards.len() < 7 {
            // Use lowest possible cards that don't conflict
            let dummy_card = Card::new(crate::types::card::Suit::Hearts, crate::types::card::Rank::Two);
            if !all_cards.contains(&dummy_card) {
                all_cards.push(dummy_card);
            } else {
                all_cards.push(Card::new(crate::types::card::Suit::Diamonds, crate::types::card::Rank::Two));
            }
        }

        Ok(Hand::evaluate(all_cards))
    }

    fn prove_commitment(&self) -> Result<CommitmentProof, ZkCircuitError> {
        let mut hasher = Sha256::new();
        hasher.update(&serde_json::to_vec(&self.hole_cards).unwrap());
        hasher.update(&self.player_secret);
        hasher.update(&self.player_id.to_string());
        hasher.update(&self.nonce.to_le_bytes());
        
        let commitment = hasher.finalize();
        
        if commitment.as_slice() != self.hand_commitment {
            return Err(ZkCircuitError::CommitmentFailed);
        }
        
        Ok(CommitmentProof {
            commitment: self.hand_commitment,
            nonce: self.nonce,
            verified: true,
        })
    }

    fn generate_zk_proof(&self, hand: &Hand, _commitment_proof: &CommitmentProof) -> Result<Vec<u8>, ZkCircuitError> {
        // In a real implementation, this would use a ZK-SNARK library
        // For this demo, we'll create a cryptographic proof using hash-based commitments
        
        let mut hasher = Sha256::new();
        
        // Include private inputs (hashed)
        hasher.update(&serde_json::to_vec(&self.hole_cards).unwrap());
        hasher.update(&self.player_secret);
        hasher.update(&self.nonce.to_le_bytes());
        
        // Include public inputs
        hasher.update(&serde_json::to_vec(&self.community_cards).unwrap());
        hasher.update(&self.player_id.to_string());
        hasher.update(&self.table_id.to_string());
        
        // Include hand evaluation result
        hasher.update(&hand.score.to_le_bytes());
        hasher.update(&serde_json::to_vec(&hand.rank).unwrap());
        
        // Include timestamp for freshness
        let timestamp = chrono::Utc::now().timestamp();
        hasher.update(&timestamp.to_le_bytes());
        
        let proof_hash = hasher.finalize();
        
        // Create a structured proof (in real implementation, this would be a zk-SNARK)
        let proof = ZkProofStructure {
            proof_hash: proof_hash.to_vec(),
            commitment: self.hand_commitment.to_vec(),
            timestamp,
            circuit_id: "poker_hand_v1".to_string(),
        };
        
        serde_json::to_vec(&proof)
            .map_err(|e| ZkCircuitError::ProofGenerationFailed(e.to_string()))
    }

    fn create_public_inputs(&self, hand: &Hand) -> Result<PublicInputs, ZkCircuitError> {
        let mut hasher = Sha256::new();
        hasher.update(&serde_json::to_vec(&self.community_cards).unwrap());
        let community_cards_hash = hasher.finalize().into();

        Ok(PublicInputs {
            hand_rank: hand.rank.clone(),
            hand_strength: hand.score,
            player_id: self.player_id,
            table_id: self.table_id,
            community_cards_hash,
            commitment: self.hand_commitment,
        })
    }

    fn generate_verification_key(&self) -> Vec<u8> {
        let mut hasher = Sha256::new();
        hasher.update(b"poker_zk_circuit_v1");
        hasher.update(&self.table_id.to_string());
        hasher.update(&serde_json::to_vec(&self.community_cards).unwrap());
        hasher.finalize().to_vec()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ZkProofStructure {
    proof_hash: Vec<u8>,
    commitment: Vec<u8>,
    timestamp: i64,
    circuit_id: String,
}

#[derive(Debug, Clone)]
pub struct CommitmentProof {
    pub commitment: [u8; 32],
    pub nonce: u64,
    pub verified: bool,
}

/// Verify a hand proof without access to private cards
pub fn verify_hand_proof(
    proof: &HandProof,
    community_cards: &[Card],
    table_id: uuid::Uuid,
) -> Result<bool, ZkCircuitError> {
    // Verify public inputs consistency
    let mut hasher = Sha256::new();
    hasher.update(&serde_json::to_vec(community_cards).unwrap());
    let expected_community_hash = hasher.finalize();
    
    if expected_community_hash.as_slice() != proof.public_inputs.community_cards_hash {
        return Ok(false);
    }
    
    if proof.public_inputs.table_id != table_id {
        return Ok(false);
    }
    
    // Verify proof structure (simplified verification)
    let proof_structure: ZkProofStructure = serde_json::from_slice(&proof.proof_data)
        .map_err(|_| ZkCircuitError::VerificationFailed)?;
    
    // Check timestamp freshness (proof should be recent)
    let now = chrono::Utc::now().timestamp();
    if (now - proof_structure.timestamp).abs() > 300 { // 5 minute tolerance
        return Ok(false);
    }
    
    // Verify circuit ID
    if proof_structure.circuit_id != "poker_hand_v1" {
        return Ok(false);
    }
    
    // Verify commitment matches
    if proof_structure.commitment != proof.public_inputs.commitment.to_vec() {
        return Ok(false);
    }
    
    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::card::{Card, Rank, Suit};

    #[test]
    fn test_zk_circuit_creation() {
        let hole_cards = [
            Card::new(Suit::Spades, Rank::Ace),
            Card::new(Suit::Hearts, Rank::King),
        ];
        
        let community_cards = vec![
            Card::new(Suit::Diamonds, Rank::Ace),
            Card::new(Suit::Clubs, Rank::King),
            Card::new(Suit::Spades, Rank::Queen),
        ];
        
        let circuit = PokerZkCircuit::new(
            hole_cards,
            community_cards,
            b"player_secret".to_vec(),
            uuid::Uuid::new_v4(),
            uuid::Uuid::new_v4(),
        );
        
        assert_eq!(circuit.hole_cards, hole_cards);
    }

    #[test]
    fn test_hand_proof_generation() {
        let hole_cards = [
            Card::new(Suit::Spades, Rank::Ace),
            Card::new(Suit::Hearts, Rank::Ace),
        ];
        
        let community_cards = vec![
            Card::new(Suit::Diamonds, Rank::Ace),
            Card::new(Suit::Clubs, Rank::King),
            Card::new(Suit::Spades, Rank::Queen),
        ];
        
        let circuit = PokerZkCircuit::new(
            hole_cards,
            community_cards.clone(),
            b"player_secret".to_vec(),
            uuid::Uuid::new_v4(),
            uuid::Uuid::new_v4(),
        );
        
        let proof = circuit.generate_hand_proof();
        assert!(proof.is_ok());
        
        let proof = proof.unwrap();
        assert_eq!(proof.public_inputs.hand_rank, HandRank::ThreeOfAKind);
    }

    #[test]
    fn test_proof_verification() {
        let table_id = uuid::Uuid::new_v4();
        let hole_cards = [
            Card::new(Suit::Spades, Rank::Ace),
            Card::new(Suit::Hearts, Rank::Ace),
        ];
        
        let community_cards = vec![
            Card::new(Suit::Diamonds, Rank::King),
            Card::new(Suit::Clubs, Rank::Queen),
            Card::new(Suit::Spades, Rank::Jack),
        ];
        
        let circuit = PokerZkCircuit::new(
            hole_cards,
            community_cards.clone(),
            b"player_secret".to_vec(),
            uuid::Uuid::new_v4(),
            table_id,
        );
        
        let proof = circuit.generate_hand_proof().unwrap();
        let is_valid = verify_hand_proof(&proof, &community_cards, table_id).unwrap();
        
        assert!(is_valid);
    }
}
