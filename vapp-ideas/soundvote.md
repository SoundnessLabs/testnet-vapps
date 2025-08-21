# SoundVote – Voting dApp on Soundness Layer

## Author
GitHub: @dinhtruyenhcm  
Discord: yourdiscord#1234  

## Idea
SoundVote is a decentralized voting application that leverages Soundness proofs to ensure fair, tamper-proof, and anonymous voting.  

## Problem
Traditional on-chain voting can be manipulated, costly (gas), and lacks strong privacy for voters.

## Solution
- Use Soundness Layer to verify that each vote is valid without revealing voter identity.  
- Each wallet can only cast 1 vote.  
- Votes are aggregated and verified on-chain.  

## Benefits
- Transparent results (anyone can verify).  
- Privacy-preserving voting.  
- Practical for DAOs, governance, and community polls.

## Next Steps
- Implement a smart contract for poll creation and vote casting.  
- Integrate Soundness CLI for proof generation.  
- Build simple frontend (React + Wagmi) for user interaction.
