# vApp Submission: AnonCV – Zero-Knowledge Resume

## Verification
```yaml
github_username: "juanpabbello"
discord_id: "988614071190450206"
timestamp: "2025-09-01"
```

## Developer
- **Name**: Juan Pabello
- **GitHub**: @juanpabbello
- **Discord**: pabbello
- **Experience**: Experience in blockchain testnets, zkApp experimentation, and decentralized identity solutions.

## Project

### Name & Category
- **Project**: AnonCV
- **Category**: identity / social

### Description
AnonCV solves the problem of **privacy-invasive resumes and credential sharing**.  
Instead of exposing full CVs or personal data, users can **prove claims like education, work history, or skills through zero-knowledge proofs**.  

Examples:
- Prove you graduated without sharing transcripts.  
- Prove you are over 21 without sharing birthdate.  
- Prove years of experience without exposing employer details.  
- DAOs and Discord servers can onboard based on skill proofs, not KYC.  

### SL Integration  
- Use **Soundness Layer proving system (ligetron)** to generate zkProofs for claims.  
- Store encrypted claims off-chain and proofs on-chain for validation.  
- Allow verifiers (employers, DAOs, protocols) to check proof validity via SL without accessing raw data.  

## Technical

### Architecture
1. User creates encrypted CV claims (degree, skill, experience).  
2. Client generates zkProof for each claim.  
3. Proof submitted and validated via Soundness Layer.  
4. Verifier checks proof validity (yes/no) → without seeing private data.  

### Stack
- **Frontend**: React + Tailwind  
- **Backend**: Node.js (Express) + Rust (proof engine)  
- **Blockchain**: Soundness Layer (Testnet)  
- **Storage**: IPFS for encrypted claims  

### Features
1. Create anonymous CV with zkProof-based claims  
2. Verification through SL contracts and APIs  
3. Discord/DAO bot for proof-based role assignment  

## Timeline

### PoC (2-4 weeks)
- [ ] Define CV schema + zk circuits (age, degree)  
- [ ] Integrate SL CLI + proof submission  
- [ ] Basic UI for creating/verifying claims  

### MVP (4-8 weeks)  
- [ ] Expand claims (skills, experience, reputation)  
- [ ] Full web UI + Discord bot integration  
- [ ] Early user testing with DAOs and small communities  

## Innovation
Unlike traditional resumes or KYC, **AnonCV enables selective disclosure**.  
It’s the first zk-based CV platform where users **control what they prove without exposing sensitive details**.  
This bridges **identity, social, and DeFi onboarding** in one solution.  

## Contact
- Discord: pabbello  
- Discord ID: 988614071190450206  
- Email: juanpabbello@gmail.com  
- Updates shared via GitHub repo + Discord community  

---

✅ Checklist before submitting:
- [x] All fields completed  
- [x] GitHub username matches PR author  
- [x] SL integration explained  
- [x] Timeline is realistic  
