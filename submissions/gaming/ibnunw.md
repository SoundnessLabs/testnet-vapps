# vApp Submission: zkPuzzleSolver

## Verification
```yaml
github_username: "ibnunw"
discord_id: "815144001753317377"
timestamp: "2025-08-21"
```

## Developer
- **Name**: Sinunu
- **GitHub**: @ibnunw
- **Discord**: sinunu182
- **Experience**: Web developer with experience in game development and puzzle games. Basic knowledge of zero-knowledge applications.

## Project

### Name & Category
- **Project**: zkPuzzleSolver
- **Category**: gaming

### Description
A puzzle game where players solve logical puzzles and generate zero-knowledge proofs to verify their solutions without revealing the answers. Players can challenge friends to solve puzzles and verify solutions trustlessly using zk-proofs.

### SL Integration  
Using Soundness Layer for puzzle verification:
1. Generate zk-proofs for correct puzzle solutions
2. Verify solution proofs without revealing answers
3. Use SL's proof system for puzzle verification circuits
4. On-chain verification for puzzle competitions

## Technical

### Architecture
Web Game Interface → Puzzle Solving → Proof Generation (SL) → Verification → Score Tracking

### Stack
- **Frontend**: Phaser.js (HTML5 game framework)
- **Backend**: Node.js for basic API
- **Blockchain**: Soundness Layer (proof generation/verification)
- **Storage**: Local storage for player progress

### Features
1. Multiple logical puzzle types (grid-based, number puzzles)
2. zk-proof generation for completed puzzles
3. Solution verification without revealing answers
4. Puzzle difficulty progression
5. Challenge friends with proof verification

## Timeline

### PoC (3 weeks)
- [ ] Basic puzzle game interface
- [ ] First puzzle type implementation
- [ ] SL integration for proof generation
- [ ] Local proof verification

### MVP (5 weeks)
- [ ] Multiple puzzle types
- [ ] Difficulty progression system
- [ ] Friend challenge system
- [ ] Basic leaderboard
- [ ] Enhanced UI/UX

## Innovation
Brings zero-knowledge proofs to casual gaming, allowing players to prove puzzle solutions without spoilers. Creates new possibilities for competitive puzzle solving with verifiable but private solutions.

## Contact
Discord: sinunu182. Will share progress in Soundness Discord and maintain GitHub repository with regular updates.

**Checklist before submitting:**
- [x] All fields completed
- [x] GitHub username matches PR author
- [x] SL integration explained
- [x] Timeline is realistic
