use zkhealth_verifier::prelude::*;
use chrono::Utc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
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
        raw_data: serde_json::to_vec(&serde_json::json!({
            "patient_name": "John Doe",
            "vaccine_batch": "ABC123XYZ",
            "vaccination_dates": ["2024-01-15", "2024-02-15"],
            "administering_nurse": "Nurse Smith"
        }))?,
        provider_signature: simulate_provider_signature("city_hospital", "john_doe_123"),
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
        
        // Step 3: Airline verifies the proof
        if let Some(proof) = verification_result.proof {
            println!("\n3. Airline verifying the proof...");
            let is_proof_valid = service.validate_proof(
                &proof,
                &RequirementType::VaccinationComplete {
                    vaccine_name: "COVID-19 mRNA".to_string(),
                },
                "global_airlines"
            ).await?;
            
            if is_proof_valid {
                println!("✅ Airline verified: Passenger has valid vaccination status");
                println!("   ✈️  Boarding pass approved!");
            } else {
                println!("❌ Proof verification failed");
            }
        }
    } else {
        println!("❌ Vaccination verification failed");
        if let Some(error) = verification_result.error {
            println!("   Error: {}", error);
        }
    }

    // Step 4: Demonstrate age verification
    println!("\n4. Creating age verification record...");
    let age_record = CreateRecordRequest {
        patient_id: "john_doe_123".to_string(),
        provider_id: "dmv_office".to_string(),
        record_type: HealthRecordType::AgeVerification {
            minimum_age: 21,
            verified_above_minimum: true,
        },
        raw_data: b"birth_certificate_data".to_vec(),
        provider_signature: simulate_provider_signature("dmv_office", "john_doe_123"),
    };

    let age_record_id = service.create_record(age_record).await?;
    println!("✅ Age verification record created");

    // Verify age for bar entry
    let age_verify_request = VerifyRequest {
        record_id: age_record_id,
        requirement: RequirementType::AgeAbove(18),
        verifier_id: "downtown_bar".to_string(),
        patient_secret: b"john_doe_secret_key_456".to_vec(),
    };

    let age_verification_result = service.verify_health_requirement(age_verify_request).await?;
    
    if age_verification_result.is_valid {
        println!("✅ Age verification successful for bar entry!");
        println!("   🍺 Access granted (age verified above 18 without revealing exact age)");
    } else {
        println!("❌ Age verification failed");
    }

    // Step 5: Show privacy preservation
    println!("\n5. Privacy Analysis:");
    println!("   �� Patient's exact age: NEVER REVEALED");
    println!("   🔒 Medical details: NEVER REVEALED"); 
    println!("   🔒 Personal identifiers: NEVER REVEALED");
    println!("   ✅ Only verification status: PROVEN CRYPTOGRAPHICALLY");

    Ok(())
}

// Helper function to simulate provider signatures
fn simulate_provider_signature(provider_id: &str, patient_id: &str) -> Vec<u8> {
    use sha2::{Sha256, Digest};
    
    let mut hasher = Sha256::new();
    hasher.update(provider_id.as_bytes());
    hasher.update(patient_id.as_bytes());
    hasher.update(b"provider_private_key_simulation");
    hasher.finalize().to_vec()
}
