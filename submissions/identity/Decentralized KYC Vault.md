# vApp Submission: Decentralized KYC Vault

## Verification
```yaml
github_username: "abrak2"
discord_id: "994625305111183503"
timestamp: "2025-08-30"
```

## Developer
- Name: abrak
- GitHub: @abrak2
- Discord: abrak_
- Experience: Web3 developer with background in smart contracts, zk-SNARK circuits, and decentralized identity systems. Contributed to multiple DeFi and privacy-preserving dApps.

## Project

### Name & Category
- Project: Decentralized KYC Vault
- Category: identity

### Description
Decentralized KYC Vault is a privacy-preserving system that allows users to complete KYC verification once and reuse cryptographic proofs across multiple dApps.  
It ensures compliance for exchanges, DeFi apps, and DAOs, while protecting user privacy through zk-SNARK proofs and Soundness Layer integration.

### SL Integration
This project leverages Soundness Layer to:
- Store encrypted KYC documents in decentralized storage.  
- Generate zk-SNARK proofs for "KYC passed" conditions.  
- Verify proofs on-chain via SL smart contracts, bound to user wallets.  
- Allow dApps to validate KYC status without accessing raw documents.  

## Technical

### Architecture
1. **User Onboarding**: User submits KYC docs → encrypted & stored on SL.  
2. **Proof Generation**: zk-SNARK circuits generate proof-of-verification (e.g., over 18, unique human).  
3. **SL Smart Contract**: Registers proof and binds it to wallet identity.  
4. **dApp Verification**: dApps call SL contract to check proof validity (yes/no).  

### Stack
- Frontend: React + Next.js  
- Backend: Node.js  
- Blockchain: Soundness Layer + Solidity contracts  
- Storage: SL encrypted storage + WALRUS/IPFS anchoring  

### Features
1. One-time KYC verification with reusable zk proof.  
2. Privacy-first validation across multiple dApps.  
3. Sybil-resistant identity binding (one-human-one-wallet).  
4. Selective disclosure proofs (e.g., only "over 18" shared).  

## Timeline

### PoC (2-4 weeks)
- [ ] Smart contract + zk circuit for basic "KYC passed" proof  
- [ ] SL integration for encrypted storage  
- [ ] Simple UI for document submission and proof generation  

### MVP (4-8 weeks)  
- [ ] Extended zk circuits (age, residency, uniqueness)  
- [ ] Full SL contract integration  
- [ ] Partner dApp testing and feedback  

## Innovation
Unlike centralized KYC providers, this system enables reusable, privacy-preserving identity verification across the Web3 ecosystem. It eliminates repeated document sharing, supports regulatory compliance, and empowers users to control their personal data through zero-knowledge proofs.  

## Contact
- Discord: abrak_  
- GitHub: @abrak2  

---

Checklist before submitting:
- [x] All fields completed  
- [x] GitHub username matches PR author  
- [x] SL integration explained  
- [x] Timeline is realistic  
