# vApp Submission: [Kite Battle]

## Verification
```yaml
github_username: "taritoro"
discord_id: "1091220736338837564"
timestamp: "2025-08-31"
```

## Developer
- **Name**: Tororo
- **GitHub**: @taritoro
- **Discord**: tari_toro
- **Experience**: Beginner developer exploring blockchain gaming with Soundness Layer. Previously experimented with testnet projects and smart contract basics.

## Project

### Name & Category
- **Project**: Kite Battle
- **Category**: gaming

### Description
Kite Battle is a simple yet fun game inspired by traditional kite fighting. Players launch virtual kites, try to cut their opponents’ kite strings, and aim to be the last kite flying. The vApp brings nostalgic real-world gameplay into a decentralized environment where results are transparent and verifiable on-chain.

### SL Integration  
Kite Battle uses Soundness Layer for:

Verifying kite battles through on-chain randomness and fairness.
Recording match results and leaderboard scores on the blockchain.
Enabling players to create or join matches securely without a centralized server.

## Technical

### Architecture
Player actions (attack/defend) are signed and sent to SL smart contracts.
SL handles randomness (e.g., wind effects) and battle outcome logic.
Game state and match history are stored on-chain.
A lightweight frontend connects to SL and displays the game UI.

### Stack
- **Frontend**: React
- **Backend**: Node.js (for API helpers, if needed)  
- **Blockchain**: Soundness Layer (main logic and state)
- **Storage**: On-chain storage via SL, WALRUS/IPFS for static assets (optional)

### Features
1. Create or join a kite battle room.
2. On-chain battle logic with randomness (wind + attack/defense).  
3. Leaderboard showing top players and their win streaks.

## Timeline

### PoC (2-4 weeks)
- [ ] Basic functionality (kite launching + cutting mechanic)
- [ ] SL integration for match result verification
- [ ] Simple web UI with battle animation

### MVP (4-8 weeks)  
- [ ] Full battle features (attack, defense, randomness)
- [ ] Production-ready deployment on SL testnet
- [ ] Leaderboard and user testing with community

## Innovation
Kite Battle is unique because it combines cultural nostalgia with decentralized gaming. Instead of a centralized server deciding outcomes, every battle is validated on-chain, ensuring fairness and transparency. The lightweight mechanics make it easy for anyone to try, while still showcasing how games can run trustlessly on the Soundness Layer.

## Contact
Discord: tari_toro
GitHub: taritoro/testnet-vapps
Updates will be shared via Discord and GitHub.
