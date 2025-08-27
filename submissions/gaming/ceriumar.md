# LootArena

## Category
gaming

## Project Title
LootArena: Verifiable On-Chain Loot Battles

## Summary
LootArena is a vApp where players engage in simple turn-based battles using NFT characters and verifiable random outcomes.  
Instead of relying on centralized RNG, the randomness and battle results are generated and proven via the Soundness Layer (SL). This ensures transparent, tamper-proof gameplay where every battle can be verified with a blob id.

## Architecture & SL Integration
- **Frontend**: lightweight web UI for players to connect wallets, select NFT character, and start a battle.  
- **Battle Engine**: computes outcomes (damage, critical hits, loot drops) using verifiable randomness.  
- **Soundness Layer (SL)**: generates proofs for each battle result and publishes them as blob ids.  
- **Verifier**: optional smart contract or explorer page where players can confirm that the battle result was fair.  

## MVP Scope
- Simple battle: 1v1 fight with 2–3 actions (attack, defend, special).  
- Integration with SL for randomness + result proof.  
- Demo page: player selects NFT → battle runs → result + blob id displayed.  

## Milestones & Timeline
- **Week 1–2**: Build basic battle simulation + connect SL for randomness proof.  
- **Week 3–4**: Publish results with blob ids, display on demo frontend.  
- **Week 5**: Optional — add loot drop system (e.g., NFT item reward) proven via SL.  

## Risks & Dependencies
- Proof generation latency could make battles feel slower.  
- Game logic needs to be deterministic for proofs to stay consistent.  
- Requires NFT metadata for characters.  

## Success Criteria
- Battles always produce a blob id proof.  
- Users can independently verify fairness of outcomes.  
- Demo shows fun, transparent game loop powered by Soundness Layer.  

## Contact
- **GitHub**: [ceriumar](https://github.com/ceriumar)  
- **Discord**: vitalikbuterin_
