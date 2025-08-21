vApp Submission: zk-Social Proof (ZSP)

### Verification
github_username: "blustackk"
discord_id: "942396446286684210"
timestamp: "2025-03-21T00:00:00Z"

### Developer
Name: blustackk
GitHub: https://github.com/blustackk
Discord: blustackkk
Experience: I have experience in web3 development using Node.js and React, with a strong interest in privacy-preserving technologies and dApp integrations.

### Project
Project Name: zk-Social Proof (ZSP)
Category: social

Description:
ZSP enables decentralized communities, DAOs, and dApps to verify user social reputation without exposing private information.

Example use cases:
- Prove a Twitter account has >= 500 followers
- Prove a Discord account has been active for >= 30 days
- Prove a GitHub account is older than 6 months

All verifications are powered by zero-knowledge proofs (zk) to protect user privacy.

### Soundness Layer Integration
- Uses Soundness CLI to generate witness data from social platforms
- Creates zk-proofs and uploads them to the Soundness Layer to receive a blob_id
- Verifications can be:
  - Off-chain: via Soundness Layer verifier API
  - On-chain: through an EVM-compatible smart contract adapter

### Technical
Architecture:
- Client (Next.js): Handles social login and witness generation
- Worker: Executes Soundness CLI, manages proof jobs, uploads to SL
- SL Verifier: Validates proofs stored in Soundness Layer
- Smart Contract Adapter (optional): Enables on-chain proof validation

Stack:
- Frontend: Next.js (React)
- Backend: Node.js
- Blockchain: Soundness Layer + optional EVM networks
- Storage: IPFS for metadata, Soundness Layer for proofs

Core Features:
1. Private proof of social account ownership and activity
2. Dual-mode verification (off-chain & on-chain)
3. Token gating & DAO access control integration

### Timeline
PoC (2-4 weeks):
- Integrate basic social logins (Twitter/Discord/GitHub)
- Generate proofs using Soundness CLI
- Build minimal UI

MVP (4-8 weeks):
- Fully featured API & smart contract adapter
- Expanded dApp integrations
- User testing & feedback iteration

### Innovation
- Privacy-first: Zero-knowledge proofs prevent sensitive data leakage
- Cross-platform: Works with Twitter, Discord, and GitHub
- Flexible integration: Suitable for both off-chain apps and on-chain smart contracts

### Contact
Preferred contact: Discord (DM or developer channel)
Updates shared via: GitHub & Soundness Dev Discord

### Checklist
- All fields are completed
- GitHub username matches PR author
- Soundness Layer integration explained
- Timeline is realistic

