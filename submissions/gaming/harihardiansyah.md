# Project: zkQuest Arena

## Category
gaming

## Summary
zkQuest Arena is a web-based RPG game where players can prove their achievements (quests, levels, items) through zero-knowledge proofs without exposing their full data. By leveraging the Soundness Layer, we aim to deliver a fair, private, and verifiable on-chain gaming experience.

## Problem & Value
Most blockchain games today store player progress and inventories publicly, making them easy to copy or forge. This reduces fairness and compromises player privacy.  
Our solution: use verifiable compute via the Soundness Layer to validate player progress without revealing raw gameplay data.

## Architecture & Soundness Layer Integration
- **Frontend (React/Next.js)** → game UI for players.  
- **Off-chain Game Engine** → handles quest logic & inventory updates.  
- **Soundness Layer** → generates and verifies zk proofs for:
  - Confirming that a player completed a specific quest.  
  - Validating ownership of certain items without exposing the full inventory.  
- **Smart Contracts (EVM)** → record verified quest completions and distribute NFT/token rewards.  
- **On-chain Leaderboard** → displays only valid proofs from the SL, not raw data.

## Milestones & Timeline
- **M1 (Week 1–2):** Basic smart contracts + SL integration for a simple quest proof.  
- **M2 (Week 3–4):** Core UI (login, quest, leaderboard).  
- **M3 (Week 5):** zk proof integration for inventory + NFT reward distribution on testnet.  
- **M4 (Week 6):** Documentation & public demo for community testing.  

## Team
@harihardiansyah — game logic & smart contracts.  

## Discord
(1026452666274033694)
