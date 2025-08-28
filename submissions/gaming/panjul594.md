
vApp Submission: NFT TicTacToe

Verification  
github_username: "panjul594"  
discord_id: "1316123177914466378"  
timestamp: "2025-08-28"  

---

## Developer
**Name**: Panjul  
**GitHub**: @panjul594  
**Discord**: jullabruzzi  
**Experience**: experience building simple web apps (JS/TS), familiar with React, interested in learning smart contracts & cross-chain integration  

---

## Project
**Project**: NFT TicTacToe  
**Category**: gaming  

---

## Description
NFT TicTacToe is the classic 3x3 board game but cross-chain. Players can bring their NFTs (from any chain) as the X or O markers. The game is recorded on-chain, and results are verified through the Soundness Layer, ensuring fairness and immutability.  

---

## SL Integration
- **State Lens**: reads NFT ownership and metadata from source chains (ETH, Polygon, Solana, etc.)  
- **Unified Proof**: every move and the final result are validated once by the Soundness Layer  
- **Settlement**: the winner automatically receives small reward tokens (e.g., testnet tokens) without manual bridging  

---

## Technical Architecture
1. **Frontend**: TicTacToe board UI (React)  
2. **Game Engine**: simple game logic in Node.js  
3. **NFT Reader**: fetch NFT ownership data via State Lens  
4. **Game Result → Proof**: match results submitted to Soundness Layer for unified proof  
5. **Reward**: tokens automatically distributed to the winner’s wallet  

---

## Stack
- **Frontend**: React  
- **Backend**: Node.js (Typescript)  
- **Blockchain**: Soundness Layer + Ethereum + Polygon  
- **Storage**: IPFS for match metadata  

---

## Features
- simple NFT-based cross-chain TicTacToe  
- match results validated with zk proof  
- automatic cross-chain reward distribution  

---

## Timeline
**PoC (2–4 weeks)**  
- basic 3x3 board UI  
- integrate State Lens to check NFT ownership  
- record game moves to storage  

**MVP (4–8 weeks)**  
- Soundness Layer proof for match results  
- cross-chain reward distribution to the winner  
- open testnet for community play  

---

## Innovation
Unique because a very simple game (TicTacToe) directly showcases the power of cross-chain apps and unified proofs. Players can use NFTs from any chain as their markers, results are fair and transparent.  

---

## Contact
- **email**: suhadaparhan@gmail.com  
- **discord**: jullabruzzi  
- progress updates will be shared via GitHub repo & Discord dev thread  

---

### Checklist
- [x] all fields completed  
- [x] GitHub username matches PR author  
- [x] SL integration explained  
- [x] timeline is realistic