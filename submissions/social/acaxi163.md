# vApp Submission: zkHire

## Verification
```yaml
github_username: "acaxi163"
discord_id: "1074218986511794206"
timestamp: "2025-08-21"
```

## Developer
- **Name**: Acaxi  
- **GitHub**: @acaxi163  
- **Discord**: acaxi5775  
- **Experience**: 15+ years of professional software development experience, specializing in ERP systems design, customization, and integration. 2+ years in blockchain development.
## Project

### Name & Category
- **Project**: zkHire  
- **Category**: identity / social

### Description
zkHire is a privacy-preserving recruitment platform. It allows job candidates to prove they meet requirements (e.g., degree, certifications, years of experience) using zero-knowledge proofs, without revealing unnecessary personal details.  
For example, a candidate can prove "I hold a valid Computer Science degree from an accredited university" or "I have more than 3 years of Solidity development experience" — without exposing the actual documents or sensitive data.

This addresses the problem of privacy leakage in traditional hiring and prevents discrimination based on gender, age, or nationality.

### SL Integration
- Use **Soundness Layer** as the verification backend to handle modular zk verification efficiently.  
- Deploy zk circuits for credential attestations, age-proof, and skill-proof.  
- Rely on SL’s proof-agnostic verification to support SNARKs/STARKs depending on credential providers.  
- zkHire smart contracts will integrate SL’s APIs to validate proofs before granting applicants access to job listings.

## Technical

### Architecture
- Candidate stores credentials in encrypted form on decentralized storage.  
- Credential issuers (universities, cert platforms) provide zk attestations.  
- zkHire dApp verifies proofs via Soundness Layer before allowing access or matching.  
- Employers see only verified claims, not raw data.

### Stack
- **Frontend**: React + Next.js  
- **Backend**: Node.js + Express  
- **Blockchain**: Soundness Layer + Sui 
- **Storage**: WALRUS + IPFS for encrypted credential storage

### Features
1. zk-proof based credential verification  
2. Privacy-preserving candidate matching  
3. Employer dashboard with proof-verified filters (e.g., “CS degree + 3 years Solidity experience”)  

## Timeline

### PoC (5-10 weeks)
- [ ] Build simple zk circuits for degree & age verification  
- [ ] Integrate with Soundness Layer testnet  
- [ ] Minimal React UI for candidate proof submission

### MVP (8-12 weeks)
- [ ] Expand to skill/experience proofs  
- [ ] Employer job board with zk-filtering  
- [ ] Production-ready verification pipeline  
- [ ] User testing with testnet credential issuers

## Innovation
- Unlike traditional job platforms, zkHire allows **anonymous yet trustworthy hiring**.  
- Candidates maintain privacy while employers still get reliable proof of qualifications.  
- Prevents data misuse, reduces bias, and improves trust in decentralized hiring.  

## Contact
- Email: acaxi163@gmail.com  
- Discord: acaxi5775
