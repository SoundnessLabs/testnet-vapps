VApp Proposal – Rock Paper Scissors (RPS) on Soundness

Overview

Rock Paper Scissors (RPS) is a simple and globally recognized game, yet it provides a perfect use case to demonstrate the power of Soundness layer.
Players commit their moves secretly, reveal them simultaneously, and the result is verified using Soundness proofs.
This eliminates the possibility of cheating, ensures fairness, and showcases how zero-knowledge proofs can enhance even the simplest games.

Why Build on Soundness?

Commit-Reveal Fairness: Moves are hidden until both players reveal. Soundness ensures neither side can alter their choice after committing.
Provable Randomness: In single-player vs bot mode, Soundness provides verifiable randomness for the bot’s move.
Developer-Friendly: Simple rules but demonstrates key Soundness features (proof generation, verification, privacy).
Engagement: Universally understood game → great entry point for community members testing Soundness.

Minimum Viable Product (MVP)
1. Game Setup: Player chooses Rock, Paper, or Scissors.
2. Commit Phase: Player’s move is hashed/committed.
3. Reveal Phase: Both moves are revealed simultaneously.
4. Verification: Soundness validates the moves and determines the winner (Win / Lose / Draw).

Future Extensions
Multiplayer Mode: Challenge another wallet address with proof-backed gameplay.
Leaderboards: Track win rates, displayed publicly with verified outcomes.
Tokenized Matches: Small stake matches (testnet tokens) where results are provable on-chain.
Tournaments: Host Soundness-powered competitions to drive community engagement.

Impact

Although simple, RPS highlights the unique value of Soundness: provable fairness, anti-cheating mechanisms, and practical zero-knowledge applications.
This project can act as an onboarding showcase, encouraging developers to build more complex games and dApps on Soundness layer.
