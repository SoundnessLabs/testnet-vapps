
# vApp Proposal — 2048 ZK Score (SP1 → Walrus → Soundness Layer)

**Category:** gaming  
**GitHub Username (PR author):** `mogabejo7`  
**Discord ID:** `825570534162169876`  
**App repo:** https://github.com/mogabejo7/2048  
**License:** MIT (inherits base game license)

---

## Summary
**2048 ZK Score** turns a classic browser game into a verifiable score challenge. The app uses **SP1 (Groth16)** to prove that a player's final **score** and **max tile** are the correct result of a deterministic replay of their inputs, stores the proof & program ELF on **Walrus** for public data availability, and submits the result to **Soundness Layer** for an attestation. Users authenticate with a **Sui wallet** (signing a personal message), play as usual, then click **Prove & Submit** to publish a verifiable record of their run.

**User value:** anti-cheat high scores, transparent competitions, and an integration pattern reusable for many casual games where state can be deterministically replayed from user inputs.

---

## Problem & Target Users
- **Problem:** Leaderboards for casual web games can be faked via local hacks or state manipulation. Organizers need scores that are tamper-resistant and publicly auditable.  
- **Users:** casual gamers, speedrun/score-challenge communities, mini-tournament hosts.  
- **Need:** low-friction wallet login, one-click proving, public verification links, and optional on-chain leaderboards.

---

## Technical Solution (High Level)
1. **Input instrumentation** — every arrow input (`↑→↓←`) is appended to `moves: Vec<u8>` (`0,1,2,3`).  
2. **Deterministic RNG** — public `seed: [u8;32]` + a `session: [u8;16]` drive the 2048 tile spawns in the guest program (SP1), ensuring replays are deterministic.  
3. **SP1 guest program (Rust)** — replays the full 4×4 2048 game: compress/merge rules, 90% *2* vs 10% *4* spawns, updates `score` and `max_tile`. It commits a 32-byte public output:  
   ```rust
   // Public = { player: [u8;32], score: u64, max_tile: u16, session: [u8;16] }
   let payload = bcs::to_bytes(&public).expect("bcs");
   let digest = sha3::Sha3_256::digest(&payload); // [u8;32]
   sp1_zkvm::io::commit(&digest);
   ```
4. **SP1 prover host (Rust, `sp1-sdk`)** — loads the guest ELF, writes stdin `{seed, moves, player, session}`, then produces a **Groth16** proof + `public_values`.  
5. **Walrus DA** — uploads `proof.bin` and `program.elf` via **Upload Relay**, returns `blobId/quiltId` references.  
6. **Soundness Layer** — the server submits `{ publicInputsHash, walrusProofId, walrusElfId, metadata }` using the current official path (CLI server-side for now; will switch to REST when available).  
7. **(Optional) Sui on-chain leaderboard** — a Move module stores `(player, score, max_tile, publicHash, walrusRef)` and emits events; verification can reference Soundness attestations.

---

## Architecture & Data Flow

```
[Browser]
  ├─ Sui Wallet (connect + personal message sign)
  ├─ Game UI (Next.js/React) ───► record moves[]
  └─ POST /api/score/submit { seed, moves, player, session }
            │
            ▼
[Next.js API]
  ├─ spawn SP1 prover host (Groth16)
  │    └─ run guest ELF ⇒ commit H = sha3_256(bcs(Public))
  ├─ read out/{proof.bin, public_values.bin, program.elf}
  ├─ WalrusClient.writeFiles([proof, elf]) → blobId/quiltId
  └─ Soundness submit (server-side) { publicHash, walrusIds, meta }
            │
            ▼
[Soundness Layer]
  ├─ verifies proof & commit
  └─ publishes attestation (linking Walrus and public hash)

(Optional) [Sui] Move leaderboard ingests attestation/hash
```

**Key components**
- **Frontend:** Next.js + `@mysten/dapp-kit` for wallet connect & personal message signing; a small hook records `moves[]`.  
- **Guest:** `zk/sp1-program` with `sp1_zkvm::entrypoint!`; deterministic 2048 + `commit`.  
- **Host:** `zk/prover` using `ProverClient::setup(&elf)` and `.groth16().run()`; writes artifacts to `zk/out`.  
- **Walrus:** `@mysten/walrus` + **Upload Relay** to avoid thousands of chunked requests.  
- **Soundness:** server runs CLI until a public REST is provided; our code isolates this in a single adapter function.

---

## Integration with Soundness Layer
- **Data availability:** publish `proof.bin` and `program.elf` on Walrus; keep returned IDs as permanent references.  
- **Attestation payload:** `{ publicInputsHash: "0x…", walrusProofId, walrusElfId, metadata: { score, maxTile, session, player } }`.  
- **Verification UX:** a “View Proof” page shows the public hash and links to Walrus objects and the Soundness attestation.  
- **Migration to REST:** once the official REST API is released, we will swap out the CLI-based submitter with an HTTP client while keeping payload shape intact.

---

## Development Timeline (4–6 weeks)
- **M1 — SP1 local proving (CPU)** (Week 1–2): finalize guest replay + commit; produce Groth16 proof deterministically for given `moves[]`.  
- **M2 — Walrus Upload Relay + persistence** (Week 2–3): upload proof/ELF; store `blobId/quiltId`; public read endpoint.  
- **M3 — Soundness submission (server-side)** (Week 3–4): integrate CLI; save `submissionId`; “View Proof” page.  
- **M4 — (Optional) Sui leaderboard** (Week 4–5): minimal Move module + UI table.  
- **M5 — Hardening & UX polish** (Week 5–6): rate limits, replay-protection for signatures, upload retries, and basic telemetry.

---

## Risks & Mitigations
- **API changes (Soundness submit):** isolate in an adapter; trivial swap CLI → REST later.  
- **Proving time on CPU:** expose a queue + progress; document GPU/network-prover options.  
- **Artifact size & upload time:** rely on Upload Relay; optionally compress safe payloads.  
- **Input fraud:** inputs flow from the in-app listener; bind session to wallet signature + nonce to prevent replay/mix-and-match.

---

## Success Criteria & Demo Plan
- **Success:** (i) an end-to-end run produces a Soundness attestation tied to Walrus blob IDs, (ii) anyone can inspect the public hash and referenced artifacts, (iii) (optional) the score appears on-chain in a Sui leaderboard.  
- **Demo:** live web app link, JSON dump of `{player, score, maxTile, session, publicHash}`, and Walrus/Soundness links.

---

## Team
- **Owner:** @mogabejo7 (solo developer).  
- **Needs:** testnet access, Walrus relay quota, Soundness CLI/REST credentials.

---

## Checklist
- [x] GitHub username equals PR author (`mogabejo7`)  
- [ ] Discord ID filled (`<YOUR_DISCORD_ID>`)  
- [x] SL integration & architecture documented  
- [x] Realistic 4–6 week timeline

---

## Technical Appendix
**Public struct**
```rust
#[derive(Serialize, Deserialize)]
pub struct Public {
  pub player: [u8;32],
  pub score: u64,
  pub max_tile: u16,
  pub session: [u8;16],
}
```
**Commit**: `H = sha3_256(bcs(Public))` (32 bytes) — submitted as `publicInputsHash`.  
**Move dir mapping**: `0=Up, 1=Right, 2=Down, 3=Left`.  
**Spawn**: 90% → 2, 10% → 4; PRNG xorshift64* seeded by `seed`.
