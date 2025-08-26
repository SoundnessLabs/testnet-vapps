/// ## Example Usage
/// 
/// ```
/// use zkhealth_verifier::*;
/// 
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let service = VerificationService::new();
///     
///     // Create a vaccination record
///     let record_request = CreateRecordRequest {
///         patient_id: "patient123".to_string(),
///         provider_id: "hospital_abc".to_string(),
///         record_type: HealthRecordType::Vaccination {
///             vaccine_name: "COVID-19".to_string(),
///             doses_completed: 2,
///             required_doses: 2,
///         },
///         raw_data: b"vaccination_data".to_vec(),
///         provider_signature: b"signature".to_vec(),
///     };
///     
///     println!("zkHealth-Verifier is working!");
///     Ok(())
/// }
/// ```

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
