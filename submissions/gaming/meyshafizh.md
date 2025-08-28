---
title: sound-cryptogame
category: gaming
githubUsername: meyshafizh
discordId: hafizhabsi
---

## Description
**sound-cryptogame** is a blockchain-based mini gaming hub inspired by platforms like crypto.games.  
Players can enjoy simple betting games such as dice, coin flip, and roulette — all powered by the **Soundness Layer** to guarantee fairness and verifiable randomness.

## Technical Architecture
- **Frontend**: React web app with a minimal casino-like UI.  
- **Smart Contracts**: Deployed on Sui testnet to handle bets, payouts, and player balances.  
- **RNG & Proofs**: All randomness is generated inside **SP1 zkVM**, producing proofs verified by Soundness Layer.  
- **Soundness Layer**:  
  - Verifies zkProofs for all game outcomes.  
  - Publishes results and ensures data availability via WALRUS.  

**Flow:**
1. Player selects a game (dice, coin flip, roulette).  
2. A bet transaction is sent to the smart contract.  
3. RNG + zkProof are generated inside SP1 zkVM.  
4. Soundness Layer verifies the proof and publishes the outcome.  
5. Smart contract processes win/loss and updates balances.  

## Development Timeline
| Phase            | Duration  | Milestone                                 |
|------------------|-----------|-------------------------------------------|
| Proposal PR      | 1 day     | Fork repo, submit proposal                |
| UI Prototype     | 1 week    | Basic React UI for multiple mini-games    |
| Smart Contracts  | 2 weeks   | Deploy contracts for dice/coin/roulette   |
| zk Integration   | 2 weeks   | RNG + proof with SP1 zkVM                 |
| SL Integration   | 1 week    | Full SL verification & WALRUS storage     |
| Testing & PoC    | 1 week    | End-to-end testnet demo                   |

## Soundness Layer Integration
- RNG proofs generated via SP1 zkVM.  
- Proofs + outputs stored in WALRUS for availability.  
- Contracts accept only outcomes verified by Soundness Layer.  

## Goal
Deliver a **provably fair multi-game platform** demonstrating how Soundness Layer can power secure, transparent, and fun blockchain-based betting experiences.
