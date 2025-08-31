# vApp Submission: [Your Project Name]

## Verification
```yaml
github_username: "ira300"
discord_id: "207654997318434816"
timestamp: "2025-01-15"
```

## Developer
- **Name**: Ira
- **GitHub**: @ira300
- **Discord**: ira300
- **Experience**: analyze and develop software

## Project

### Name & Category
- **Project**: GamesHash
- **Category**: Gaming

### Description
GamesHash is a gaming platform that uses zero-knowledge proofs (zk-proofs) to ensure matches are fair and secure. Currently, the Snake game is implemented with zk-proofs validating the scores. The plan is to expand to other games and integrate all into the Soundness ecosystem.

### SL Integration  
We use the Soundness Layer to generate and verify game proofs, ensuring only legitimate data is submitted on-chain. This is done via the Soundness CLI for proof submission and verification.

## Technical

### Architecture
The platform consists of a frontend for user interaction, a backend handling proof generation via Nargo/Noir circuits, and integration with the Soundness Layer for blockchain proof verification. Storage uses IPFS and WALRUS for off-chain data.

### Stack
- **Frontend**: Vanilla JavaScript
- **Backend**: Rust (Noir/Nargo), Solidity (smart contracts)
- **Blockchain**: SL + others
- **Storage**: WALRUS/IPFS

### Features
1. ZK proofs to validate moves and scores in Snake.
2. On-chain verification of game outcomes.
3. Framework to easily add more games following the same pattern.

## Timeline

### PoC (2-4 weeks)
- [X] Basic Snake game functionality with zk-proofs.
- [X] Integration with Soundness Layer for proof submission.
- [X] Simple UI

### MVP (4-8 weeks)  
- [ ] Full features
- [X] Production ready
- [X] User testing

## Innovation
GamesHash leverages zk-proofs to guarantee fairness and security in gaming by decentralizing verification on the blockchain with Soundness Layer, making game outcomes transparent and trustless.

## Contact
Preferred contact via Discord: Ira300


**Checklist before submitting:**
- [X] All fields completed
- [X] GitHub username matches PR author  
- [X] SL integration explained
- [X] Timeline is realistic
