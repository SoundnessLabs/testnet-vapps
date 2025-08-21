# vApp Submission: [Seven App]

## Verification
```yaml
github_username: "b3rk4h"
discord_id: "759635242704175145"
timestamp: "2025-08-21"
```

## Developer
- **Name**: TJ
- **GitHub**: @b3rk4h
- **Discord**: tjseven
- **Experience**: Web3 developer with 2+ years in dApps and ZK integrations

## Project

### Name & Category
- **Project**: Seven App
- **Category**: identity

### Description
Seven App provides decentralized, privacy-preserving identity.
It solves the issue of relying only on wallet addresses or centralized logins by enabling verifiable identities across dApps using zero-knowledge proofs.

### SL Integration  
- Use SL to generate and verify ZK-proofs of identity.
- Store attestations on SL for cross-dApp usage.
- Integrate SL contracts for login/verification.

## Technical

### Architecture
Frontend (React) → Backend (Node.js) → SL Contracts → IPFS/WALRUS storage.
User logs in → generates ZK proof → proof verified on SL → access granted.

### Stack
- **Frontend**: React
- **Backend**: Node.js  
- **Blockchain**: SL + EVM testnet
- **Storage**: WALRUS/IPFS

### Features
1. Decentralized identity wallet
2. ZK-based login/verification  
3. Attestation registry
4. User dashboard

## Timeline

### PoC (2-4 weeks)
- [ ] Wallet login  
- [ ] SL proof generation  
- [ ] Basic UI  

### MVP (4-8 weeks)
- [ ] Full verification + registry  
- [ ] Dashboard integration  
- [ ] Community testing  

## Innovation
Seven App enables cross-platform, privacy-first identity with ZK-proofs, combining self-sovereign identity and interoperability in Web3.

## Contact
Discord: tjseven

**Checklist before submitting:**
- [x] All fields completed
- [x] GitHub username matches PR author  
- [x] SL integration explained
- [x] Timeline is realistic
