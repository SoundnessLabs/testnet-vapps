# vApp Submission: Proof-of-Community Membership

## Verification
```yaml
github_username: "zjoki4"
discord_id: "974909879053615194"
timestamp: "2025-09-01"
```

## Developer
- **Name**: bohki
- **GitHub**: @zjoki4
- **Discord**: bhoki4
- **Experience**: I have experience in full-stack development and blockchain integration, including smart contract development, API design, and backend systems. My background includes working with zero-knowledge proof frameworks, authentication flows, and community-driven applications. I have previously built Discord bots, DAO tooling, and dApp prototypes using technologies like Node.js, TypeScript, Solidity, and PostgreSQL. With this foundation, I can design, implement, and scale a Soundness Layer–integrated vApp from proof-of-concept to production-ready MVP.

## Project

### Name & Category
- **Project**: Proof-of-Community Membership
- **Category**: identity

### Description
- A decentralized verification system that enables online communities (e.g., Discord, DAOs, Telegram) to ensure their members are unique, human, and verified without exposing sensitive information. By leveraging Soundness Layer proofs, the system allows users to generate zero-knowledge proofs of membership and authenticity, effectively preventing bots, fake accounts, or Sybil attacks while protecting privacy.

### SL Integration  
The vApp integrates with the Soundness CLI to generate and validate membership proofs.
- Users create a proof of uniqueness and membership through the Soundness Layer.
- Community admins verify proofs before granting roles, tokens, or gated access.
- All proof verification runs off-chain using Soundness CLI, ensuring low latency and scalability.

## Technical

### Architecture
1. User requests to join a community.
2. The system triggers proof generation via Soundness CLI (verifying uniqueness/valid credentials).
3. A zero-knowledge proof is generated and stored.
4. Verification bot (Discord/DAO integration) checks the proof validity.
5. Upon success, user is granted verified membership role/access.

### Stack
- **Soundness CLI**: proof generation & validation
- **Node.js / TypeScript**: backend and bot logic
- **PostgreSQL / Redis**: storing proof references and session states
- **Discord API**: integration for community access control

### Features
1. Privacy-preserving community membership validation.
2. Prevention of Sybil attacks and fake accounts.
3. Easy integration with Discord, DAOs, and other communities.

## Timeline

### PoC (2-4 weeks)
- [ ] Integrate Soundness CLI to generate basic membership proofs
- [ ] Build a minimal backend service with API endpoints
- [ ] Create a simple Discord bot for proof submission and verification

### MVP (4-8 weeks)  
- [ ] Expand proof logic for uniqueness & identity checks
- [ ] Add admin dashboard to manage verified members
- [ ] Launch closed testnet pilot with 1–2 online communities

## Innovation
- This vApp introduces a new way to combat Sybil attacks in online communities using zero-knowledge proofs. Unlike traditional verification methods (KYC or CAPTCHA), it ensures privacy while guaranteeing uniqueness and authenticity. It also provides a reusable framework for other identity-driven applications built on the Soundness Layer.

## Contact
- Discord: bhoki4
- GitHub: @zjoki4


**Checklist before submitting:**
- [X] All fields completed
- [X] GitHub username matches PR author  
- [X] SL integration explained
- [X] Timeline is realistic
