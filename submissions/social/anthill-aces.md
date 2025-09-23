# vApp Submission: PeerVote

## Verification
```yaml
github_username: "anthill-aces"
discord_id: "957823989722533948"
timestamp: "2025-09-23"
```

## Developer
- **Name**: Makar Kapustin
- **GitHub**: @anthill-aces
- **Discord**: lefevresebastien
- **Experience**: 4 years as a software engineer, specializing in distributed systems and 1.5 years in Web3 development with expertise in smart contracts.

## Project

### Name & Category
- **Project**: PeerVote
- **Category**: social

### Description
PeerVote is a decentralized voting platform that addresses issues of transparency and trust in online polls. It enables users to create, participate in, and verify polls on the blockchain, ensuring immutable results and preventing manipulation. The vApp supports anonymous voting while maintaining verifiable outcomes, ideal for community governance and public opinion surveys

### SL Integration  
PeerVote utilizes Soundness Layer (SL) for secure and efficient vote processing. Specific SL features:
SL's zero-knowledge proofs: For anonymous yet verifiable voting.
SL's scalable transaction layer: To handle high-volume votes efficiently.
SL's smart contract framework: For automated vote tallying and result publication.

## Technical

### Architecture
PeerVote employs a layered architecture with a frontend for user interaction, a backend for vote processing, and SL for blockchain operations. Votes are encrypted client-side, with only hashes stored on-chain via SL. Off-chain storage holds encrypted vote details, linked to on-chain records for verification. The system ensures low-latency vote submission and result display

### Stack
- **Frontend**: Vue.js, JavaScript
- **Backend**: Node.js, Express  
- **Blockchain**: Soundness Layer, Ethereum
- **Storage**: WALRUS for vote data, Redis for caching

### Features
1. Anonymous vote submission with zk-proof verification
2. Real-time result visualization  
3. Public auditability of vote integrity

## Timeline

### PoC (2-4 weeks)
- [ ] Basic poll creation and voting
- [ ] SL integration for vote storage
- [ ] Simple Vue-based UI

### MVP (4-8 weeks)  
- [ ] Full voting and auditing features
- [ ] Production-ready with SL smart contracts
- [ ] User testing with 100 beta participants

## Innovation
PeerVote stands out by combining anonymity with transparency, using SL’s zero-knowledge proofs to protect voter privacy while ensuring results are publicly verifiable. Unlike centralized platforms like SurveyMonkey, it eliminates single points of failure and builds trust through decentralization, making it ideal for DAOs, community polls, and grassroots movements

## Contact
Discord (lefevresebastien)


**Checklist before submitting:**
- [ ] All fields completed
- [ ] GitHub username matches PR author  
- [ ] SL integration explained
- [ ] Timeline is realistic
