# vApp Submission: [Your Project Name]

## Verification
```yaml
github_username: "rawakinode"
discord_id: "648020496192897045"
timestamp: "2025-08-22"
```

## Developer
- **Name**: Wahyu
- **GitHub**: @rawakinode
- **Discord**: Rawakinode#2016
- **Experience**: Experienced in building several web3 dapps (web3 games, blockchain dashboards, and on-chain API integrations).

## Project

### Name & Category
- **Project**: Sound Shooter
- **Category**: gaming

### Description
Sound Shooter is a web3-based rhythm shooting game where players shoot targets in sync with music beats. The game merges sound with interactive gameplay, where every shot is triggered by rhythm or sound input.
Problem solved: most on-chain games rely on basic click/interaction mechanics, while Sound Shooter introduces a new approach by combining music-driven gameplay with blockchain to create a unique, verifiable gaming experience.

### SL Integration  
Sound Shooter will leverage Soundness Layer (SL) for:

- Score verification: all high scores are validated and stored via SL to prevent manipulation.
- On-chain proof: key in-game events (level clears, achievements, competitions) are recorded through SL for transparency.
- Identity & leaderboard: decentralized identity integration with SL ensures a global leaderboard that is fair, secure, and tamper-proof.

## Technical

### Architecture
- Client: Frontend handles gameplay (beat detection, music input, shooting mechanics) and submits results to backend.
- Backend: Processes game results, connects to Soundness Layer for verification, and manages leaderboard data.
- Blockchain Integration: Backend commits proof of scores and achievements to SL, ensuring integrity and immutability.
- Leaderboard: Displayed in real time on frontend using on-chain verified data.

### Stack
- **Frontend**: React + Phaser (2D rhythm-based gameplay)
- **Backend**: Express JS, MongoDB
- **Blockchain**: Soundness Layer (SL) + EVM-compatible testnet
- **Storage**: WALRUS

### Features
1. Rhythm-based gameplay.
2. On-chain leaderboard.
3. Music integration.

## Timeline

### PoC (2-4 weeks)
- [x] Basic functionality
- [x] SL integration
- [x] Simple UI

### MVP (4-8 weeks)  
- [ ] Full features
- [x] Production ready
- [x] User testing

## Innovation
Sound Shooter is unique because it combines music rhythm mechanics with on-chain verifiable gameplay. Instead of just clicking or tapping, players engage with the beat of the music, turning every game session into a personal rhythm-driven experience.

- Brings sound + blockchain fusion rarely seen in web3 gaming.
- Ensures trustless leaderboards with verifiable scores.
- Appeals both to gamers and music enthusiasts.

## Contact
Shared via GitHub repo or Discord community.


**Checklist before submitting:**
- [x] All fields completed
- [x] GitHub username matches PR author  
- [x] SL integration explained
- [x] Timeline is realistic
