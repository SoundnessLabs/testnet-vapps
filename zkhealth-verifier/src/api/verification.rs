use crate::circuits::*;
use crate::types::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::sync::RwLock;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct VerificationService {
    // In-memory storage for demo purposes
    // In production, this would be a database
    records: Arc<RwLock<HashMap<String, HealthRecord>>>,
    verifications: Arc<RwLock<HashMap<String, VerificationResult>>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateRecordRequest {
    pub patient_id: String,
    pub provider_id: String,
    pub record_type: HealthRecordType,
    pub raw_data: Vec<u8>,
    pub provider_signature: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VerifyRequest {
    pub record_id: String,
    pub requirement: RequirementType,
    pub verifier_id: String,
    pub patient_secret: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VerifyResponse {
    pub verification_id: String,
    pub is_valid: bool,
    pub proof: Option<ZkProof>,
    pub error: Option<String>,
}

impl VerificationService {
    pub fn new() -> Self {
        Self {
            records: Arc::new(RwLock::new(HashMap::new())),
            verifications: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Store a health record
    pub async fn create_record(
        &self,
        request: CreateRecordRequest,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let mut record = HealthRecord::new(
            request.patient_id,
            request.provider_id,
            request.record_type,
            &request.raw_data,
        );
        
        record.provider_signature = request.provider_signature;
        
        let record_id = record.id.clone();
        
        let mut records = self.records.write().await;
        records.insert(record_id.clone(), record);
        
        Ok(record_id)
    }

    /// Generate verification proof for a health record
    pub async fn verify_health_requirement(
        &self,
        request: VerifyRequest,
    ) -> Result<VerifyResponse, Box<dyn std::error::Error + Send + Sync>> {
        // Retrieve the health record
        let records = self.records.read().await;
        let record = records.get(&request.record_id)
            .ok_or("Health record not found")?
            .clone();
        drop(records);

        // Create verification circuit
        let circuit = HealthVerifierCircuit::new(
            request.requirement.clone(),
            request.verifier_id.clone(),
            record,
            request.patient_secret,
        );

        // Generate proof
        match circuit.generate_proof() {
            Ok(proof) => {
                let verification_result = VerificationResult {
                    request_id: uuid::Uuid::new_v4().to_string(),
                    is_valid: true,
                    proof: proof.clone(),
                    verified_at: chrono::Utc::now(),
                    verifier_signature: Vec::new(), // Would be signed by verifier
                };

                let verification_id = verification_result.request_id.clone();
                
                let mut verifications = self.verifications.write().await;
                verifications.insert(verification_id.clone(), verification_result);

                Ok(VerifyResponse {
                    verification_id,
                    is_valid: true,
                    proof: Some(proof),
                    error: None,
                })
            },
            Err(e) => {
                Ok(VerifyResponse {
                    verification_id: uuid::Uuid::new_v4().to_string(),
                    is_valid: false,
                    proof: None,
                    error: Some(e.to_string()),
                })
            }
        }
    }

    /// Verify a submitted proof
    pub async fn validate_proof(
        &self,
        proof: &ZkProof,
        requirement: &RequirementType,
        verifier_id: &str,
    ) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
        let is_valid = verify_proof(proof, requirement, verifier_id)?;
        Ok(is_valid)
    }

    /// Get verification result by ID
    pub async fn get_verification(
        &self,
        verification_id: &str,
    ) -> Option<VerificationResult> {
        let verifications = self.verifications.read().await;
        verifications.get(verification_id).cloned()
    }

    /// List all records for a patient (for demo purposes)
    pub async fn get_patient_records(
        &self,
        patient_id: &str,
    ) -> Vec<HealthRecord> {
        let records = self.records.read().await;
        records.values()
            .filter(|record| record.patient_id == patient_id)
            .cloned()
            .collect()
    }
}

impl Default for VerificationService {
    fn default() -> Self {
        Self::new()
    }
}
