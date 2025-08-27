# vApp Submission: [Your Project Name]

## Verification
```yaml
github_username: "https://github.com/Decknova"
discord_id: "393123490355478528"
timestamp: "2025-08-27"
```

## Developer
- **Name**: Decknova
- **GitHub**: @Decknova
- **Discord**: decknova
- **Experience**: Brief background

## Project

### Name & Category
- **Project**: SentryID
- **Category**: identity
- **short_description**: "Decentralized identity dApp for Authentication, Credentials, and Reputation with strong privacy guarantees


### Description
SentryID enables users to:
1. Register **credentials** (KYC-lite, verified email, certificates) as *verifiable credentials*  
2. Authenticate to third-party services without disclosing sensitive data using **zero-knowledge proofs**
3. Build and display a **reputation score** based on credentials and both on-chain/off-chain interactions


### SL Integration  
- Use SL to store/serve **validated proof blobs** 
- Identity verification is performed off-chain; results are generated as zk-proofs and stored/served through SL blob IDs
- Smart contracts on-chain only read *proof commitments* / metadata from SL, ensuring no personal data is exposed on-chain

## Technical
- Frontend: React + Vite, WalletConnect / MetaMask integration
- Backend: Node.js (Express) for orchestration, key management handled client-side (users hold their own private keys)
- Crypto: zkSNARKs / zk-STARKs (circuits for attribute verification), BLS signatures for credential aggregation.  
- Storage: Soundness Layer blobs for proofs, IPFS for non-sensitive artifacts  
- Smart Contracts: Solidity (verifier contract) to accept proof commitments and issue attestation tokens

### Architecture
High-level system design and approach

### Stack
- **Frontend**: React/Vue/etc
- **Backend**: Rust/Node.js/Python/etc  
- **Blockchain**: SL + others
- **Storage**: Database/WALRUS/IPFS/etc

### Features
1. Credential registration: users upload documents → receive verifiable credential.  
2. Generate ZK-proofs for claims (e.g., "age > 18", "residency verified").  
3. One-click authentication to partner dApps via proof (no PII sharing).  
4. Reputation engine: scoring from credentials + attestations.  
5. Admin dashboard (trusted issuers) for issuing/revoking credentials.  

### Security
- Sensitive data encrypted client-side; only zk-proofs stored in SL.  
- Minimal disclosure: selective disclosure for claims.  
- Audit plan: external security audit before mainnet

  
## Timeline
- Week 0–2: Setup repo, fork, build basic circuit (prove age/email).  
- Week 3–4: Integrate SL blob storage & proof upload flow.  
- Week 5–8: Frontend auth flow + partner demo (PoC).  
- Week 9–12: Reputation module, testing, audit prep (MVP).  

### PoC (2-4 weeks)
- [x] Basic credential registration & ZK-proof generation
- [x] Store proofs in Soundness Layer blobs
- [ ] Simple React UI for credential submission & proof demo

### MVP (4-8 weeks)  
- [ ] Full authentication flow with partner dApp
- [ ] Reputation engine implementation
- [ ] User testing & feedback loop
- [ ] Security review & optimizations

## Innovation
- Combination of verifiable credentials + SL as proof store → fast, private, decentralized.  
- Allows dApps to adopt verifiable authentication without handling sensitive data or compliance burdens.  
- Reputation is portable and reusable across services

## Contact
- email: yudy.aniis66@gmail.com
- discord: decknova  
- github: https://github.com/Decknova


**Checklist before submitting:**
- [x] All fields completed
- [x] GitHub username matches PR author  
- [x] SL integration explained clearly
- [x] Timeline is realistic
