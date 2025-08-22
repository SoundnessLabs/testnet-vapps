# vApp Submission: ProofSocial

## Verification
```yaml
github_username: "davis9c"
discord_id: "358926301698195459" 
timestamp: "2025-08-22"
```

## Developer
- **Name**: Davis Purwo Aji
- **GitHub**: @davis9c
- **Discord**: cixzer
- **Experience**: Backend Developer Frontend Developer AI Enthusiast Android Developer

## Project

### Name & Category
- **Project**: ProofSocial
- **Category**: social

### Description
Proofsocial is a simple social application that allows users to prove that they are members of the community (for example Discord Server or Github Org) without having to share their personal data directly.
### SL Integration  
Use the Soundness Layer to create and verify ZK-Proofs that the user is really a member of a community.

SL will function as a prover/verifier so that the original data (such as email or username) does not need to be displayed in the blockchain.
## Technical

### Architecture
User → Frontend (React) → Backend (Node.js) → Soundness Layer prover/verifier → Blockchain (SL testnet).

### Stack
- **Frontend**: React + Tailwind
- **Backend**: Node.js + Express
- **Blockchain**: Soundness Layer testnet
- **Storage**: Minimum, only user metadata (postgreSQL or in-memory for POC)

### Features
1. User login with Github/Discord (OAUTH).
2. Generate Proof of Membership via SL.
3. Proof verification and display the "Verified Member" badge in social applications.

## Timeline

### PoC (2-4 weeks)
- [ ] Basic functionality
- [ ] SL integration
- [ ] Simple UI

### MVP (4-8 weeks)  
- [ ] Full features
- [ ] Production ready
- [ ] User testing

## Innovation
Social proof facilitates communication to provide the status of "verified members" with maintained privacy. No need to divulge original identity, quite a proof from the Soundness Layer.

## Contact
Discord: cixzer
GitHub: @davis9c


**Checklist before submitting:**
- [ ] All fields completed
- [ ] GitHub username matches PR author  
- [ ] SL integration explained
- [ ] Timeline is realistic
