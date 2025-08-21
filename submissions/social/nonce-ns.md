# vApp Submission: [ConnectSphere]

## Verification
```yaml
github_username: "nonce-ns"
discord_id: "660475312127213578"
timestamp: "2025-08-22"
```

## Developer
- **Name**: Nasrul Hakim
- **GitHub**: @nonce-ns
- **Discord**: noncen_ns
- **Experience**: 2 years of experience in full-stack development with a focus on blockchain technologies and decentralized applications.

## Project

### Name & Category
- **Project**: ConnectSphere
- **Category**: social

### Description
ConnectSphere is a decentralized social media platform that empowers content creators by giving them full ownership of their content and direct access to their audience. It solves the problem of censorship, unfair monetization, and data exploitation prevalent in current centralized social media platforms.

### SL Integration  
We will leverage the Soundness Layer as the core infrastructure for content management and user interactions. Specifically, we'll use SL's smart contracts to create a transparent and decentralized system for content ownership (as NFTs) and implement ZK-proofs to offer private messaging and interactions, ensuring user data remains confidential.

## Technical

### Architecture
Our architecture is built around a set of smart contracts on the Soundness Layer that govern content posting, user reputation, and monetization. The frontend will interact directly with these contracts. User-generated content will be stored on IPFS to ensure decentralization, with metadata referenced on the blockchain.

### Stack
- **Frontend**: React, TypeScript
- **Backend**: Node.js, GraphQL
- **Blockchain**: SL + SUI
- **Storage**: Database, WALRUS, IPFS, PostgreSQL for off-chain data indexing

### Features
1. **Decentralized Content Ownership**: Users mint their posts as NFTs, giving them true ownership.
2. **Censorship-Resistant Feeds**: Content feeds are managed by community-governed rules, not a central authority.
3. **Direct Creator Monetization**: Tipping, subscriptions, and exclusive content powered by crypto payments.

## Timeline

### PoC (2-4 weeks)
- [✅] Basic functionality
- [✅] SL integration
- [✅] Simple UI

### MVP (4-8 weeks)  
- [✅] Full features
- [✅] Production ready
- [✅] User testing

## Innovation
ConnectSphere is unique because it combines content ownership with robust privacy features using the Soundness Layer. Unlike other platforms, it creates a truly sovereign social graph where users, not corporations, control their digital presence and community. This will attract users seeking freedom and direct engagement.

## Contact
The best way to reach me is on Discord. I will be sharing regular progress updates in the Soundness Labs community channels.


**Checklist before submitting:**
- [✅] All fields completed
- [✅] GitHub username matches PR author  
- [✅] SL integration explained
- [✅] Timeline is realistic
