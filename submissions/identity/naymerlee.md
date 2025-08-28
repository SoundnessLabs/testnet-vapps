# vApp Submission: [ChainTrust]

## Verification
```yaml
github_username: "naymerlee"
discord_id: "taroli24"
timestamp: "2025-08-28"
```

## Developer
- **Name**: Taro
- **GitHub**: @naymerlee
- **Discord**: taroli24
- **Experience**: Active Web3 community contributor, focusing on user growth, decentralized governance, and product research.

## Project

### Name & Category
- **Project**: ChainTrust
- **Category**: identity

### Description
ChainTrust is a decentralized reputation and credential system.
It solves the problem of fake identities and low trust in anonymous communities by providing verifiable on-chain credentials and a scoring system based on user activity and peer validation.

### SL Integration  
Use Soundness Layer (SL) for verifiable proofs of reputation.
SL ensures low-cost verification of credentials without revealing sensitive personal data.
Integrates SL’s ZK features for private scoring and attestations.

## Technical

### Architecture
User requests identity credential.
ChainTrust Protocol verifies with SL ZK attestation.
Credential stored on-chain + optional off-chain IPFS backup.
Applications (DeFi/social/gaming) can query credentials via ChainTrust API.

### Stack
Frontend: Next.js + Tailwind
Backend: Node.js (Express)
Blockchain: SL + Ethereum L2
Storage: IPFS + PostgreSQL

### Features
On-chain identity credential issuance
ZK-based reputation scoring
API for dApps to integrate trust scores

## Timeline

### PoC (2-4 weeks)
User credential minting demo
Basic SL ZK proof integration
Minimal web dashboard

### MVP (4-8 weeks)  
Full reputation scoring logic
dApp API support
Initial partner integration

## Innovation
Privacy-preserving identity: user proves reputation without revealing full data.
Cross-dApp utility: same reputation usable across DeFi, SocialFi, and Gaming.
Incentive alignment: good behavior = higher trust score, enabling access to more dApps.

## Contact
Github:@naymerlee

**Checklist before submitting:**
- [ ] All fields completed
- [ ] GitHub username matches PR author  
- [ ] SL integration explained
- [ ] Timeline is realistic
