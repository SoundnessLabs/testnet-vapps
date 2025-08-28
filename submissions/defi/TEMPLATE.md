# vApp Submission: DeFi Wallet Tracker

## Verification
```yaml
github_username: "ahrabi839"
discord_id: "844820232353873951"
timestamp: "2025-08-28"
```
## Developer
- **Name**: ahrabi839
- **GitHub**: @ahrabi839
- **Discord**: ahrabi895
- **Experience**: Hands-on Web3 contributor. Built small automation tools/bots for testnets and airdrops; familiar with Node.js/JavaScript, ethers.js, simple indexers, and EVM tooling. Recently exploring basic zk-proof flows and integrating them into lightweight dashboards.
## Project
### Name & Category
- **Project**: DeFi Wallet Tracker
- **Category**: defi
### Description
A lightweight portfolio tracker for balances, transfers, and yield positions across EVM testnets. It generates shareable, privacy-preserving proofs so users can verify portfolio metrics (TVL, PnL, APY) without exposing raw addresses or sensitive data.
### SL Integration
We integrate Soundness Layer zk-proofs to verify wallet balances and transactions securely. This ensures privacy-preserving validation without requiring private key exposure.
## Technical
### Architecture
Frontend (React) â†’ Backend API (Node.js) â†’ Soundness Layer (zkProof validation) â†’ Database (PostgreSQL/IPFS)
### Stack
- **Frontend**: React + Vite
- **Backend**: Node.js (Fastify)
- **Blockchain**: Soundness Layer + Ethereum testnet
- **Storage**: PostgreSQL (app state), IPFS (proof artifacts)
### Features
1. Real-time portfolio dashboard (balances, positions, recent transfers)
2. Shareable zk-proof of portfolio metrics (privacy-preserving badges/links)
3. Address bundling: aggregate multiple wallets into one verifiable claim
## Timeline
### PoC (2-4 weeks)
- [ ] Wallet connect & read-only indexing
- [ ] Minimal SL proof: single-asset balance claim
- [ ] Simple UI + proof verification flow
### MVP (4-8 weeks)
- [ ] Multi-asset, multi-wallet aggregation
- [ ] Shareable proof badges & public verifier page
- [ ] Testnet deployment, user testing, docs
## Innovation
zk-proof portfolio claims let users prove â€œwhat they haveâ€ without exposing â€œwho they are.â€ This is safer for airdrop hunters and privacy-focused users while keeping data verifiable.
## Contact
Preferred contact: Telegram @ahrabi839 â€¢ GitHub Issues on the submission repo
**Checklist before submitting:**
- [x] All fields completed
- [x] GitHub username matches PR author
- [x] SL integration explained
- [x] Timeline is realistic