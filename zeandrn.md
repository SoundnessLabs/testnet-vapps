# vApp Submission: ZK-based KYC verification, integrated with Walrus and SL

## Verification
```yaml
github_username: "zean6178"
discord_id: "1035001460582273144"
timestamp: "2025-08-27"
```

## Developer
- **Name**: Zean Darren
- **GitHub**: @zean6178
- **Discord**: @zeandrn
- **Experience**: I am full time web3 moderator and ambassador, also i am beginner in Web3 programming. I am currently learning zero-knowledge proofs and the Soundness stack.

## Project

### Name & Category
- **Project**: ZK-based KYC verification, integrated with Walrus and SL
- **Category**: identity

### Description
ZK-based KYC verification integrated with Walrus decentralized programmable storage in the Sui ecosystem and Soundness Layer decentralized verification layer for ZK proof verification.
- **KYC Issuer (Trusted KYC Provider)**: Perform off-chain document/biometric checks or Issues credential (e.g. “KYC Passed”, “≥18”, “country domicile X”) + commitment hash of underlying data (without disclosing PII). or issuing soulbound tokens/attestations that reference commitments. (An example of a modern zkKYC approach uses a zkVM like RISC Zero to prove verification without exposing data.
- **ZKProof System**: SNARK (Groth16/PLONK) or zkVM (RISC Zero / SP1)
- **Walrus (Storage/DA)**: Store encrypted KYC artifacts (e.g., signed credentials, CRLs/revocation lists, audit trails) as blobs; metadata and proof-of-availability are recorded in Sui. Walrus facilitates programmable storage, censorship resistance, and data availability certification.
- **Soundness Layer (ZK Verification Layer)**: Soundness Layer validators fetch blobs, verify proof, and issue attestation.
- **Third Party Applications/Protocols**: Only requires valid attestation ID/nullifier + ZK proof; never touches PII.

### SL Integration
- **Walrus**: credential_signed.enc, policy_manifest.json, historical revocation_merkle_root(s), and hash-chain audit logs.
- **Soundness Layer**: The app calls verifyKyc(proof, signals) on the Soundness Layer.The Soundness contract performs ZK verification and root revocation checks (updated periodically from Walrus metadata). It emits a KycVerified(nullifier, policy_id, expiry) event that can be read by apps on any chain (via relayer/IBC).

## Technical

### Architecture
1. Enrollment (once per user)
2. ZK Proof Creation (every time you want to access the service)
3. Proof Verification (on-chain, via Soundness Layer)
4. Audit & Compliance (SNARK (Groth16/PLONK), zkVM (RISC Zero / SP1).

### Stack
- **Frontend**: Rust/Go
- **zkVM**: (RISC Zero/SP1) + verification in Soundness Layer
- **Walrus CLI**: for upload artifacts (credential template, policy, CRL), record walrusBlobId; store the commits on-chain
- **Soundness CLI**: for for proof verification + revocation_root synchronization from Walrus metadata.
  
### Features
1. The prover only needs to prove that they have certain information (e.g., a secret key) without ever actually showing that information.
2. Proof is verified once, the result is carried over (reusable) by many dApps without expensive re-verification on each chain.
3. Trustless without reveal all data.

## Timeline

### PoC (2-4 weeks)
- Select proving stack: zkVM (RISC Zero/SP1) + verification in Soundness Layer.
- Define credential schema & policy manifest
- Arrange circuit/guest program
- Walrus setup: upload artifacts
- Deploy contracts in the Soundness Layer for proof verification + revocation_root synchronization from Walrus metadata.

### MVP (4-8 weeks)
- Data minimization
- Anti-Sybil
- Liveness/DA: Walrus ensures data availability and certification without full blob downloads.
- Observability: integration with tooling (e.g. Walrus Explorer/Space and Time) for network telemetry & transparency.
- Cross-chain adapter: KycVerified(nullifier, policy_id) event consumable for EVM/SVM dApps
- Negative test cases: fake credential, proof replay, revoked, expired, wrong policy, wrong blobId.

## Innovation
First identity system using SL for true cross-chain coordination with privacy preservation.

## Contact Discord / X
@zeandrn
