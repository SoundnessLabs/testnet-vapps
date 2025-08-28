# vApp Submission: [Decentralized PvP Questing Game]

## Verification
```yaml
github_username: "Aslaann"
discord_id: "928720550266806282"
timestamp: "2025-01-15"
```

## Developer
- **Name**: Aslaann
- **GitHub**: Aslaann
- **Discord**: jrann#5095
- **Experience**: Currently learning and exploring web development and blockchain technologies. Gaining hands-on experience through building small projects, contributing to testnets, and experimenting with tools like Node.js, Docker, and GitHub. Excited to keep improving and growing as a developer.

## Project

### Name & Category
- **Project**: Decentralized PvP Questing Game
- **Category**: Gaming

### Description
What problem does your vApp solve? What does it do?
QuestArena is a player-versus-player questing game built on top of the Soundness Layer. Players create characters, complete quests, and challenge each other in competitive battles. Progress such as levels, achievements, and unique items is recorded on-chain, while zero-knowledge proofs ensure privacy and fairness. The goal is to combine engaging gameplay with verifiable, trustless mechanics that make every action meaningful and transparent.


### SL Integration  
How will you use Soundness Layer? What specific SL features?
1. Quest Validation – Each completed quest generates an intent, which the Soundness Layer validates to confirm legitimate completion.

2. PvP Battles – Outcomes of battles (damage rolls, critical hits, etc.) are proven using ZK proofs, ensuring fairness without exposing internal calculations.

3. On-chain Assets – Unique rewards (items, skins, weapons) are minted as NFTs on the Soundness Layer, providing verifiable ownership.

4. Privacy – Player stats and strategies can be protected using ZK proofs, while still enabling public verification of results.

## Technical

### Architecture
1. Client (Game UI): A browser-based game built with React + Phaser, handling user interactions, animations, and wallet integration.

2. Game Logic: Smart contracts manage quests, battle logic, and NFT item distribution.

3. Relayer/Backend: A lightweight Node.js service for matchmaking and off-chain computation where appropriate.

4. Blockchain Integration: Soundness Layer validates intents, ZK proofs, and manages NFT rewards.

5. Storage:

   a.)On-chain: character profiles, achievements, NFT items.

   b.)Off-chain: large media assets (sprites, animations, lore) via IPFS/Arweave.

### Stack
- **Frontend**: React + Phaser
- **Backend**: Node.js (with optional Rust modules for performance-critical game logic) 
- **Blockchain**: Soundness Layer (SL) as core execution + validation
- **Storage**: IPFS/Arweave for assets; on-chain state for game progress

### Features
1. Quest System – Players complete on-chain quests for rewards.

2. PvP Arena – Competitive battles with outcomes verified via ZK proofs.

3. NFT Rewards – Unique in-game items and achievements represented as NFTs.

4. Leaderboard – On-chain ranking for competitive play.

5. Community Expansion – Future ability for community to propose new quests or events.


## Timeline

### PoC (2-4 weeks)
- [ ] Character creation and inventory basics
- [ ] Quest system with SL intent validation
- [ ] Simple battle mechanic (turn-based PvP)
- [ ] Minimal web interface

### MVP (4-8 weeks)  
- [ ] Advanced PvP mechanics with ZK fairness proofs
- [ ] Full quest and reward cycle on-chain
- [ ] NFT minting for achievements/items
- [ ] Leaderboard integration
- [ ] User testing + community feedback
## Innovation
What makes this unique? Why will people use it?
QuestArena stands out by blending fun gameplay with trustless mechanics. Unlike traditional blockchain games that focus heavily on tokenomics, QuestArena emphasizes fairness, verifiability, and real community-driven content. Zero-knowledge proofs add a unique layer of transparency: battles are provably fair without revealing sensitive in-game decisions.

## Contact
1. Preferred Contact: Discord (jrann#5095)

2. Updates: Progress reports will be shared via GitHub repo (fork of testnet-vapps) and community updates on Discord.


**Checklist before submitting:**
- [x] All fields completed
- [x] GitHub username matches PR author  
- [x] SL integration explained
- [x] Timeline is realistic
