# vApp Submission: [soundness project]

## Verification
```yaml
github_username: "pradanaaa999"
discord_id: "570312933813518346"
timestamp: "2025-01-15"
```

## Developer
- **Name**: Pradana
- **GitHub**: @pradanaaa999
- **Discord**: kaaa#4051
- **Experience**: Fronten Developer, Backend Developer and Web3 enthusiast, actively managing and running multiple nodes (Aztec, Nexus, EthStorage, Titan, etc.), blockchain integration & distributed infrastructure experience.

## Project

### Name & Category
- **Project**: zk-Reputation Passport
- **Category**: identity & infrastructure

### Description
zk-Reputation Passport is a zero-knowledge based identity system that allows users to prove their reputation without leaking personal data.
-> Proving DeFi transaction history (e.g. trader activity) without leaking the entire wallet.

### SL Integration  
- Use the Soundness Layer to validate zk-proof reputation.
- Integrate proof verification directly into the SL chain to reduce reliance on a central server.
- Discord bots will query the Soundness Layer to check the validity of proofs before granting roles/access.

## Technical

### Architecture
1. **Proof Generator**: Client generates zk-proof from data (GitHub commits, wallet activity, Discord metadata).
2. **Verifier**: Soundness Layer verifies the proof trustlessly.
3. **Applications**: 
- Discord bot gating channels. 
- Smart contract gating (DAO, DeFi whitelist).

### Stack
- **Frontend**: React/Vue/etc (dasboard pasport)
- **Backend**: Rust/Node.js/Python/etc  (API + prof aggregation)
- **Blockchain**: Soundness Layer testnet + optional Ethereum L2
- **Storage**: IPFS untuk metadata, WALRUS untuk logs/analytics

### Features
1. **Privacy-preserving reputation proofs**  
2. **Discord & DAO integration** untuk gating akses  
3. **Multi-platform reputation** (GitHub, wallet, social)  

## Timeline

### PoC (2-4 weeks)
- [ ] Proof of GitHub contribution (basic zk-proof)
- [ ] Integration with Soundness Layer verifier
- [ ] Simple Discord bot for verification

### MVP (4-8 weeks)  
- [ ] Multi-platform integration (GitHub + Discord + wallet)
- [ ] React dashboard for Passport users
- [ ] DAO/DeFi integration for whitelisting/voting

## Innovation
user friendly, Most Web3 identities are still wallet-based (address-based). zk-Reputation Passport introduces the concept of a “multi-platform identity with privacy” that can be used for DAO governance, DeFi, and Web3 communities — without invasive KYC.

## Contact
Discord : 570312933813518346
Github : pradanaaa999 


**Checklist before submitting:**
- [ v ] All fields completed
- [ v ] GitHub username matches PR author  
- [ v ] SL integration explained
- [ v ] Timeline is realistic
