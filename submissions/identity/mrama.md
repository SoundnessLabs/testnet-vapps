# vApp Submission: ZK Score

## Verification
```yaml
github_username: "galipeli"
discord_id: "905467633653153792"
timestamp: "2025-08-27"
```

## Developer
- **Name**: Ramadhani
- **GitHub**: @galipeli
- **Discord**: m.rama
- **Experience**: Web3 enthusiast with strong interest in ZK and privacy-based systems. Basic experience with smart contracts and backend development (Node.js & Solidity).

## Project

### Name & Category
- **Project**: ZK Score
- **Category**: identity

### Description
ZK Score is a Web3 reputation system that allows users to prove their on-chain value without revealing private off-chain data. The system collects off-chain activity (GitHub contributions, DAO participation, etc.), calculates a score locally, and generates a ZK proof using Soundness CLI. This proof can then be verified by a smart contract, ensuring trustless reputation validation.

### SL Integration  
ZK Score uses the Soundness Layer to:

- Fetch off-chain data (e.g., GitHub, Snapshot)
- Calculate a reputation score locally
- Generate a verifiable ZK proof using Soundness CLI
- Verify the proof on-chain through a smart contract

## Technical

### Architecture
1. Off-chain Data Fetcher: Pulls GitHub/DAO data via API
2. Reputation Calculator: Converts data into a score
3. Soundness CLI: Generates ZK proof of the score
4. Smart Contract: Verifies proof and stores reputation

### Stack
- **Frontend**: (Not planned for PoC)
- **Backend**: Node.js (data fetcher + reputation logic)
- **Blockchain**: Soundness Layer + EVM-compatible chain
- **Storage**: Local, optionally IPFS for metadata

### Features
1. Off-chain activity verification
2. ZK-based private scoring
3. On-chain score validation (DAO-ready)
   
## Timeline

### PoC (2-4 weeks)
- [x] Basic functionality
- [ ] SL integration
- [ ] Simple UI

### MVP (4-8 weeks)  
- [ ] Full features
- [ ] Production ready
- [ ] User testing

## Innovation
ZK Score offers a novel way to prove Web3 reputation without revealing underlying personal data. It's a private-by-default scoring system that supports Sybil resistance, DAO voting, and reputation-based access control—while keeping users in control of their data.

## Contact
- Discord: m.rama
- GitHub: @galipeli
- Updates will be shared in Soundness Discord after approval.


**Checklist before submitting:**
- [x] All fields completed
- [x] GitHub username matches PR author  
- [x] SL integration explained
- [x] Timeline is realistic
