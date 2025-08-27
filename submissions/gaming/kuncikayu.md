# vApp Submission: ZK Defender

## Verification
```yaml
github_username: "kuncikayu"
discord_id: "802782677903081492"
timestamp: "2025-08-27"
```

## Developer
- **Name**: Keywood
- **GitHub**: @kuncikayu
- **Discord**: kuncikayu_
- **Experience**: I'm Node & Validator Runner, having some experience about creating contract and dApps on EVM.

## Project

### Name & Category
- **Project**: ZK Defender
- **Category**: gaming

### Description
What problem does your vApp solve? What does it do?
Answer:
ZK Defender is an educational game that teaches players about zero-knowledge proofs (ZKP), data availability, and cross-chain interoperability through interactive puzzles and simulations. It addresses the challenge of understanding complex blockchain concepts by:

1. ZKP Mechanisms: Players prove knowledge of a secret without revealing it (e.g., solving a puzzle to unlock a vault).
2. Data Availability: Simulates decentralized storage using erasure coding (e.g., splitting data into fragments for redundancy).
3. Cross-Chain Integration: Players bridge transactions between Sui (sequencing) and Walrus (storage) to complete missions.

### SL Integration  
How will you use Soundness Layer? What specific SL features?
Answer:
1. Zero-Knowledge Proofs (ZKP):
- Used in puzzles to verify player answers without exposing the secret pattern.
- Example: Players prove they know a password without revealing it.
2. Data Availability via Walrus:
- Simulates decentralized storage with erasure coding to protect data from "Data Shadows."
3. Cross-Chain Bridge:
- Integrates Sui (sequencing) and Walrus (storage) to demonstrate cross-chain collaboration.

## Technical

### Architecture
High-level system design and approach

### Stack
- **Frontend**: React.js (interactive UI)
- **Backend**: Node.js + Express (game logic and API) 
- **Blockchain**: 
    - Soundness Layer: For ZKP verification and data availability.
    - Sui: For transaction sequencing and real-time actions.
    - Walrus: For decentralized storage simulations.
- **Storage**: Walrus

### Features
1. ZKP Puzzle:
    - Players solve cryptographic puzzles to prove knowledge without revealing secrets.
2. Data Availability Challenge:
    - Players store data fragments using erasure coding to ensure redundancy.
3. Cross-Chain Bridge:
    - Players connect Sui and Walrus to complete missions (e.g., verifying transactions).

## Timeline

### PoC (2-4 weeks)
- [ ] Basic functionality (ZKP puzzle + simple UI)
- [ ] SL integration (ZKP and data availability simulations)
- [ ] Simple UI (HTML/CSS/JavaScript)

### MVP (4-8 weeks)  
- [ ] Full features (3 levels: ZKP, Data Availability, Cross-Chain)
- [ ] Production-ready (React + Node.js backend)
- [ ] User testing (beta release)

## Innovation
What makes this unique? Why will people use it?
Answer:
1. Interactive Education: Teaches blockchain concepts (ZKP, data availability) through gameplay.
2. Real-World Tech Simulation: Uses Soundness Layer, Sui, and Walrus to mimic real blockchain infrastructure.
3. Play-to-Learn: Players earn virtual tokens ($SND) for completing challenges, incentivizing learning.

## Contact
Discord: https://discord.gg/user/802782677903081492
X (Twitter): https://x.com/kuncikayu_
Telegrm: https://t.me/KENW00D23


**Checklist before submitting:**
- [✅] All fields completed
- [✅] GitHub username matches PR author  
- [✅] SL integration explained
- [✅] Timeline is realistic
