# vApp Submission: Decentralized Task Manager

## Verification
```yaml
github_username: "ygarov."
discord_id: "957480972922810410"
timestamp: "2025-09-22"
```

## Developer
- **Name**: Ygarov Anton
- **GitHub**: @payday-sepia
- **Discord**: ygarov.
- **Experience**: 5 years as a full-stack developer, specializing in blockchain applications and decentralized systems. Built multiple dApps on Ethereum and Solana.

## Project

### Name & Category
- **Project**: TaskChain
- **Category**: productivity

### Description
TaskChain is a decentralized task management vApp that solves the problem of centralized control and data privacy in traditional task management tools. It allows users to create, assign, and track tasks on a secure, transparent blockchain, ensuring data ownership and immutability. Users can collaborate across organizations without relying on a central authority, with tasks encrypted and accessible only to authorized parties

### SL Integration  
TaskChain leverages the Soundness Layer (SL) for secure, scalable transaction processing and smart contract execution. Specific SL features used include:Zero-Knowledge Proofs: For private task assignments and verifications.
SL's High-Throughput Consensus: To handle real-time task updates across distributed teams.
Smart Contract Templates: For automating task workflows and reward distribution.

## Technical

### Architecture
TaskChain uses a modular architecture with a frontend for user interaction, a backend for business logic, and SL for blockchain operations. Tasks are stored off-chain with metadata hashed on SL for integrity. Smart contracts manage task states and user permissions

### Stack
- **Frontend**: React with TypeScript
- **Backend**: Node.js with Express 
- **Blockchain**: SL for core logic, Ethereum for token-based rewards
- **Storage**: WALRUS for encrypted task data, IPFS for redundant file storage

### Features
1. Decentralized task creation and assignment with role-based access control.
2. Encrypted task data with zero-knowledge proofs for privacy  
3. Token-based incentives for task completion, integrated with SL smart contracts

## Timeline

### PoC (2-4 weeks)
- [ ] Basic task creation and assignment functionality
- [ ] SL integration for task metadata hashing
- [ ] Simple React-based UI

### MVP (4-8 weeks)  
- [ ] Full task management features (creation, assignment, tracking, completion)
- [ ] Production-ready smart contracts and encryption
- [ ] User testing with 50 beta users

## Innovation
TaskChain is unique because it combines privacy-preserving task management with blockchain-based transparency and incentives. Unlike centralized tools like Trello or Asana, TaskChain ensures users own their data and can collaborate securely across organizations. The integration of SL’s zero-knowledge proofs and high-throughput consensus makes it scalable and private, appealing to privacy-conscious teams and enterprises

## Contact
Preferred contact: "Discord ygarov."


**Checklist before submitting:**
- [ ] All fields completed
- [ ] GitHub username matches PR author  
- [ ] SL integration explained
- [ ] Timeline is realistic
