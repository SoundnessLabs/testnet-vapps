# vApp Submission: ZK Puzzle Games

## Verification
```yaml
github_username: "donydp07"
discord_id: "1283447991889297511"
timestamp: "2025-08-26""
```

## Developer
- **Name**: Dony Dwi P.
- **GitHub**: @donydp07
- **Discord**: donydp07#
- **Experience**: im 27th, from indonesia.

## Project

### Name & Category
- **Project**: ZK Puzzle Games
- **Category**:gaming

### Description
ZK Puzzle Games is a collection of simple but challenging mini-games powered by Zero-Knowledge proofs.
It introduces puzzles such as Sudoku ZK, Magic Square (3x3 / 4x4), and Knight’s Tour (Chess) where users can prove their solutions without revealing the entire process.
This makes learning ZK fun, engaging, and accessible.

### SL Integration  
Use Soundness Layer to verify zk-proofs for puzzle solutions.
Smart contract on SL testnet validates proofs and rewards users with points.
Daily/Weekly quests stored on SL to ensure fairness and immutability.

## Technical

### Architecture
Frontend handles puzzle generation and user interface.
ZK circuits (Sudoku, Magic Square, Knight’s Tour) written in Circom/Halo2.
Proofs generated client-side, then sent to backend.
Smart contract verifier deployed on Soundness Layer Testnet.
Database tracks scores, but final verification is on-chain.

### Stack
- **Frontend**: React + Tailwind
- **Backend**: Node.js + Express  
- **Blockchain**: Soundness Layer (testnet)
- **Storage**: PostgreSQL (for user progress), IPFS (for puzzle metadata)

### Features
1. Daily & Weekly puzzle challenges.
2. ZK verification of solutions on-chain.  
3. Point & reward system for active users.

## Timeline

### PoC (2-4 weeks)
- [ ] Sudoku ZK circuit + UI prototype
- [ ] SL contract for proof verification
- [ ] Basic UI & point tracking

### MVP (4-8 weeks)  
- [ ] Add Magic Square & Knight’s Tour
- [ ] Full integration with SL smart contracts
- [ ] User testing & bug fixes

## Innovation
Makes ZK proofs fun and interactive via puzzles.
Scalable difficulty → beginners & experts can join.
Bridges gaming + education + Web3 through SL integration.

## Contact
Discord: donydp07
X (Twitter): @mydony07


**Checklist before submitting:**
- [ ] All fields completed
- [ ] GitHub username matches PR author  
- [ ] SL integration explained
- [ ] Timeline is realistic
