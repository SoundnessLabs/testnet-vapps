# vApp Submission: [SudokuProof]

## Verification

```yaml
github_username: "Fiqrian"
discord_id: "346425843100155904"
timestamp: "2025-08-21"
```

## Developer

- **Name**: Fiqrian
- **GitHub**: @fiqrian
- **Discord**: 0xfix09
- **Experience**: Hello, let me introduce myself. My name is Fiqrian Faturachman and I am currently working as a freelancer in various company agencies. I have taken various jobs related to Blockchain and finance, including being a tester for apps, Testnet, and running validator.

## Project

### Name & Category

- **Project**: SudokuProof
- **Category**: gaming

### Description

**Problem:** Classic Sudoku apps cannot cryptographically prove honest play and are easy to cheat or share solutions. They are also a missed opportunity to teach zero-knowledge proofs (ZK) with a smooth user experience.
**What it does:** SudokuProof lets players solve Sudoku off-chain and submit a zero-knowledge proof on-chain that verifies the solution matches the chosen puzzle and satisfies all Sudoku constraints. Valid proofs earn on-chain badges/NFTs, streak tracking, and entries on a public leaderboard.

### SL Integration

- **On-chain ZK verification:** Use Soundness Layer (SL) verifier (PLONK/Groth16) to verify the Sudoku circuit.
- **Nullifier for anti double-claim:** `nullifier = PoseidonHash(address, puzzleId, salt)`; the contract ensures each nullifier is used only once.
- **Puzzle bank membership:** Store a Merkle root of allowed puzzles on-chain; claims include `puzzleId` and membership proof.
- **WALRUS blobs (optional):** Store puzzle JSON, artwork, and score proofs as blobs; contract persists `blobId` references.
- **(Optional) VRF/entropy:** Fair, unpredictable daily puzzle selection on-chain.

## Technical

### Architecture

- **Publish puzzles:** Off-chain curator compiles a puzzle set → compute `merkleRoot` → register in `PuzzleRegistry` contract.
- **Player flow:** Pick puzzle → solve in browser → generate ZK proof (WASM prover) with private inputs (solution) and public inputs (`puzzleId`, `puzzleCommit`, `solutionHash`, `nullifier`).
- **On-chain claim:** Submit `proof + publicInputs` to `SudokuVerifier` contract → verify proof → check `!used[nullifier]` and Merkle membership → mint reward (SBT/NFT) + emit events.
- **Indexing & UX:** Off-chain indexer consumes events for leaderboard, streaks, and season rewards; frontend shows verified results in real time.

**Circuit constraints (summary):**

- Each cell ∈ {1..9}.
- Each row/column/subgrid is a permutation of 1..9.
- Clue cells equal their given values.
- `Poseidon(solution) == solutionHash` tied to `puzzleId`.

### Stack

- **Frontend:** React + TypeScript + Vite; wagmi/ethers; WebWorker + WASM prover (snarkjs/halo2-wasm).
- **Backend (optional):** Node.js/NestJS for indexer + REST/GraphQL for leaderboard/streaks.
- **Blockchain:** Soundness Layer (verifier + reward contracts).
- **Storage:** WALRUS for puzzle/artwork blobs; Postgres/SQLite for leaderboard cache; IPFS as static asset fallback.

### Features

1. **Verifiable Play:** Submit a ZK proof that your Sudoku solution is valid and matches the puzzle.
2. **Daily & Seasons:** Rotating daily puzzles, streak tracking, and seasonal rewards (NFT/SBT).
3. **Anti-Cheat:** Nullifier-based one-claim-per-puzzle; Merkle membership restricts claims to curated puzzles.

## Timeline

### PoC (2–4 weeks)

- [ ] Basic play UX (board, clues, validation)
- [ ] SL verifier integration with a single puzzle
- [ ] Simple UI + claim flow

### MVP (4–8 weeks)

- [ ] Full features (daily puzzles, Merkle registry, rewards, leaderboard)
- [ ] Production hardening (tests, monitoring, rate limits, docs)
- [ ] User testing (closed beta & feedback iteration)

## Innovation

- **Proof-of-Play:** First-class, on-chain verifiable Sudoku—players prove correctness without revealing the full solution publicly.
- **Education-friendly:** A practical, approachable demo of constraint satisfaction and ZK proofs with in-browser proving.
- **Composable reputation:** On-chain badges/streaks can be re-used by other dApps for reputation, quests, and gated access.

## Contact

- **Preferred:** Discord (to be shared in PR)
- **Updates:** GitHub Releases & project README

**Checklist before submitting:**

- [ ] All fields completed
- [ ] GitHub username matches PR author
- [ ] SL integration explained
- [ ] Timeline is realistic
