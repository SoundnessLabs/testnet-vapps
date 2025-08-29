# Project Proposal: zk-Tic Tac Toe

**Author:** folkart00
**Discord:** anjobigin  
**Category:** Gaming / Demo  

---

## Summary
zk-Tic Tac Toe is a classic 3x3 board game reimagined with Zero-Knowledge Proofs (ZKPs).  
Each move is verified using the Soundness Layer to ensure fairness and prevent cheating.  
Players can prove that their moves follow the rules without revealing hidden strategies, demonstrating the value of ZKPs in casual gaming.

---

## How It Works
1. Two players start a standard Tic Tac Toe match in the browser.  
2. Each move is submitted through the Soundness CLI, generating a proof that the move is valid (correct cell, player’s turn, not overwriting existing moves).  
3. Proofs are uploaded to Walrus (Blob ID) and verified by the Soundness Layer.  
4. Once a game ends, the winner’s proof is verifiable on-chain or off-chain, ensuring integrity of the match.  

---

## Future Extensions
- Leaderboard integration to track verified matches  
- NFT badges for winners  
- Support for online matchmaking using Soundness proofs  

---

## Motivation
Tic Tac Toe is universally understood and simple to play, making it an ideal demo to showcase the Soundness Layer.  
By applying ZKPs to a familiar game, this project lowers the barrier to understanding complex cryptographic proofs while highlighting their practical applications.
