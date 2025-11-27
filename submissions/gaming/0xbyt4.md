# vApp Submission: ZK Sudoku Mini

## Verification
- **GitHub Username**: 0xbyt4
- **Discord ID**: 399489194839506945
- **Submission Date**: 2025-11-27

---

## Developer Information

### About You
- **Name/Alias**: 0xbyt4
- **GitHub**: https://github.com/0xbyt4
- **Discord**: xeni0381
- **Background**: Web3 developer with experience in blockchain development, smart contracts, and zero-knowledge applications.

---

## Project Details

### Project Name
**ZK Sudoku Mini** (4x4 Sudoku with Zero-Knowledge Proofs)

### Category
**Gaming**

### Problem Statement
Current Soundness Layer testnet games (8 Queens, Knights Tour) are complex and time-consuming. There's a need for a quick, simple game that:
- Can be completed in under 1 minute
- Is universally known and easy to understand
- Introduces new users to ZK proofs without intimidation
- Provides variety in the gaming experience

### Solution
A 4x4 Mini Sudoku game that generates ZK proofs upon completion. The simplified grid (16 cells vs 81 in standard Sudoku) makes it:
- Fast to solve (~30 seconds)
- Easy to verify with ZK
- Accessible to all skill levels
- Perfect for demonstrating ZK concepts

---

## Soundness Layer Integration

### How will you use Soundness Layer?
1. **Proof Generation**: Use Ligetron (client-side WebGPU prover) to generate ZK proofs of valid Sudoku solutions
2. **Proof Storage**: Store proofs on Walrus via Blob ID
3. **Verification**: Submit proofs to Soundness Layer for attestation
4. **On-chain Record**: Attestations stored on Sui blockchain

### Which SL features will you leverage?
- **Ligetron Prover**: Client-side ZK proof generation (WebGPU accelerated)
- **Walrus Integration**: Decentralized proof storage
- **Soundness CLI**: Proof submission (`soundness-cli send`)
- **Attestation System**: On-chain verification records

### Integration Flow
```
User Solves Puzzle → Ligetron Generates Proof → Walrus Stores Proof
                                                      ↓
                                              Blob ID Returned
                                                      ↓
                              Soundness CLI Submits → Attestation on Sui
```

---

## Technical Architecture

### System Overview
```
┌─────────────────────────────────────────────────────────────┐
│                      FRONTEND (Next.js)                      │
├─────────────────────────────────────────────────────────────┤
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────┐  │
│  │ Sudoku Grid │  │   Timer     │  │  Proof Status       │  │
│  │   (4x4)     │  │             │  │  [Generate] [Submit]│  │
│  └─────────────┘  └─────────────┘  └─────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                    ZK CIRCUIT (Ligetron)                     │
├─────────────────────────────────────────────────────────────┤
│  Input: solution[16], seed, public_key                       │
│  Constraints:                                                │
│    - Each cell ∈ {1,2,3,4}                                  │
│    - Each row contains {1,2,3,4} exactly once               │
│    - Each column contains {1,2,3,4} exactly once            │
│    - Each 2x2 box contains {1,2,3,4} exactly once           │
│  Output: SHA256 hash commitment                              │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                    SOUNDNESS LAYER                           │
├─────────────────────────────────────────────────────────────┤
│  Walrus (Storage) → Soundness CLI → Sui (Attestation)       │
└─────────────────────────────────────────────────────────────┘
```

### Tech Stack

| Layer | Technology | Purpose |
|-------|------------|---------|
| **Frontend** | Next.js 14, React, TypeScript, TailwindCSS | User interface |
| **Game Logic** | TypeScript | Puzzle generation, validation |
| **ZK Circuit** | C++ (Ligetron) | Zero-knowledge proof generation |
| **Prover** | Ligetron (WebGPU) | Client-side proof computation |
| **Storage** | Walrus | Decentralized proof storage |
| **Blockchain** | Sui | Attestation records |
| **CLI** | Soundness CLI | Proof submission |

### Core Features

1. **Puzzle Generator**
   - Generates valid 4x4 Sudoku puzzles
   - Adjustable difficulty (number of pre-filled cells)
   - Ensures unique solution

2. **Interactive Game UI**
   - Clean, responsive 4x4 grid
   - Real-time validation feedback
   - Timer for speedrun tracking
   - Mobile-friendly design

3. **ZK Proof Integration**
   - One-click proof generation
   - Progress indicator during computation
   - Blob ID display after upload
   - CLI command generator for submission

---

## Development Timeline

### Phase 1: Proof of Concept (2-3 weeks)

| Week | Milestone | Deliverables |
|------|-----------|--------------|
| 1 | ZK Circuit | `sudoku4x4.cpp` Ligetron circuit complete |
| 1-2 | Frontend | Basic 4x4 grid UI, puzzle generator |
| 2-3 | Integration | Ligetron prover connection, proof generation |

**PoC Success Criteria:**
- [ ] Valid Sudoku solutions generate ZK proofs
- [ ] Proofs can be submitted via Soundness CLI
- [ ] Basic web UI functional

### Phase 2: MVP (3-4 weeks)

| Week | Milestone | Deliverables |
|------|-----------|--------------|
| 4 | Polish UI | Animations, mobile support, timer |
| 5 | Features | Difficulty levels, daily challenges |
| 5-6 | Testing | Bug fixes, performance optimization |
| 6 | Launch | Documentation, community release |

**MVP Success Criteria:**
- [ ] Production-ready UI/UX
- [ ] Multiple difficulty levels
- [ ] Leaderboard integration (if API available)
- [ ] Complete documentation

---

## Innovation

### What makes your vApp unique?

1. **First Mini Sudoku on Soundness Layer**
   - No existing Sudoku game in the ecosystem
   - Fills gap between complex games (8 Queens, Knights Tour) and quick games

2. **Accessibility Focus**
   - 30-second average solve time vs 5+ minutes for existing games
   - Universal game recognition (everyone knows Sudoku)
   - Lower barrier to entry for ZK newcomers

3. **Educational Value**
   - Simple constraint system demonstrates ZK concepts clearly
   - Open-source circuit serves as learning resource
   - Template for other puzzle games

4. **Expandability**
   - 4x4 → 6x6 → 9x9 progression possible
   - Daily challenge system potential
   - Tournament/competition ready

---

## Resources & Links

- **Soundness Layer Docs**: https://soundness.xyz
- **Ligetron Examples**: https://github.com/SoundnessLabs/zkvm-examples/tree/main/ligetron
- **8 Queens Circuit Reference**: https://github.com/SoundnessLabs/zkvm-examples/blob/main/ligetron/8queens.cpp

---

## Contact

- **Preferred Contact Method**: Discord
- **Discord**: xeni0381
- **GitHub**: https://github.com/0xbyt4
- **Response Time**: Within 24 hours

---

## Pre-submission Checklist

- [x] All fields completed
- [x] GitHub username matches PR author
- [x] Discord ID is correct
- [x] SL integration clearly documented
- [x] Technical architecture defined
- [x] Timeline is realistic
- [x] Innovation value explained
