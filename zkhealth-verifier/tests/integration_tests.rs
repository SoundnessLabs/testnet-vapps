use zkhealth_verifier::{
    CreateRecordRequest, 
    VerifyRequest, 
    RequirementType,
    HealthRecordType,
    VerificationService
};
use chrono::Utc;

#[tokio::test]
async fn test_complete_vaccination_workflow() {
    let service = VerificationService::new();

    // Create vaccination record
    let record_request = CreateRecordRequest {
        patient_id: "test_patient".to_string(),
        provider_id: "test_hospital".to_string(),
        record_type: HealthRecordType::Vaccination {
            vaccine_name: "COVID-19".to_string(),
            doses_completed: 2,
            required_doses: 2,
        },
        raw_data: b"test_vaccination_data".to_vec(),
        provider_signature: b"test_signature".to_vec(),
    };

    let record_id = service.create_record(record_request).await.unwrap();

    // Verify vaccination
    let verify_request = VerifyRequest {
        record_id,
        requirement: RequirementType::VaccinationComplete {
            vaccine_name: "COVID-19".to_string(),
        },
        verifier_id: "test_verifier".to_string(),
        patient_secret: b"test_secret".to_vec(),
    };

    let result = service.verify_health_requirement(verify_request).await.unwrap();
    assert!(result.is_valid);
    assert!(result.proof.is_some());
}

#[tokio::test]
async fn test_age_verification_workflow() {
    let service = VerificationService::new();

    // Create age verification record
    let record_request = CreateRecordRequest {
        patient_id: "test_patient".to_string(),
        provider_id: "test_dmv".to_string(),
        record_type: HealthRecordType::AgeVerification {
            minimum_age: 25,
            verified_above_minimum: true,
        },
        raw_data: b"test_birth_certificate".to_vec(),
        provider_signature: b"test_signature".to_vec(),
    };

    let record_id = service.create_record(record_request).await.unwrap();

    // Verify age above 18
    let verify_request = VerifyRequest {
        record_id: record_id.clone(),
        requirement: RequirementType::AgeAbove(18),
        verifier_id: "test_verifier".to_string(),
        patient_secret: b"test_secret".to_vec(),
    };

    let result = service.verify_health_requirement(verify_request).await.unwrap();
    assert!(result.is_valid);

    // Verify age above 30 (should fail)
    let verify_request_fail = VerifyRequest {
        record_id,
        requirement: RequirementType::AgeAbove(30),
        verifier_id: "test_verifier".to_string(),
        patient_secret: b"test_secret".to_vec(),
    };

    let result_fail = service.verify_health_requirement(verify_request_fail).await.unwrap();
    assert!(!result_fail.is_valid);
}

#[tokio::test]
async fn test_medical_clearance_workflow() {
    let service = VerificationService::new();

    // Create medical clearance record
    let future_date = Utc::now() + chrono::Duration::days(30);
    let record_request = CreateRecordRequest {
        patient_id: "test_patient".to_string(),
        provider_id: "test_clinic".to_string(),
        record_type: HealthRecordType::MedicalClearance {
            clearance_type: "Travel Clearance".to_string(),
            valid_until: future_date,
        },
        raw_data: b"test_clearance_data".to_vec(),
        provider_signature: b"test_signature".to_vec(),
    };

    let record_id = service.create_record(record_request).await.unwrap();

    // Verify medical clearance
    let verify_request = VerifyRequest {
        record_id,
        requirement: RequirementType::MedicalClearance {
            clearance_type: "Travel Clearance".to_string(),
        },
        verifier_id: "test_verifier".to_string(),
        patient_secret: b"test_secret".to_vec(),
    };

    let result = service.verify_health_requirement(verify_request).await.unwrap();
    assert!(result.is_valid);
}

#[test]
fn test_health_record_validation() {
    use zkhealth_verifier::{HealthRecord, HealthRecordType};
    
    let record = HealthRecord::new(
        "patient123".to_string(),
        "provider456".to_string(),
        HealthRecordType::Vaccination {
            vaccine_name: "Test Vaccine".to_string(),
            doses_completed: 2,
            required_doses: 2,
        },
        b"test_data",
    );

    // Record should be invalid without signature
    assert!(!record.is_valid());

    // Record should be valid with signature
    let mut record_with_sig = record;
    record_with_sig.provider_signature = b"test_signature".to_vec();
    assert!(record_with_sig.is_valid());
}
