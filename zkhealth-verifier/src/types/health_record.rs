use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct HealthRecord {
    pub id: String,
    pub patient_id: String,
    pub provider_id: String,
    pub record_type: HealthRecordType,
    pub data_hash: [u8; 32],
    pub provider_signature: Vec<u8>,
    pub timestamp: DateTime<Utc>,
    pub expiry_date: Option<DateTime<Utc>>,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum HealthRecordType {
    Vaccination {
        vaccine_name: String,
        doses_completed: u32,
        required_doses: u32,
    },
    MedicalClearance {
        clearance_type: String,
        valid_until: DateTime<Utc>,
    },
    AgeVerification {
        minimum_age: u8,
        verified_above_minimum: bool,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationRequest {
    pub id: String,
    pub verifier_id: String,
    pub requirement_type: RequirementType,
    pub requested_at: DateTime<Utc>,
    pub valid_until: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RequirementType {
    VaccinationComplete { vaccine_name: String },
    AgeAbove(u8),
    MedicalClearance { clearance_type: String },
    HealthStatus { status_type: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZkProof {
    pub proof_data: Vec<u8>,
    pub public_inputs: Vec<u8>,
    pub verification_key: Vec<u8>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationResult {
    pub request_id: String,
    pub is_valid: bool,
    pub proof: ZkProof,
    pub verified_at: DateTime<Utc>,
    pub verifier_signature: Vec<u8>,
}

impl HealthRecord {
    pub fn new(
        patient_id: String,
        provider_id: String,
        record_type: HealthRecordType,
        data: &[u8],
    ) -> Self {
        use sha2::{Sha256, Digest};
        
        let mut hasher = Sha256::new();
        hasher.update(data);
        let data_hash: [u8; 32] = hasher.finalize().into();

        Self {
            id: uuid::Uuid::new_v4().to_string(),
            patient_id,
            provider_id,
            record_type,
            data_hash,
            provider_signature: Vec::new(), // Will be filled by provider
            timestamp: Utc::now(),
            expiry_date: None,
            metadata: HashMap::new(),
        }
    }

    pub fn is_valid(&self) -> bool {
        // Check if record is not expired
        if let Some(expiry) = self.expiry_date {
            if Utc::now() > expiry {
                return false;
            }
        }

        // Check signature (placeholder logic)
        !self.provider_signature.is_empty()
    }
}
