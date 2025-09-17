
# vApp Proposal: ZK‑OIDC — Selective Disclosure Sign‑in (with Fair Raffle demo)

**Category:** identity  
**GitHub Username:** kopidskem  
**Discord ID:** 825543339608375326  
**Project Repository:** https://github.com/kopidskem/zk-oidc  
**Demo URL (optional):** -

---

## 1) Summary

**ZK‑OIDC** lets users prove they signed in with an OIDC provider (e.g., Google/GitHub) and that specific claims are valid—**without revealing PII**.  
The proof is generated in **SP1** (zkVM), public artifacts are optionally published to **Walrus** (data availability), and a **Sui Move** contract (via `sp1‑sui`) verifies the proof on‑chain. As a demo, the contract can **publish fair‑raffle winners** only if a user satisfies claim requirements (e.g., `email_verified`, org domain).

**Why now?** It aligns perfectly with the **Soundness Layer** model: proofs in Walrus; attestations / verification on Sui.

---

## 2) Problem / Motivation

Today, OAuth/OIDC centralizes identity and leaks PII to dApps. Projects want trustable “sign‑in” signals (issuer, audience, claim checks) without collecting user emails or tokens. **ZK‑OIDC** proves “_I logged in with X and meet policy Y_” privately, enabling anonymous gating, allowlists, and fair raffles.

---

## 3) Core Idea & User Flow

1. **User OIDC login** (off‑chain) to obtain an ID token (JWT) and JWKS URL (per issuer).  
2. **Prover (SP1)** consumes: policy + JWT + JWKS, verifies signature (RS256/ES256), `iss/aud/exp`, computes `claims_bitmap`, and outputs public inputs (policy hash, issuer hash, nonce, claims).  
3. **(Optional) Walrus publish**: store `vk`, `proof`, `public_inputs` and keep blob IDs.  
4. **On‑chain verification (Sui)**: call Move `verify_and_publish` (uses `sp1‑sui` Groth16 verifier for BN254) with artifacts or Walrus blob IDs; contract checks proof, emits events, and (demo) **publishes raffle winners** if policy passes.

---

## 4) Technical Architecture

- **zk program (`zk/program`)**: Rust for SP1 zkVM. Parses JWT, selects JWKS key by `kid`, verifies signature, computes `claims_bitmap`, hashes policy and issuer into 32‑byte digests, asserts public inputs.
- **host (`zk/host`)**: CLI that feeds inputs, drives SP1 proving, and exports `vk.bin`, `proof.bin`, `public_inputs.json`.
- **Move (`contracts`)**: `RAFFLE::ZkOidc` imports `sp1‑sui` Groth16 verifier; `verify_and_publish` validates proof and emits `WinnersPublished` (demo).
- **web (`web`)**: Next.js minimal UI for OIDC sign‑in, prover trigger, and Sui tx submit.
- **scripts**: helpers to publish artifacts to **Walrus** and to submit the Sui verification tx.

### Public Inputs (draft)

```rust
// zk/program/src/main.rs
pub struct PublicInputs {
    pub policy_hash: [u8; 32],
    pub issuer_id_hash: [u8; 32],
    pub claims_bitmap: u32,
    pub nonce: [u8; 32],
}
```

### Policy (example)

```rust
pub struct Policy {
    pub issuer: String,    // e.g. "https://accounts.google.com"
    pub audience: String,  // your OIDC client_id
    pub required: u32,     // bitmap of required claims (bit flags)
}
```

---

## 5) Data Formats

- **Artifacts**: 
  - `vk.bin`, `proof.bin` – Groth16, BN254 (SP1).
  - `public_inputs.json` – JSON with fields above.
- **Walrus**: optional blob IDs for the three artifacts.
- **On‑chain call**: either pass raw bytes or Walrus blob IDs that the contract dereferences.

---

## 6) Soundness Integration

- **SP1** for proving (Groth16 over BN254).  
- **Walrus** for DA of artifacts (optional, recommended).  
- **Sui + sp1‑sui** verifier for on‑chain proof checking and event emission.

---

## 7) Proof of Concept (PoC)

This repo contains a runnable skeleton; TODOs are clearly marked. You can build and run end‑to‑end with placeholders first, then replace with real logic.

### Prerequisites

- **Rust** (stable toolchain) and SP1 installed.
- **Node.js 20+** and **pnpm**.
- **Sui** CLI & Move toolchain (testnet).
- **(Optional)** Walrus CLI or SDK.
- A Sui keypair for testnet (funded via faucet).

### Environment

Create `.env` at the repo root (or per tool as you prefer):

```bash
# Sui
SUI_RPC=https://fullnode.testnet.sui.io:443
SUI_KEY=<hex-or-mnemonic-or-keystore-path>

# Walrus (optional)
WALRUS_URL=https://walrus.<network>.xyz

# OIDC (used by web or host)
OIDC_ISSUER=https://accounts.google.com
OIDC_AUDIENCE=<your-client-id>
```

### Build

```bash
# 1) zk/host (prover driver)
cargo build -r -p prover-host

# 2) Move package
cd contracts
sui move build

# 3) Web (optional UI)
cd ../web
pnpm i && pnpm dev
```

> **Note on SP1**: set the prover backend with `SP1_PROVER` in `{mock|cpu|cuda|network}`; e.g.  
> `SP1_PROVER=cpu cargo run -r -p prover-host -- prove --out out/ ...`

### Run the PoC (placeholders first)

```bash
# Generate placeholder artifacts (writes out/public_inputs.json, vk.bin, proof.bin)
SP1_PROVER=mock cargo run -r -p prover-host -- prove   --policy policy.json   --jwt sample.jwt   --jwks jwks.json   --out out/

# (Optional) publish to Walrus (returns blob IDs)
pnpm ts-node scripts/publish_walrus.ts out/proof.bin

# Verify on-chain (replace pkg address & function as needed)
pnpm ts-node scripts/verify.ts
```

> The current Move contract contains a demo `verify_and_publish` that should be wired to `sp1‑sui`'s Groth16 verifier. Replace the TODOs and import paths accordingly.

---

## 8) Milestones & Timeline

- **M1 — zk core & host (Week 1):** JWT parsing, JWKS select by `kid`, RS256/ES256 verify in SP1, compute `claims_bitmap`; host exports artifacts.
- **M2 — On‑chain verify (Week 2):** integrate `sp1‑sui` Groth16 verifier; add `verify_and_publish` with events; wire winners decoding from `public_inputs`.
- **M3 — Web & DA (Week 3):** OIDC login UI, wallet connect, submit tx; optional Walrus publishing; polish and docs.

---

## 9) Risks & Mitigations

- **JWKS freshness / key‑rollover** → embed `kid` and issuer hash; require recent fetch; allow multiple keys in witness.  
- **Time checks (`exp`, `iat`)** → assert in zk program with prover time source (nonce + policy).  
- **Cross‑provider variations** → normalize JWT claims; start with Google/GitHub then extend.  
- **Proof size / gas** → prefer Walrus blobs; pass IDs to Move contract.

---

## 10) Team

- <Your name / handle> — Full‑stack ZK dev (Rust/Move/TS).

---

## 11) How to Run PoC (Quick)

```bash
# clone
git clone https://github.com/<your‑github‑username>/zk-oidc && cd zk-oidc

# env
cp .env.example .env && edit

# build & run artifacts
SP1_PROVER=mock cargo run -r -p prover-host -- prove --policy policy.json --jwt sample.jwt --jwks jwks.json --out out/

# (optional) Walrus publish
pnpm ts-node scripts/publish_walrus.ts out/proof.bin

# call Sui verifier
pnpm ts-node scripts/verify.ts
```

---

## 12) Links

- SP1 SDK — https://docs.rs/crate/sp1-sdk/latest  
- SP1 on Sui (`sp1-sui`) — https://github.com/SoundnessLabs/sp1-sui  
- Soundness Layer & Walrus — https://soundness.xyz/blog/soundness-layer

