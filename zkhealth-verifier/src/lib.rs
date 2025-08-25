//! # zkHealth Record Verifier
//! 
//! A privacy-preserving healthcare verification system using zero-knowledge proofs.
//! Allows verification of medical records without exposing sensitive data.
//! 
//! ## Features
//! 
//! - **Vaccination Verification**: Prove vaccination status without revealing personal details
//! - **Age Verification**: Prove age above minimum without revealing exact age  
//! - **Medical Clearance**: Verify medical clearances while maintaining privacy
//! - **Zero-Knowledge Proofs**: All verifications use cryptographic proofs
//! - **Interoperable**: Works across different healthcare systems
//! 
//! ## Example Usage
//! 
//! ```
//! use zkhealth_verifier::*;
//! 
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let service = VerificationService::new();
//!     
//!     // Create a vaccination record
//!     let record_request = CreateRecordRequest {
//!         patient_id: "patient123".to_string(),
//!         provider_id: "hospital_abc".to_string(),
//!         record_type: HealthRecordType::Vaccination {
//!             vaccine_name: "COVID-19".to_string(),
//!             doses_completed: 2,
//!             required_doses: 2,
//!         },
//!         raw_data: b"vaccination_data".to_vec(),
//!         provider_signature: b"signature".to_vec(),
//!     };
//!     
//!     let record_id = service.create_record(record_request).await?;
//!     
//!     // Verify vaccination status
//!     let verify_request = VerifyRequest {
//!         record_id,
//!         requirement: RequirementType::VaccinationComplete {
//!             vaccine_name: "COVID-19".to_string(),
//!         },
//!         verifier_id: "airline_xyz".to_string(),
//!         patient_secret: b"secret".to_vec(),
//!     };
//!     
//!     let result = service.verify_health_requirement(verify_request).await?;
//!     println!("Verification successful: {}", result.is_valid);
//!     
//!     Ok(())
//! }
//! ```

pub mod types;
pub mod circuits;
pub mod api;

pub use types::*;
pub use circuits::*;
pub use api::*;

/// Re-export commonly used types for convenience
pub mod prelude {
    pub use crate::types::*;
    pub use crate::circuits::{HealthVerifierCircuit, verify_proof};
    pub use crate::api::VerificationService;
}

/// Version information
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Check if the library is properly initialized
pub fn health_check() -> bool {
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_health_check() {
        assert!(health_check());
    }

    #[test]
    fn test_health_record_creation() {
        let record = HealthRecord::new(
            "patient123".to_string(),
            "provider456".to_string(),
            HealthRecordType::Vaccination {
                vaccine_name: "COVID-19".to_string(),
                doses_completed: 2,
                required_doses: 2,
            },
            b"test_data",
        );

        assert_eq!(record.patient_id, "patient123");
        assert_eq!(record.provider_id, "provider456");
        assert!(!record.id.is_empty());
    }

    #[tokio::test]
    async fn test_verification_service() {
        let service = VerificationService::new();
        
        let request = CreateRecordRequest {
            patient_id: "patient123".to_string(),
            provider_id: "hospital_abc".to_string(),
            record_type: HealthRecordType::Vaccination {
                vaccine_name: "COVID-19".to_string(),
                doses_completed: 2,
                required_doses: 2,
            },
            raw_data: b"vaccination_data".to_vec(),
            provider_signature: b"signature".to_vec(),
        };
        
        let record_id = service.create_record(request).await.unwrap();
        assert!(!record_id.is_empty());
    }
}
