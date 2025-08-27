github_username: "renxcz"
discord_id: "986937712756027432"
timestamp: "2025-08-28"

# vApp Proposal: Escape Room ZK Challenge

## Overview
Escape Room ZK Challenge is a puzzle-based game where players must solve logic riddles and hidden key combinations inside a virtual room. To unlock each stage, they need to provide a **zero-knowledge proof (ZKP)** that they know the correct solution, without revealing the actual answer.

## Motivation
Many puzzle and escape room games rely on trust between players and game hosts. With zero-knowledge proofs, the system can verify that a player solved the puzzle **without exposing the secret solution**. This keeps the game fair, competitive, and resistant to cheating.

## Features
- Multiple levels of escape room puzzles (math, logic, cipher).
- Zero-Knowledge verification for puzzle completion.
- On-chain rewards for completing rooms under time limits.
- Multiplayer mode: compete with others in solving puzzles.

## Technical Details
- **Program**: Implemented in WASM.
- **Proving System**: Ligetron.
- **Payload Structure**:
  - Puzzle input (public).
  - Player’s secret answer (private).
  - Proof that the answer unlocks the puzzle.

## Example Use Case
1. Player receives an encrypted puzzle clue.
2. Player finds the secret answer (e.g., `secret_key_123`).
3. Player submits a ZKP that they know `secret_key_123` unlocks the puzzle.
4. The system verifies proof and grants access to the next room.

## Value
- Brings **Web3 + ZKP** into gaming.
- Fun, engaging, and provably fair.
- Opens possibilities for esports tournaments with transparent proof of skill.

---
