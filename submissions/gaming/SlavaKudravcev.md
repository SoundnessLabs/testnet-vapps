# vApp Submission: Proof-of-Play Arena

## Verification
```yaml
github_username: "SlavaKudravcev"
discord_id: "957583229030920263"
timestamp: "2025-09-16"
```

## Developer
- **Name**: Slava
- **GitHub**: @SlavaKudravcev
- **Discord**: slavakudravcev#0246
- **Experience**: Experience in game development

## Project

### Name & Category
- **Project**: Proof-of-Play Arena
- **Category**: gaming

### Description
Proof-of-Play Arena is a gaming platform where player achievements are validated using Zero-Knowledge proofs. The system enables proving quests and achievements without exposing private data, while rewards and leaderboards remain transparent and verifiable.

### SL Integration  
Uses Soundness Layer to verify ZK proofs, integrated with the testnet to record results and issue rewards.

## Technical

### Architecture
- **Game Client Plugin** — collects data and generates proofs.
- **Verifier Service** — validates via Soundness Layer.
- **Leaderboard** — global player ranking.
- **NFT Rewards** — issues achievement rewards.
- **Discord Bot** — grants roles and badges.

### Stack
- **Frontend**: React + Tailwind
- **Backend**: Node.js (Express)
- **Blockchain**: Soundness Layer testnet
- **Storage**: PostgreSQL + IPFS

### Features
1. Local proof generation for achievements.
2. Verification via Soundness Layer.
3. NFT rewards.
4. Global leaderboards.
5. Discord integration.

## Timeline

### PoC (2-4 weeks)
- [ ] Basic prototype
- [ ] SL integration
- [ ] Simple leaderboard

### MVP (4-8 weeks)  
- [ ] NFT rewards
- [ ] Discord bot
- [ ] UI and documentation

## Innovation
The project’s uniqueness lies in portable achievements — players can prove progress across services without revealing sensitive data.

## Contact
**Discord**: slavakudravcev#0246

**Checklist before submitting:**
- [ ] All fields completed
- [ ] GitHub username matches PR author  
- [ ] SL integration explained
- [ ] Timeline is realistic
