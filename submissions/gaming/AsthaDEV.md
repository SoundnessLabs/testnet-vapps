vApp Submission: Sound Tap (Click Challenge)

Verification  
github_username: "ashta089"  
discord_id: "1194622040318148708"  
timestamp: "2025-08-29"  

---

## Developer
Name: AsthaDEV  
GitHub: @ashta089 
Discord: bongseng02#0 
Experience: Basic web development (HTML/CSS/JS). Interested in building verifiable mini-games on testnet to explore the Soundness Layer.  

---

## Project Name & Category
Project: Sound Tap (Click Challenge)  
Category: gaming  

---

## Description
Sound Tap is a **fast click challenge** game: players must click a button as many times as possible within 10 seconds.  
Problem solved: traditional clicker games are vulnerable to cheating (scripts/auto-clickers). By integrating the **Soundness Layer (SL)**, game results are validated through cryptographic proofs, making the competition transparent, fair, and tamper-proof.  

---

## SL Integration
- **Commit–Reveal**: each click is stored in batched hashes, preventing manipulation after the game.  
- **Unified Proof**: the final click count is proven via SL, enabling public verification.  
- **Settlement**: optional—winners in tournaments automatically receive testnet tokens if the proof is valid.  

---

## Technical Architecture
1. **Frontend**: Clicker UI with timer + button.  
2. **Game Engine**: counts clicks per session, batches them into hashes.  
3. **Proof Generation**: after the game, results + batch hashes are sent to backend for witness creation.  
4. **SL Proof**: witness is verified on SL → unified proof of final score is generated.  
5. **Leaderboard/Settlement**: proof is stored on IPFS/Walrus, and rewards distributed via smart contract.  

---

## Stack
- **Frontend**: React + Tailwind  
- **Backend**: Node.js (Express/Fastify)  
- **Blockchain**: Soundness Layer (proofs), optional Sui for rewards  
- **Storage**: IPFS/Walrus  

---

## Features
- Core feature 1: Click as many times as possible within 10 seconds.  
- Core feature 2: Game results are validated with proofs on SL.  
- Core feature 3: Proof-based leaderboard (immutable & tamper-proof).  

---

## Timeline
**PoC (2–3 weeks)**  
- Basic clicker UI.  
- Store results with simple commit–reveal.  
- Publish metadata to IPFS.  

**MVP (4–6 weeks)**  
- SL integration with unified proof for final scores.  
- Proof-based leaderboard.  
- Optional smart contract for testnet rewards.  

---

## Innovation
Clicker games are usually just casual fun. **Sound Tap** transforms it into a **verifiable game**, where every score can be cryptographically audited. This makes tournaments and leaderboards fully transparent, cheat-resistant, and an excellent onboarding experience for newcomers into verifiable gaming with the Soundness Layer.  

---

## Contact
Discord: bongseng02#0  
Updates: via GitHub repo + developer Discord channel  

---

## Checklist before submitting
- [x] All fields completed  
- [x] GitHub username matches PR author  
- [x] SL integration explained  
- [x] Timeline is realistic  
