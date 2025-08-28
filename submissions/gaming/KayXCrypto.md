# vApp Submission: 2048 On-chain

## Verification
```yaml
github_username: "KayXCrypto"
discord_id: "750006071523016754"
timestamp: "2025-08-28"
```

## Developer
- **Name**: Kay Nguyen
- **GitHub**: @KayXCrypto
- **Discord**: manhmeo0520#750006071523016754
- **Experience**: Brief background

## Project

### Name & Category
- **Project**: 2048 On-chain
- **Category**: gaming

### Description
2048 On-chain is a blockchain-powered version of the classic 2048 puzzle game. Players can enjoy the game on the web, while their scores and wins are recorded on-chain. Beyond entertainment, this project demonstrates how to integrate Soundness Layer for transparent, verifiable, and tamper-proof gameplay data.

### SL Integration  
Use Soundness Layer to record player high scores on-chain → transparent and resistant to cheating.

Integrate WALRUS storage for leaderboard data.

Optionally use zk-proofs (from SL) to verify that a player actually achieved a score without revealing the full game state.

## Technical

### Architecture
Player interacts with the game → frontend handles 2048 logic.

When a new score is achieved, the data is sent to backend → verified → written onto Soundness Layer.

Leaderboard is fetched from blockchain and displayed to all players.

### Stack
- **Frontend**: React + TailwindCSS
- **Backend**: Python (FastAPI or Flask)
- **Blockchain**: Soundness Layer (+ web3.py / project SDK if available)
- **Storage**: WALRUS (Python SDK) for leaderboard, IPFS (via ipfshttpclient) for assets

### Features
1.Play 2048 directly in the browser

2.On-chain score recording with Soundness Layer

3.Transparent, anti-cheat leaderboard

## Timeline

### PoC (2-4 weeks)
- [ ] Implement basic 2048 gameplay on web
- [ ] Integrate score submission to Soundness Layer
- [ ] Simple leaderboard UI

### MVP (4-8 weeks)  
- [ ] Full-featured UI/UX with animations
- [ ] Real-time leaderboard updates
- [ ] User testing with multiple players

## Innovation
Simple yet effective showcase of on-chain gaming.

Anti-cheat score verification via Soundness Layer.

Transparent leaderboard, extendable to tournaments, token rewards, or NFTs for top players.

## Contact
Discord: manhmeo0520#750006071523016754

Email/Twitter: nguyenducmanh2097@gmail.com


**Checklist before submitting:**
- [ ] All fields completed
- [ ] GitHub username matches PR author  
- [ ] SL integration explained
- [ ] Timeline is realistic
