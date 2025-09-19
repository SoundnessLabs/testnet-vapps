# vApp Submission: Decentralized Task Manager

## Verification
```yaml
github_username: "ckvorcobakriss"
discord_id: "957765974151553025"
timestamp: "2025-09-19"
```

## Developer
- **Name**: Сkvorcoba Kristina
- **GitHub**: @ckvorcobakriss
- **Discord**: kaufmanerna_
- **Experience**: 5 years as a full-stack developer, with 2 years focused on blockchain and decentralized applications using Rust and TypeScript

## Project

### Name & Category
- **Project**: TaskMaster
- **Category**: productivity

### Description
TaskMaster is a decentralized task management vApp that solves the problem of centralized control and data privacy in traditional task management tools. It allows users to create, assign, and track tasks securely on the Soundness Layer (SL), ensuring data ownership and tamper-proof records. Users can collaborate across teams with encrypted task data, accessible only to authorized participants.

### SL Integration  
TaskMaster leverages Soundness Layer for:Identity: Decentralized identity for user authentication and task assignment.
Smart Contracts: Task creation, status updates, and completion verification.
Data Storage: Storing task metadata and encrypted user data on SL’s secure infrastructure

## Technical

### Architecture
TaskMaster uses a client-serverless model with a frontend interfacing directly with SL smart contracts. Task data is stored on SL’s decentralized storage, with off-chain metadata cached for performance. The system ensures scalability through SL’s transaction throughput and low-latency consensus

### Stack
- **Frontend**: React with TypeScript
- **Backend**: Rust (for SL smart contract development)  
- **Blockchain**: Soundness Layer
- **Storage**: SL Storage + IPFS for redundant task data

### Features
1. Decentralized task creation and assignment with role-based access
2. Real-time task status updates with SL smart contract triggers  
3. Encrypted task notes and attachments, accessible only to assigned users

## Timeline

### PoC (2-4 weeks)
- [ ] Basic task creation and assignment on SL
- [ ] SL identity integration for user authentication
- [ ] Minimal React-based UI for task interaction

### MVP (4-8 weeks)  
- [ ] Full task management features (create, edit, assign, complete)
- [ ] Production-ready smart contracts and encryption
- [ ] User testing with 50 beta testers

## Innovation
TaskMaster is unique in its privacy-first approach, using SL’s decentralized identity and storage to ensure users retain full control over their task data. Unlike centralized tools like Trello or Asana, TaskMaster prevents data harvesting and offers verifiable task histories, appealing to privacy-conscious teams and enterprises

## Contact
Preferred contact: Discord (kaufmanerna_)


**Checklist before submitting:**
- [ ] All fields completed
- [ ] GitHub username matches PR author  
- [ ] SL integration explained
- [ ] Timeline is realistic
