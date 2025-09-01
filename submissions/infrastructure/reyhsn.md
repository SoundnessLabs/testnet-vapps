# vApp Submission: [Your Project Name]

## Verification
```yaml
github_username: "reyhsn"
discord_id: "1227142910471045142"
timestamp: "2025-09-01"
```

## Developer
- **Name**: Reyhan
- **GitHub**: @reyhsn
- **Discord**: ngaplaycay#0
- **Experience**: 
Web3 developer with experience in zkApps, smart contract development, and building proof-of-concept dApps
## Project

### Name & Category
- **Project**: Quilt Batch Proof Logger
- **Category**: infrastructure

### Description
This vApp demonstrates the use of Soundness Layer's Quilt feature for batching zkProofs efficiently.
Instead of submitting proofs individually, users can submit multiple proofs (e.g., from AI inference logs, voting data, or game scores) and aggregate them into one on-chain commitment. This reduces gas costs and improves scalability for proof-heavy applications.

### SL Integration  
Use Quilt API to batch up to 600 zkProofs into one submission.

Store a single aggregated commitment on-chain via Soundness Layer.

Enable cost-efficient, scalable infrastructure for dApps needing high-frequency proof logging.

## Technical

### Architecture
Flow: User submits proof → backend collects proofs → Quilt batches them → commitment stored on-chain → verifier contract ensures integrity.

Backend batching service to queue proofs until threshold is reached or timer expires.

On-chain contract validates batched commitments.

### Stack
- **Frontend**: React + wallet integration (Sui wallet)
- **Backend**: Node.js/Express batching service 
- **Blockchain**: Soundness Layer on Sui, with future Ethereum attestations
- **Storage**: Quilt (Walrus), optional off-chain database (Postgres)

### Features
1. Core feature 1 Submit zkProofs via simple UI
2. Core feature 2 Batch up to 600 proofs into one commitment
3. Core feature 3 On-chain contract to verify integrity of batched proofs

## Timeline

### PoC (2-4 weeks)
- [x] Basic functionality
- [x] SL integration
- [x] Simple UI

### MVP (4-8 weeks)  
- [x] Full features
- [x] Production ready
- [x] User testing

## Innovation
While most zkApps focus on individual proofs, this vApp uniquely showcases high-frequency, cost-efficient proof batching. It enables new categories like AI logging, decentralized voting, and gaming that require many proofs but cannot afford high costs.

## Contact
Telegram/Discord DM: @reyhsn

Updates will be shared in the Soundness Layer Discord.


**Checklist before submitting:**
- [x] All fields completed
- [x] GitHub username matches PR author  
- [x] SL integration explained
- [x] Timeline is realistic
