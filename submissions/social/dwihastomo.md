# vApp Submission: [Your Project Name]

## Verification
```yaml
github_username: "dwihastomo"
discord_id: "1227513117056045127"
timestamp: "2025-09-01"
```

## Developer
- **Name**: Dwi Hastomo
- **GitHub**: @dwihastomo
- **Discord**: acrux_xs#0
- **Experience**: Full-stack developer with hands-on experience building community applications, OAuth integrations, and EVM-compatible smart contracts. Comfortable working across frontend, backend, and blockchain layers with a focus on privacy-preserving systems

## Project

### Name & Category
- **Project**: zkCommunity Gate (ZKG)
- **Category**: social

### Description
zkCommunity Gate (ZKG) is a social vApp designed to bring secure, privacy-preserving community membership into the Soundness Layer ecosystem.

The problem:

Community channels often suffer from spam, bots, and sybil attacks.

Reputation and contribution tracking are fragmented and difficult to verify.

Users are forced to expose personal identifiers (Discord usernames, emails) just to prove they belong.

The solution:
ZKG enables community access and participation through zero-knowledge proofs (ZKPs). Users can prove they meet reputation or badge requirements, or that they have not exceeded message rate limits, without revealing their actual identity. This balances privacy with accountability and provides moderators with verifiable tools to manage access

### SL Integration  
ZKG will integrate directly with the Soundness Layer verifier:

Membership proofs: Users generate proofs binding their Discord account to a wallet, but only a nullifier (anonymous identifier) is revealed on-chain.

Reputation checks: Proofs include reputation scores and badges stored in a soulbound token (SBT).

Anti-spam enforcement: Each proof carries a nullifier and epoch, enabling on-chain rate limiting without identity leakage.

This makes Soundness Layer the backbone for secure and private community gating.

## Technical

### Architecture
Frontend: A web app that handles wallet connection, Discord OAuth, and proof generation using the SL prover SDK.

Backend: A Node.js service that stores salted bindings between Discord IDs and wallets (hashed), relays webhook events (e.g., GitHub PRs, community contributions), and updates reputation on-chain.

Smart Contracts:

ReputationSBT: Non-transferable tokens representing scores and badges.

ZKGate: Verifies ZK proofs via the Soundness Layer and enforces community policies (minimum score, required badges, daily limits).

Storage:

On-chain: Reputation data, access proofs, and policy rules.

Off-chain: Messages stored in a database or IPFS, with hashes logged on-chain for auditability.

### Stack
- **Frontend**: Next.js + WalletConnect
- **Backend**: Node.js + Express  
- **Blockchain**: SL + others
- **Storage**: Database/WALRUS/IPFS/PostgreSQL + IPFS

### Features
1. Anonymous membership verification — join community channels using ZK proofs instead of revealing Discord IDs
2. Reputation-driven access — badges and scores determine eligibility for gated channels or privileges  
3. Spam prevention with ZK rate-limiting — ensures users cannot exceed their daily message quota, while staying anonymous

## Timeline

### PoC (2-4 weeks)
- [x] Basic functionality
- [x] SL integration
- [x] Simple UI

### MVP (4-8 weeks)  
- [x] Full features
- [x] Production ready
- [x] User testing

## Innovation
What makes this unique? Why will people use it?

## Contact
Preferred contact method and where you'll share updates.


**Checklist before submitting:**
- [x] All fields completed
- [x] GitHub username matches PR author  
- [x] SL integration explained
- [x] Timeline is realistic
