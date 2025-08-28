# vApp Submission:  Decentralized Identity Verification

## Verification
```yaml
github_username: "sentra0550"
discord_id: "1135536859032858698"
timestamp: "2025-08-28"
```

## Developer
- **Name**: Fiqri
- **GitHub**: @sentra0550
- **Discord**: sentra0550
- **Experience**: Brief background

## Project
ProofID – Decentralized Identity Verification
### Name & Category
- **Project**: ProofID – Decentralized Identity Verification
- **Category**: identity

### Description
ProofID solves the problem of trustless identity verification for Web3 apps without exposing sensitive personal information.
It enables users to prove attributes (age, residency, membership, credentials) via zero-knowledge attestations, while dApps can instantly validate proofs on-chain. This reduces onboarding friction for DeFi, gaming, and social platforms that require gated access.

### SL Integration  
Use Soundness Layer’s zk-proof verification to check identity attestations.
Rely on SL’s proof aggregation to batch-verify multiple credentials at low cost.
Leverage SL’s decentralized verification network for cross-chain identity portability.

## Technical
### Architecture
Users mint a non-transferable ProofID credential NFT.
Off-chain verifiers issue signed claims (e.g., KYC provider confirms "over 18").
SL verifies zk-proofs of claims without revealing raw data.
dApps query ProofID contracts to gate access.

### Stack
Frontend: React + Next.js
Backend: Node.js + Express
Blockchain: Soundness Layer + Ethereum (L2s for scaling)
Storage: WALRUS (encrypted claims), IPFS (public metadata)

### Features
1. Zero-knowledge proof-based identity verification
2. Non-transferable credential NFTs
3. Cross-dApp proof portability

## Timeline

### PoC (2-4 weeks)
- [ ] Credential NFT prototype
- [ ] SL proof verification integration
- [ ] Simple React UI for demo

### MVP (4-8 weeks)  
 Full zk-proof credential flows
 DeFi/social dApp integrations
 User testing with gated communities

## Innovation
Unlike traditional identity layers, ProofID is privacy-first and dApp-agnostic, leveraging SL’s zk-proof capabilities for trustless validation. Users own and control credentials, while dApps can enforce compliance without handling sensitive data.

## Contact
Discord DM: sentra0550
