# vApp Submission: QuestPay

## Verification
```yaml
github_username: "feeder-irate"
discord_id: "957578663006711829"
timestamp: "2025-09-19"
```

## Developer
- **Name**: Demidov Andrew
- **GitHub**: @feeder-irate
- **Discord**: demidovandru
- **Experience**: Full-stack developer with 6+ years of experience in Web3, DeFi protocols, and secure smart contract development. Previously contributed to open-source Ethereum tooling.

## Project

### Name & Category
- **Project**: QuestPay
- **Category**: defi

### Description
QuestPay is a decentralized milestone-based payment platform for freelancers and teams. It allows clients to lock funds into smart contracts and automatically release them once predefined milestones are met, solving the problem of trust and delayed payments in remote work.

### SL Integration  
QuestPay uses Soundness Layer to:
- Verify milestone completion through cryptographic proofs and oracle attestations
- Leverage SL’s zk-proof APIs for dispute resolution without revealing sensitive project data
- Provide tamper-proof transaction logs accessible to both parties

## Technical

### Architecture
A React frontend interacts with a Node.js backend that manages smart contract deployment and communicates with SL APIs. Milestone data and proofs are stored on IPFS with references anchored on-chain. SL smart contracts handle verification and escrow logic.

### Stack
- **Frontend**: React + Next.js
- **Backend**: Node.js + GraphQL  
- **Blockchain**: SL + Polygon
- **Storage**: IPFS + WALRUS

### Features
1. Create milestone-based escrow contracts
2. Submit & verify proofs of milestone completion using SL
3. Automated dispute resolution and transparent payment tracking

## Timeline

### PoC (2-4 weeks)
- [ ] Basic milestone contract creation
- [ ] SL integration for proof verification
- [ ] Simple user dashboard

### MVP (4-8 weeks)  
- [ ] Full milestone escrow system
- [ ] Production-ready UI & backend
- [ ] User testing with early adopters

## Innovation
QuestPay uniquely combines milestone-based escrow with zero-knowledge proof verification, enabling trustless freelance work payments without revealing confidential project details. This makes it more secure and transparent than existing platforms

## Contact
Preferred contact: Discord - demidovandru


**Checklist before submitting:**
- [ ] All fields completed
- [ ] GitHub username matches PR author  
- [ ] SL integration explained
- [ ] Timeline is realistic
