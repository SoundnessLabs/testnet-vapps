# vApp Submission: [Your Project Name]

## Verification
```yaml
github_username: "gusiolala"
discord_id: "443405952662896640"
timestamp: "2025-01-15"
```

## Developer
- **Name**: gusi
- **GitHub**: @gusiolala
- **Discord**: Gusioblag
- **Experience**: web3 explorer

## Project

### Name & Category
- **Project**: questhub
- **Category**: defi/gaming

### Description
GameFi QuestHub is a blockchain-based marketplace quest platform where game developers can create quests with token/NFT rewards.
Players complete quests, receive proof of completion via the Soundness Layer, and rewards are automatically awarded

### SL Integration  
Proof of Completion: A Soundness Layer is used to ensure players correctly complete quests without data manipulation.
Verifiable Rewards: Rewards are recorded on the blockchain, are transparent, and can be audited.
Identity Integration: SL can be used to prevent multi-account abuse (one player, one identity).

## Technical
Developers have difficulty ensuring that players correctly complete missions without cheating.
Rewards are often opaque or slow.

There is no centralized platform for the "quest economy" in the GameFi world.
### Architecture
High-level system design and approach

1. Frontend (Player & Developer App)
React + Next.js web app for players and developers.
Players: register, take quests, submit proof, claim rewards.
Developers: create quests, set rewards, monitor results.

2. Backend (Quest Management & Reward Engine)
Node.js + Express as API server.
Quest Management Service: CRUD quests, quest matching.
Reward Engine: trigger smart contracts for NFT/token distribution.
Leaderboard Service: update rankings and player stats.

3. Soundness Layer (SL Integration)
Proof Verification: validate quest completion (anti-cheat).
Identity Check: prevent multi-account abuse.
Audit Logs: all quest and reward activity can be verified.

4. Blockchain Layer
Smart Contracts (SL + Polygon): store quest contracts, reward distribution, and NFT minting.
Token & NFT contracts for quest rewards.

5. Storage Layer
WALRUS/IPFS: stores quest assets and proof of completion (screenshots, logs, etc.).
Database (Postgres/MongoDB): for off-chain metadata and caching.

Flow Overview:

Developer → create quest → save to blockchain.
Player → take quest → submit proof → SL validates.
If valid → Reward Engine triggers smart contract → reward automatically disbursed to wallet.
Leaderboard updates → results appear on the frontend and can be publicly verified.

### Stack
- **Frontend**: React + Next.js
- **Backend**: Node.js + Express  
- **Blockchain**: Soundness Layer + Polygon (for NFTs & reward tokens)
- **Storage**: WALRUS/IPFS (for quest assets & proof of completion)

### Features
1. Quest marketplace (developer → player).
2. Smart contract-based NFT/token auto-rewards.
3. Transparent global leaderboard on the blockchain.
4. Anti-cheat verification with Soundness

## Timeline

### PoC (2-4 weeks)
- User registration (developers & players).
- Developers can create simple quests.
- Basic SL proof integration.
- Simple UI.

### MVP (4-8 weeks)  
- Full quest marketplace.
- Automatic NFT/token rewards.
- Leaderboard & player stats.
- User testing (developer + player).

## Innovation
The first to combine a quest marketplace and play-to-earn platform with blockchain-proof anti-cheat.
Developers find it easier to attract players with open quests.
Players are assured of transparent and immediate rewards.

## Contact
Preferred contact method and where you'll share updates.


**Checklist before submitting:**
- [✓] All fields completed
- [✓] GitHub username matches PR author  
- [✓] SL integration explained
- [✓] Timeline is realistic
