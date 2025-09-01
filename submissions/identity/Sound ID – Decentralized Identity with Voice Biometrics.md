# vApp Submission: Sound ID – Decentralized Identity with Voice Biometrics

## Verification
```yaml
github_username: "budiant281"
discord_id: "974919041347100683"
timestamp: "2025-09-02"
```

## Developer
- **Name**: budi
- **GitHub**: @budiant281
- **Discord**: budianto00
- **Experience**: I’m a full-stack Web3 developer with over 3 years of experience building decentralized applications on EVM-compatible blockchains. My expertise spans smart contract development (Solidity), zero-knowledge proof integration, and frontend dApp interfaces using React and Next.js.
I’ve previously contributed to several DeFi and identity-focused projects, including:
- A zk-based anonymous voting dApp using Semaphore
- A DAO participation tracker with Sybil-resistance
- An on-chain NFT ticketing system for live events
I’m passionate about decentralized identity, privacy-preserving technologies, and improving human authentication in Web3. I believe Sound ID brings a human-first solution to one of the space's most pressing problems: proving who you are, without compromising privacy.

## Project

### Name & Category
- **Project**: Sound ID – Decentralized Identity with Voice Biometrics
- **Category**: identity

### Description
- Sound ID is a decentralized identity protocol that uses voice as the primary authentication factor. It enables users to register a unique voiceprint, which is verified and hashed, then stored or linked to their on-chain identity. The uniqueness of the human voice makes this identity non-transferable and sybil-resistant — ideal for Web3 applications like secure login, community voting, and on-chain presence verification.
 By integrating Soundless Layer, Sound ID ensures that all voiceprints are human, unique, and not AI-generated. This makes it a powerful alternative to traditional identity systems in decentralized environments.

### SL Integration  
- Soundless CLI will be used to verify that the submitted voice samples are original, human, and unique.
- Proofs generated from Soundless will be embedded into the user’s identity record (e.g., stored on-chain as part of a DID).
- During login or interaction with dApps, real-time voice input will be verified using Soundless Layer to ensure authenticity.
- All submissions and identity updates will be validated using Soundless CLI and testnet tools.

## Technical

### Architecture
High-level system design and approach
1. Voice Enrollment: Users record a short voice sample on the frontend app.
2. Proof Generation: Voice sample is processed through Soundless CLI to generate a zero-knowledge proof of liveness and uniqueness.
3. Identity Binding: A hashed voiceprint + Soundless proof is stored on-chain and linked to the user’s wallet or DID.
4. Verification Layer: dApps can request real-time verification of a user’s voice using the Soundless API or SDK.
5. Security: No raw audio is stored; only cryptographic proofs and hashes are maintained.

### Stack
- **Frontend**: Next.js + React + Web3Auth
- **Backend**: Node.js + Express + PostgreSQL
- **Blockchain**: Solidity + EVM-compatible testnet
- **Soundless Integration**: Soundless CLI, Cockpit, @Sound_dev_vapps Discord bot

### Features
1. Decentralized voice-based identity registration
2. Real-time voice verification for dApps
3. Fully Sybil-resistant login & voting system

## Timeline

### PoC (2-4 weeks)
- [ ] Build voice enrollment interface (frontend)
- [ ] Integrate Soundless CLI for proof generation
- [ ] Deploy basic smart contract for identity binding

### MVP (4-8 weeks)  
- [ ]  Add real-time voice verification for login
- [ ] Develop developer API & SDK for dApps
- [ ] Launch testnet dApp and run user testing

## Innovation
- Sound ID introduces a novel way to authenticate users using voice biometrics, combined with Soundless Layer for zero-knowledge proof of human uniqueness. Unlike traditional wallets or passwords, this identity is biologically tied to the user and cannot be transferred, botted, or replicated by AI. This unlocks sybil-resistance for voting, identity recovery, and more — all while keeping UX intuitive and human-centered.

## Contact
- Discord: budianto00
- GitHub: @budiant281


**Checklist before submitting:**
- [X] All fields completed
- [X] GitHub username matches PR author  
- [X] SL integration explained
- [X] Timeline is realistic
