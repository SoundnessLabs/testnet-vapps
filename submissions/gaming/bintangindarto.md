
# vApp Proposal: DrownGuard Arena

github_username: "bintangIndarto"
discord_id: "848584595951059014"
timestamp: "2025-08-30"

---

## Overview
DrownGuard Arena is a browser-based PvP game where players control characters fighting in an arena filled with water obstacles.
The main concept: each character has a stamina meter and an oxygen meter. If a player runs out of oxygen, they must surface or lose.

The game utilizes zkProofs' Soundness Layer to:
- Verifiably prove match results (without manipulation).
- Efficiently store scores and rankings on-chain with blob storage.
- Provide on-chain rewards for top players on the leaderboard.

---

## Category
`gaming`

---

## Technical Architecture
- Frontend: React + Phaser.js (2D game engine).
- Backend: Node.js + Express.
- **Soundness Layer integration:**
- ZkProof is used to validate fairness (game result = final state hash + player signature).
- Leaderboards and rewards are stored via the testnet's Soundness Layer.
- Blob storage is used to store match replays (compressed).

Simple architecture diagram:


---

## Development Timeline
- **Week 1:** Basic game setup (movement, stamina/oxygen system).
- **Week 2:** Integration with Soundness CLI for proof of submission.
- **Week 3:** Implement a leaderboard with Soundness Layer.
- **Week 4:** Deploy a PoC + simple multiplayer test.

---

## Why Soundness Layer?
- **Fair play:** Match results cannot be manipulated (guaranteed with zkProofs).
- **Scalable:** Match replays can be stored in blob storage, not the full chain → cost-effective.
- **Fun + utility:** Combination of gaming and Web3 infrastructure, can attract other developers to experiment.

---

## Expected Outcome
- Minimal playable demo (PoC) with a leaderboard connected to Soundness Layer.
- Proof that games can be a unique use case on Soundness Layer, not just a typical financial dApp.    

