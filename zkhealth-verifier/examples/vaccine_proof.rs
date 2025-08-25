use zkhealth_verifier::{
    CreateRecordRequest, 
    VerifyRequest, 
    RequirementType,
    HealthRecordType,
    VerificationService
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("🏥 zkHealth Record Verifier - Vaccine Proof Example");
    println!("=================================================");

    // Initialize the verification service
    let service = VerificationService::new();

    // Step 1: Hospital creates a vaccination record
    println!("\n1. Creating vaccination record...");
    let vaccination_record = CreateRecordRequest {
        patient_id: "john_doe_123".to_string(),
        provider_id: "city_hospital".to_string(),
        record_type: HealthRecordType::Vaccination {
            vaccine_name: "COVID-19 mRNA".to_string(),
            doses_completed: 2,
            required_doses: 2,
        },
        raw_data: b"vaccination_data".to_vec(),
        provider_signature: b"hospital_signature".to_vec(),
    };

    let record_id = service.create_record(vaccination_record).await?;
    println!("✅ Vaccination record created with ID: {}", record_id);

    // Step 2: Patient requests verification for airline travel
    println!("\n2. Patient requesting vaccination verification for airline...");
    let verify_request = VerifyRequest {
        record_id: record_id.clone(),
        requirement: RequirementType::VaccinationComplete {
            vaccine_name: "COVID-19 mRNA".to_string(),
        },
        verifier_id: "global_airlines".to_string(),
        patient_secret: b"john_doe_secret_key_456".to_vec(),
    };

    let verification_result = service.verify_health_requirement(verify_request).await?;
    
    if verification_result.is_valid {
        println!("✅ Vaccination verification successful!");
        println!("   Verification ID: {}", verification_result.verification_id);
    } else {
        println!("❌ Vaccination verification failed");
        if let Some(error) = verification_result.error {
            println!("   Error: {}", error);
        }
    }

    println!("\n🎉 zkHealth demonstration completed!");
    println!("🔐 Privacy preserved - no personal medical data exposed!");

    Ok(())
}
