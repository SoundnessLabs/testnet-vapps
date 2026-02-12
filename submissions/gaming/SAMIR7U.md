# vApp Submission: Soundness

## Verification
```yaml
github_username: "SAMIR7U"
discord_id: "1215004139939958817"
timestamp: "2025-08-31"
Developer

Name: Samir Kumar

GitHub: @SAMIR7U

Discord: 1215004139939958817

Experience: Beginner web3/game dev exploring ZK apps, familiar with frontend development and smart contract basics.

Project
Name & Category

Project: Soundness

Category: gaming

Description

Soundness is a blockchain-integrated puzzle/quest game.
Players complete challenges and submit zero-knowledge proofs (via Soundness Layer) instead of sending raw data. This prevents cheating, ensures fairness, and makes achievements verifiable on-chain. Players earn NFT-style rewards for proof-verified completions.

SL Integration

Use Soundness Layer to generate and verify ZK proofs when a player completes a quest.

Proof is validated on the SL testnet before awarding rewards.

Achievements (badges) are stored as NFTs once proofs are verified.

Technical
Architecture

Frontend (game) → generates game result.

Proof Generator → uses SL SDK to create ZK proof.

Soundness Layer Testnet → validates the proof.

On-chain storage → stores achievements as NFTs.

Stack

Frontend: React + simple puzzle/quest UI

Backend: Node.js for game logic & integration

Blockchain: Soundness Layer testnet

Storage: On-chain metadata + optional IPFS for assets

Features

Puzzle/quest game with ZK proof validation

On-chain verified achievements (badges)

Fair leaderboard (no cheating possible)

Timeline
PoC (2-4 weeks)

 Build simple puzzle mini-game

 Connect to Soundness Layer SDK

 Generate proof for quest completion

MVP (4-8 weeks)

 NFT badge system for achievements

 Leaderboard with proof verification

 Testing + small community playtest

Innovation

Unlike regular web3 games, Soundness ensures fairness with zero-knowledge proofs—no fake scores or cheating. All progress is verifiable on-chain, making gaming achievements truly trustless and permanent.

Contact

Discord: Samir2134

Updates: GitHub repo + Discord thread
  
