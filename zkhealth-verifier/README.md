# 🩺 zkHealth Record Verifier  

[![Rust](https://img.shields.io/badge/Rust-1.70+-orange?logo=rust)](https://www.rust-lang.org)  
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue)](./LICENSE)  
[![Build Status](https://img.shields.io/badge/build-passing-brightgreen)]()  
[![Privacy-Preserving](https://img.shields.io/badge/Privacy-Preserving-success)]()  

> **A privacy-preserving healthcare verification system built on [Soundness Layer](https://github.com/SoundnessLabs/soundness-layer).**  
> Enables trustless verification of medical records without exposing sensitive data using **zero-knowledge proofs**.

---

## 🎯 Problem Statement  

Traditional healthcare verification systems face serious challenges:  
- 🔓 Full disclosure of medical records required  
- 🗄️ Centralized databases → single points of failure  
- 🔀 Poor interoperability across systems  
- 👤 Patient privacy compromised  
- 🐢 Manual & slow verification processes  

---

## 💡 Solution  

`zkHealth Record Verifier` leverages **Zero-Knowledge Proofs (ZKP)** for privacy-first healthcare validation:

- ✅ **Vaccination status** proven without revealing personal info  
- ✅ **Age requirements** verified without exact birthdate disclosure  
- ✅ **Medical clearance** confirmed without exposing health history  
- ✅ **Trustless cryptographic verification**  
- ✅ **Cross-system interoperability**  

---

## 🏗️ Architecture  

```text
Patient Health Record (Private)
          │
          ▼
   ZK Circuit Generation
          │
          ▼
   Cryptographic Proof (Public)
          │
          ▼
Verification (No Private Data Exposed)
```

---

## 🔎 Core Components  

1. **Health Record Types**  
   - Vaccination records  
   - Age verification  
   - Medical clearances  
   - Custom attestations  

2. **Zero-Knowledge Circuits**  
   - Proof generation (privacy-preserving)  
   - Cryptographic verification  
   - Requirement matching  

3. **Verification API**  
   - Record creation & management  
   - Proof generation  
   - Requirement validation  

---

## 🚀 Use Cases  

### ✈️ Travel & Transportation  
- Airlines: verify requirements without disclosing medical history  
- Border Control: confirm vaccination privately  
- Public Transport: age-based discounts  

### 🏢 Employment & Education  
- Workplace: medical clearances  
- Schools: vaccination requirements  
- Healthcare facilities: staff verification  

### 🎉 Events & Entertainment  
- Concerts & sports: health/age verification  
- Clubs & bars: age checks without revealing DOB  
- Conferences: health status confirmation  

### 💳 Insurance & Finance  
- Health insurance eligibility  
- Life insurance verification  
- Disability service confirmation  

---

## 📋 Technical Specs  

### Health Record Types (Rust Enum)

```rust
pub enum HealthRecordType {
    Vaccination { vaccine_name: String, doses_completed: u32, required_doses: u32 },
    AgeVerification { minimum_age: u8, verified_above_minimum: bool },
    MedicalClearance { clearance_type: String, valid_until: DateTime<Utc> },
}
```

### Verification Requirements  

```rust
pub enum RequirementType {
    VaccinationComplete { vaccine_name: String },
    AgeAbove(u8),
    MedicalClearance { clearance_type: String },
    HealthStatus { status_type: String },
}
```

---

## 🔧 Installation & Setup  

### Prerequisites  
- 🦀 Rust 1.70+  
- 📦 Cargo  
- 🔗 Git  

```bash
# Clone repository
git clone https://github.com/SoundnessLabs/testnet-vapps.git
cd testnet-vapps/zkhealth-verifier

# Build
cargo build --release

# Run tests
cargo test

# Run example
cargo run --example vaccine_proof
```

---

## 📖 Usage Examples  

### ✅ Vaccination Proof  

```rust
let record_request = CreateRecordRequest {
    patient_id: "patient_123".to_string(),
    provider_id: "city_hospital".to_string(),
    record_type: HealthRecordType::Vaccination {
        vaccine_name: "COVID-19".to_string(),
        doses_completed: 2,
        required_doses: 2,
    },
    raw_data: b"vaccination_data".to_vec(),
    provider_signature: b"hospital_signature".to_vec(),
};

let record_id = service.create_record(record_request).await?;
```

Then verify requirement:  

```rust
let verify_request = VerifyRequest {
    record_id,
    requirement: RequirementType::VaccinationComplete { vaccine_name: "COVID-19".to_string() },
    verifier_id: "airline_company".to_string(),
    patient_secret: b"patient_secret_key".to_vec(),
};
```

👉 If valid: `✅ Vaccination verified successfully!`

---

### 🎂 Age Verification Example  

```rust
// Verify age above 18 without revealing exact age
let age_record = CreateRecordRequest {
    patient_id: "user_456".to_string(),
    provider_id: "dmv_office".to_string(),
    record_type: HealthRecordType::AgeVerification {
        minimum_age: 25, // user is 25+
        verified_above_minimum: true,
    },
    raw_data: b"birth_certificate_hash".to_vec(),
    provider_signature: b"dmv_signature".to_vec(),
};

// Requirement: age above 21 (for bar entry)
let age_requirement = RequirementType::AgeAbove(21);

// Verification will succeed without revealing exact age
```

---

## 🧪 Testing  

```bash
cargo test --lib          # Unit tests
cargo test --test ...     # Integration tests
cargo test --examples     # Example tests
cargo test -- --nocapture # Verbose
```

---

## 🔐 Privacy Guarantees  

What is **never revealed**:  
❌ Exact age / birthdate  
❌ Personal identifiers  
❌ Medical history & diagnoses  
❌ Provider internal data  
❌ Contact information  

What is **proven cryptographically**:  
✅ Requirement satisfaction (yes/no)  
✅ Record authenticity  
✅ Provider signature validity  
✅ Verification timestamp  

---

## 🛣️ Roadmap  

- **Phase 1**: Core implementation ✅  
- **Phase 2**: Multi-sig, batch verification, mobile SDK  
- **Phase 3**: Security audit, HIPAA/GDPR compliance  
- **Phase 4**: Ecosystem integrations (EHR, Gov IDs, Insurance)  

---

## 🤝 Contributing  

1. Fork & branch → `git checkout -b feature/new-feature`  
2. Add changes + tests  
3. Run `cargo test`  
4. Submit PR 🎉  

📌 Guidelines: follow Rust idioms, maintain privacy guarantees, update docs.

---

## 📄 License  

Licensed under **MIT OR Apache-2.0**.  

---

## 🔗 Links  

- 🌐 [Soundness Labs](https://github.com/SoundnessLabs)  
- 🔒 [Soundness Layer](https://github.com/SoundnessLabs/soundness-layer)  
- 🧪 [Testnet vApps](https://github.com/SoundnessLabs/testnet-vapps)  

---

⚠️ **Disclaimer**: This is a proof-of-concept for the Soundness Layer testnet. Not production-ready without security audit and compliance review.  

---

✨ **Built with ❤️ for privacy-preserving healthcare on Soundness Layer**
