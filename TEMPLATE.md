# vApp Submission: [Ahmad Syahroni Adventure]

## Verification
```yaml
github_username: "Wirya0101"
discord_id: "1108397218676871189"
timestamp: "2025-01-15"
```

## Developer
- **Name**: udin custom
- **GitHub**: @Wirya0101
- **Discord**: sbsquad
- **Experience**: gaming development

## Project

### Name & Category
- **Project**: Ahmad Syahroni Adventure
- **Category**: gaming

### Description
*The life of a proud man, continued. He collected a lot of NFTs — but he is greedy.*

### SL Integration

Ahmad Syahroni Adventure will integrate the Soundness Layer primarily for verifiable and private off-chain game logic.
- **ZK-Proofs for Game Events:** Important gameplay events (e.g., successful NFT hunts or rewards) will be computed off-chain and verified on-chain using ZK proofs, ensuring fairness and transparency without bloating the blockchain.
- **Private Player State:** Player stats, inventory, and progression will be stored off-chain but provably validated using Soundness Layer, preserving player privacy and improving scalability.
- **NFT Minting Logic:** Upon successful verification of in-game achievements via SL, NFTs will be minted as rewards using smart contracts deployed on the Soundness Layer.
This integration allows the game to remain scalable, cost-efficient, and tamper-proof while offering a fair gaming experience.

## Technical

### Architecture
**High-Level Design Overview:**

The game consists of three main components:

1. **Frontend (Client-side Game App):**
   - Built using HTML5, JavaScript, and Web3.js/ethers.js
   - Provides a browser-based adventure game interface
   - Interacts with wallet providers like MetaMask for authentication and signing

2. **Backend (Game Server & Logic):**
   - Node.js backend handles off-chain game logic (e.g., movement, enemy encounters, treasure generation)
   - Generates game outcomes which are converted to zero-knowledge proofs using the Soundness Layer SDK

3. **Smart Contract Layer (on Soundness Layer):**
   - Verifies the validity of ZK proofs for game actions (e.g., "player found rare item")
   - Mints NFTs as in-game rewards when proofs are valid
   - Stores minimal on-chain state to keep the game efficient

**Flow:**

1. Player logs in using crypto wallet
2. Player completes game actions → off-chain logic generates proof
3. Proof submitted to SL smart contract for validation
4. If valid → NFT minted to player’s wallet

This architecture balances decentralization, performance, and scalability while making use of Soundness Layer’s verifiability and ZK features.

### Stack
- **Frontend**: HTML5 + JavaScript + Web3.js (browser-based game UI)
- **Backend**: Node.js (handles off-chain game logic and SL proof generation)
- **Blockchain**: Soundness Layer (SL) for ZK-proof validation and NFT minting
- **Storage**: IPFS for storing NFT metadata and assets (images, descriptions), minimal use of database for game state caching (e.g., Redis)

### Features
1. **NFT Hunting Gameplay**  
   Players explore a digital world where they search for hidden treasures and earn NFTs by completing randomized quests and challenges.
2. **ZK-Proof Verified Rewards**  
   All in-game rewards (e.g., rare items, achievements) are earned through off-chain logic and verified using zero-knowledge proofs on Soundness Layer before minting NFTs.
3. **Greed-Based Risk System**  
   The more a player tries to claim rewards in a short time, the higher the risk of failure or penalties — encouraging strategic play and mimicking the theme of greed in the story.

## Timeline

**Week 1: Setup & Core Development**
- [x] Build basic gameplay logic (player movement, NFT hunt trigger)
- [x] Setup wallet authentication (e.g., MetaMask)
- [x] Integrate Soundness Layer SDK for proof generation

**Week 2: SL Integration & Demo**
- [x] Implement smart contract to verify ZK-proofs
- [x] Enable NFT minting upon verified gameplay events
- [x] Create minimal UI to showcase core loop: play → prove → mint
- [x] Internal testing and debugging

## Innovation
Ahmad Syahroni Adventure blends storytelling, NFTs, and verifiable gameplay in a way that's both humorous and strategic. Unlike typical NFT games that reward grinding, this game introduces a **"greed-based risk system"** — the more you try to win, the more likely you are to lose everything.

Key innovations:
- **Narrative-driven ZK game:** The game uses zero-knowledge proofs not just for fairness, but to drive gameplay itself. Every reward is earned and proven — no cheating, no bots.
- **Greed mechanics:** A unique system where being too greedy increases the chance of failure, forcing players to make calculated moves.
- **NFTs with meaning:** Every NFT collected reflects a moment of verified success in-game — they're not just items, but trophies of calculated risk.

By making greed both a tool and a trap, the game offers a fresh take on risk-reward dynamics, backed by verifiable tech.

## Contact
Preferred contact method and where you'll share updates.


**Checklist before submitting:**
- [x] All fields completed
- [x] GitHub username matches PR author  
- [x] SL integration explained
- [x] Timeline is realistic
