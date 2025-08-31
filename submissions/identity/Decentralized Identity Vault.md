# vApp Submission: 

## Verification
```yaml
github_username: "biarinlah"
discord_id: "908610836686331924"
timestamp: "2025-08-31"
```

## Developer
- Name: biarin
- GitHub: @biarinlah
- Discord: binancehoki
- Experience: Experience: 4+ years in Web3 development with expertise in identity solutions, zk-SNARK circuits, and Ethereum smart contracts. Contributed to DID (Decentralized Identity) standards and worked on privacy-preserving authentication protocols

## Project

### Name & Category
- Project: Decentralized Identity Vault
- Category: identity

### Description
- A secure identity vault built on Soundness Layer to store and verify credentials, reputation scores, and verifiable attestations.

### SL Integration
- Use SL for zk-proof generation to verify credentials without exposing raw data.
- Leverage SL storage and validation for reputation tokens.

## Technical

### Architecture
1. User registers identity and credentials.
2. Data stored encrypted, proofs generated via SL.
3. Validators confirm credential authenticity.
4. Third-party dApps query zk-proofs for verification.

### Stack
- Solidity / zk-SNARK circuits
- Soundness CLI
- IPFS / Arweave for encrypted data
- React frontend

### Features
1. Private identity credentials.
2. Reputation scoring.
3. zk-proof credential sharing.
4. API for other apps to integrate.

## Timeline

### PoC (2-4 weeks)
- [ ] Basic identity registration
- [ ] Proof generation demo
- [ ] Simple verifier contract

### MVP (4-8 weeks)  
- [ ] Reputation scoring module
- [ ] API integration
- [ ] Frontend dashboard

## Innovation
- Brings privacy-first, verifiable identity to the SL ecosystem—essential for scalable trust in DeFi and beyond.

## Contact
- Discord: binancehoki
- GitHub: biarinlah

---

Checklist before submitting:
- [x] All fields completed  
- [x] GitHub username matches PR author  
- [x] SL integration explained  
- [x] Timeline is realistic  

