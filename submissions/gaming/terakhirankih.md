# vApp Proposal: **Fair Raffle — ZK Verifiable Draw**

**Category:** Gaming / Raffle  
**GitHub Username:** terakhirankih  
**Discord ID:** 824635474668552193  
**Project Repository:** https://github.com/terakhirankih/fair-raffle-zk  

---

## 1) Summary

**Fair Raffle** is a minimal, end‑to‑end example of a verifiable on‑chain raffle built on the Soundness stack:

- Participants are listed off‑chain (optionally committed by a **Merkle root**).
- Raffle winners are drawn **deterministically** from a public seed and the Merkle root.
- A **Groth16 (BN254)** proof is generated via **SP1** to attest that the published winners are correct given the inputs.
- (Optional) Public inputs and metadata are published to **Walrus** for data availability.
- A **Move** contract on **Sui testnet** verifies the proof (via `sp1-sui`) and publishes the winners on‑chain.

The goal is a transparent, reproducible raffle where any observer can verify that winners were not tampered with.

---

## 2) Problem & Motivation

Traditional raffles rely on a trusted operator and opaque randomness. Users cannot be sure that the random draw was honest or that the participant list was frozen before drawing. **Fair Raffle** removes these trust assumptions by:
- Using a **public seed** (e.g., block hash) combined with a **committed participant set** (Merkle root).
- Proving the correct selection of **k** unique winners without revealing any secret inputs.
- Recording the verification result and winners **on‑chain**.

---

## 3) Core Idea & User Flow

1. **Freeze participant set** and compute a `merkle_root` (or use the raw list for POC).
2. Pick a **public seed** (e.g., latest finalized block hash, or verifiable randomness beacon).
3. Off‑chain **SP1 host** feeds `(merkle_root, seed_public, k)` into the **SP1 zk program** and computes the winners.
4. SP1 generates Groth16 proof + public inputs blob.
5. (Optional) Upload public inputs to **Walrus** and obtain a `blobId` for DA.
6. Submit a Sui transaction calling Move `verify_and_publish` with `(vk, proof, public_inputs)` (or link to Walrus blob).
7. Contract verifies the proof (via `sp1-sui`) and **emits an event** with the winners and `raffle_id`.

---

## 4) Technical Architecture

**Components**
- `zk/program/` — SP1 zkVM program that recomputes winners from `(seed_public || merkle_root)` and enforces:
  - Exactly **k** winners, no duplicates
  - Winners are valid indices in the committed set
- `zk/host/` — Rust CLI that prepares inputs, runs SP1 proving, and materializes artifacts:
  - `public_inputs.json`, `proof.bin`, `vk.bin`
- `contracts/` — Sui Move module (`FairRaffle.move`) that:
  - Exposes `verify_and_publish` entry to verify Groth16 proof via **sp1‑sui**
  - Stores winners and emits `WinnersPublished`
- `scripts/` — Optional helpers:
  - `publish_walrus.ts` to upload public inputs to **Walrus** and return `blobId`
  - `verify.ts` to submit verification tx using **@mysten/sui.js**
- `web/` — Minimal Next.js UI to drive the flow for a demo.

**Proof System**
- SP1 (Groth16 on BN254). Artifacts: `vk`, `proof`, `public_inputs`.

**Public Inputs (POC)**
```jsonc
{
  "merkle_root": "<32-byte hex>",
  "seed_public": "<32-byte hex>",
  "k": 5,
  "winners": [12, 987, 4321] // indices; included for on-chain checking convenience
}
```

**Private Inputs (POC)**
```jsonc
{
  "participants_leaf_hashes": ["<32b hex>", "..."], // if Merkle proof path is used per winner
  "merkle_paths": ["<encoded path>", "..."]         // optional: can be omitted in first POC
}
```

---

## 5) How to Run the POC (Local)

> Prereqs: Rust, Node 20+, pnpm, Sui CLI (testnet), and optionally Walrus CLI.  
> Notes: The repository is organized as:
>
> ```
> zk/program/   # SP1 zkVM program
> zk/host/      # Host CLI to drive proving
> contracts/    # Sui Move contract
> scripts/      # Walrus + verify scripts (TS placeholders)
> web/          # Minimal Next.js UI (POC)
> ```

### A) Build & run SP1 prover (mock or CPU)

```bash
# 1) Install rust toolchain and SP1 target if required by your environment
rustup default stable
# rustup target add riscv32im-succinct-zkvm-elf   # only if your SP1 setup requires this

# 2) Build host
cd zk/host
cargo build --release

# 3) Prepare inputs (example)
mkdir -p ../../out
echo '{ "merkle_root": "0x'$(printf "%064x" 1)'", "seed_public": "0x'$(printf "%064x" 2)'", "k": 3, "winners": [0,1,2] }' > ../../out/public_inputs.json

# 4) Run host to (stub) generate artifacts
#    SP1_PROVER can be: mock | cpu | cuda | network (depending on your environment)
SP1_PROVER=mock ./target/release/prover-host prove   --public-inputs ../../out/public_inputs.json   --out ../../out

# Artifacts will appear under ../../out:
#   - public_inputs.json
#   - proof.bin
#   - vk.bin
```

### B) (Optional) Publish public inputs to Walrus

```bash
# Node deps for scripts
cd ../../scripts
npm i -g ts-node
# Example (placeholder script)
ts-node publish_walrus.ts ../out/public_inputs.json
# -> prints a blobId; keep it to reference on-chain
```

### C) Deploy & call the Sui Move contract

```bash
# 1) Build & publish the Move package
cd ../contracts
sui client publish --gas-budget 20000000

# 2) Call the verify function (example names; adjust after you publish)
# NOTE: replace PACKAGE, MODULE, FUNCTION, and object IDs accordingly
sui client call   --package <PACKAGE_ID>   --module FairRaffle   --function verify_and_publish   --args <path_to_vk_bin> <path_to_proof_bin> <path_to_public_inputs_json_or_blobId>   --gas-budget 20000000
```

### D) Web demo (optional)

```bash
cd ../web
pnpm i
pnpm dev
# open http://localhost:3000
```

---

## 6) Stretch Goals & Roadmap

- **Merkle membership enforcement** inside zk program (supply inclusion proofs per winner).
- **VRF / beacon integration** for seed source (e.g., drand, Sui epoch hash).
- **Anti‑replay** via unique `raffle_id` and contract state checks (already scaffolded).
- **On‑chain claim flow** for winners (optional NFT ticket mint + claim bitmap).
- **DA**: Store public inputs in **Walrus** and reference via `blobId` on‑chain.
- **Audits**: Move/zk logic review, reproducible builds and CI.

---

## 7) Risks & Mitigations

- **Determinism & reproducibility**: fully specify RNG (e.g., ChaCha20 seeded by `blake3(seed_public || merkle_root)`).
- **Duplicate winners**: enforce set uniqueness in zk logic and re-check on-chain.
- **Participant manipulation**: commit set before draw; store/emit `merkle_root` and `raffle_id` on-chain.
- **Verification failure**: surface reason codes and store audit metadata (block height, seed, commit).

---

## 8) Implementation Status (POC)

- ✅ Project layout: `zk/program`, `zk/host`, `contracts`, `scripts`, `web`
- ☐ Implement zk program constraints (winner derivation, uniqueness, optional Merkle checks)
- ☐ Fill SP1 host CLI to produce real `proof.bin` / `vk.bin`
- ☐ Wire Move contract with actual `sp1-sui` verify call and public input parsing
- ☐ Finish Walrus publishing helper & Sui tx helper (`scripts/`)
- ☐ Fix small UI issues in `web/` and add basic forms to drive the flow

---

## 9) License

MIT

---

## 10) Contacts

- GitHub: **@terakhirankih**  
- Discord: **824635474668552193**

> Notes for reviewers: This POC focuses on demonstrating *end-to-end integration* across SP1 → Walrus → Sui verification. The core constraints are intentionally simple to make the flow auditable and reproducible during review.