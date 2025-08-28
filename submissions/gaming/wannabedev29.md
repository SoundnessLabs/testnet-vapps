# vApp Submission: [Your Project Name]

## Verification
```yaml
github_username: "wannabedev29"
discord_id: "1253111433881849931"
timestamp: "2025-01-15"
```

## Developer
- **Name**: erysdratz
- **GitHub**: @wannabedev29
- **Discord**: s4dmumu
- **Experience**: Enthusiastic learner exploring blockchain & zk tech. Currently focused on building practical vApps on Soundness Layer, with hands-on experience integrating smart contracts, zk-proofs, and testnet participation.

## Project

### Name & Category
- **Project**: zkSnounDphin
- **Category**: gaming

### Description
zkSnounDphin is a playful Flappy Bird–style browser game where players guide a dolphin through obstacles. The innovation: player achievements generate zero-knowledge proofs, enabling on-chain verification of high scores via Soundness Layer. This ensures fair competition, preserves privacy, and creates a verifiable “gaming reputation” tied to SL identity.

### SL Integration  
zkSnounDphin leverages Soundness CLI and proof system to record scores as zk-proofs. Each milestone or high score generates a proof that can be submitted on-chain using soundness-cli send. This ties game results to a verifiable SL identity, making gaming achievements portable, provable, and reputation-driven.

## Technical

### Architecture
Browser-based game frontend interacts with backend service for proof generation. Proofs are packaged and submitted to Soundness Layer via CLI.

### Stack
- **Frontend**: React(optional)
- **Backend**: Rust/Node.js/Python/etc  
- **Blockchain**: Soundness Layer testnet
- **Storage**: Database/WALRUS/IPFS/etc

### Features
1. Flappy Bird–style gameplay with dolphin character
2. zk-proof generation for high scores
3. Proof submission & verification on Soundness Layer

## Timeline

### PoC (2-4 weeks)
- Basic dolphin gameplay
- Initial zk-proof integration
- Submit proof to SL testnet via CLI

### MVP (4-8 weeks)  
- Full game features (badges, leaderboard)
- User dashboard with verified scores
- Community testing on SL

## Innovation
zkSnounDphin introduces the first casual game where player skill can be proven cryptographically without revealing raw gameplay data. It makes zk-tech approachable and fun, while showcasing how Soundness Layer can power verifiable gaming and portable digital reputation.

## Contact
Discord: s4dmumu | GitHub: @wannabedev29 | Updates via project repo


**Checklist before submitting:**
- ✅ All fields completed
- ✅ GitHub username matches PR author
- ✅ SL integration explained
- ✅ Timeline is realistic

