# ZK Treasure Hunt

**Category:** gaming  
**Author:** [thebonz] 
**Discord ID:** [813956004010131466]

## Summary
ZK Treasure Hunt is a decentralized game where players explore a virtual map to find hidden treasures. Players can prove they found a treasure using zero-knowledge proofs without revealing the exact location, preserving game fairness and secrecy.

## Problem
Traditional online treasure hunt or map-based games expose positions or solutions once players verify their achievements. This leads to unfair advantages and spoils the experience for other players.

## Solution
Using the Soundness Layer, ZK Treasure Hunt allows:
- Players to generate a zk proof confirming they reached a correct treasure location based on the game’s puzzle rules.
- The proof verifies correctness without leaking the actual coordinates or moves.
- Game logic ensures players cannot cheat since proofs must satisfy cryptographic constraints tied to the original challenge hash.

## Technical Architecture
- **Game Engine:** A lightweight backend serving map challenges and puzzles.
- **Prover Script:** Uses Soundness CLI to create zk proof based on:
    - Player’s movement path.
    - Game map’s treasure position hash.
- **Verifier:** Runs on Soundness Layer or CLI to validate proof without revealing path or location.
- **Frontend:** Simple web or mobile UI for interaction, movement, and proof submission.

## Development Timeline
- Week 1: Define map format, treasure challenge, and proof circuit.
- Week 2: Implement prover logic and Soundness Layer integration.
- Week 3: Build movement simulation and treasure validation rules.
- Week 4: Create simple UI and leaderboard for verified hunters.
- Week 5: Testnet deployment and community challenge.

## Future Extensions
- Multi-player competitive treasure hunts with private proofs.
- NFT-based rewards for verified treasure finds.
- On-chain tournaments where only zk proofs determine winners.

