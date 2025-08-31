# vApp Submission: DeFi Guard

## Verification
```yaml
github_username: "wildishine"
discord_id: "843492906719379466"
timestamp: "2025-08-31"
```

## Developer
- **Name**: wildishine
- **GitHub**: @wildishine
- **Discord**: wildishine
- **Experience**: Full-stack developer with experience in blockchain dApps, smart contracts, and web3 integrations.

## Project

### Name & Category
- **Project**: DeFi Guard
- **Category**: defi

### Description
DeFi Guard is a decentralized risk-protection vApp for users in DeFi protocols.
It helps users analyze risk exposure before making swaps, lending, or staking by providing real-time risk scoring and fraud detection signals powered by Soundness Layer’s zk-proofs.

Users get notified if a transaction may be interacting with malicious contracts, has abnormal slippage, or suspicious activity.

### SL Integration  
Use Soundness Layer’s zkApp proofs for verifying transaction safety without revealing sensitive trading data.

Integrate with SL’s intent-based transaction validation to block risky swaps.

Store verified proofs of user risk score on-chain for transparency.

## Technical

### Architecture
- Frontend: Web dashboard + wallet extension integration
- Backend: API service + zk-circuits for proof generation
- Soundness Layer: Risk proof validation + intent verification
- Database/Storage: Off-chain data in PostgreSQL + IPFS for logs

### Stack
- **Frontend**: React + Tailwind + ethers.js
- **Backend**: Node.js + Express  
- **Blockchain**: Soundness Layer + Ethereum testnet
- **Storage**: PostgreSQL + IPFS

### Features
1. Risk score dashboard for wallet & protocol safety
2. zk-proof backed “safe transaction” mode  
3. Historical record of verified safe transactions

## Timeline

### PoC (2-4 weeks)
- [ ] Risk scoring algorithm prototype
- [ ] SL zk-proof integration (basic)
- [ ] Simple UI dashboard

### MVP (4-8 weeks)  
- [ ] Browser wallet extension with safe-mode toggle
- [ ] Full SL integration for transaction validation
- [ ] User testing + feedback cycle

## Innovation
Most DeFi tools only alert users after losses.
DeFi Guard is proactive → it verifies before execution using zk-proofs on Soundness Layer. This ensures privacy-preserving, trustless, real-time protection.

## Contact
Preferred Contact: Discord DM (@wildishine)
Updates: GitHub repo + Discord dev channel


- [x] All fields completed
- [x] GitHub username matches PR author
- [x] SL integration explained
- [x] Timeline is realistic
