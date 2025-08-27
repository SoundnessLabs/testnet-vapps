# ProofVault

## Category
identity

## Project Title
ProofVault: Decentralized Proof-of-Identity Storage

## Summary
ProofVault is a vApp designed to let users securely store and share verifiable proofs of their identity without exposing raw personal data.  
Instead of sending sensitive information directly, users publish zero-knowledge proofs via the Soundness Layer (SL). This ensures third parties can validate identity claims (e.g., "over 18", "citizen of X") without seeing the underlying documents.

## Architecture & SL Integration
- **Frontend**: simple dashboard for users to generate and manage proofs (age, nationality, membership, etc.).  
- **Proof Generator**: takes user claims → generates ZK proofs → sends them to Soundness Layer.  
- **Soundness Layer (SL)**: produces and stores proofs as blobs that can be independently verified.  
- **Verifier**: dApps or partners fetch proofs via blob ids to validate claims (without accessing raw data).  

## MVP Scope
- Single type of claim (e.g., “over 18”) for demo.  
- Integration with Soundness Layer for proof creation & blob publishing.  
- Basic verifier page to check blob id validity.  
- Demo frontend showing “share proof” flow.  

## Milestones & Timeline
- **Week 1–2**: Build proof schema + connect to SL for publishing blobs.  
- **Week 3–4**: Frontend demo (user claim → proof → verifier page).  
- **Week 5**: Optional — add multiple claim types (age + nationality).  

## Risks & Dependencies
- Proof generation time may cause latency in validation.  
- Need careful handling of privacy so no metadata leakage.  
- Dependence on SL blob availability.  

## Success Criteria
- Users can generate at least one claim-proof and share a blob id.  
- Verifier page validates ≥90% of submitted proofs.  
- Demo shows privacy-preserving validation flow without exposing personal data.  

## Contact
- **GitHub**: [williiii28](https://github.com/williiii28)  
- **Discord**: diabloo666
