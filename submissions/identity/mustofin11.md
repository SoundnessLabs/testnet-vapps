# vApp Submission: Sui Explorer Pass (PoX Attest)

## Verification
- **github_username:** `mustofin11`
- **discord_id:** `950397008366174249`
- **timestamp:** `2025-08-30`

## Developer
- **Name:** mustofin11
- **GitHub:** @mustofin11
- **Discord:** ID 950397008366174249
- **Experience:** Testnet grinder & DeFi user; familiar with wallet integrations, simple dashboards, and task automations.

---

## Project

### Name & Category
- **Project:** Sui Explorer Pass (PoX Attest)
- **Category:** Identity

### Description
**Problem:** Many programs want real users—not sybils—who actively explore the ecosystem. Today it’s hard to prove “meaningful exploration” across dApps without exposing which apps a user touched.  
**Solution:** Sui Explorer Pass issues **portable attestations** that a wallet has interacted with **≥ K unique Sui dApps within a time window (e.g., last 14 days)**, optionally across categories (DeFi/NFT/Game/Infra). The attestation hides exact apps and only reveals **counts/buckets**, enabling allowlists, rewards, or rate-limit boosts without leaking sensitive usage data.

---

## Soundness Layer Integration
- **Attestations:** Issue verifiable attestations containing:  
  `{ wallet, chain:"sui-testnet", window_start, window_end, unique_dapp_count_bucket, category_bits, snapshot_root, issued_at, expiry }`.
- **Proofs:** Start with deterministic snapshot proofs; roadmap to **threshold/range proofs** (prove ≥K without exact list) when available.
- **Verification:** Third-party dApps verify via SL SDK/CLI before granting incentives (rebates/allowlists/XP).

---

## Technical

### Architecture (high level)
1. **dApp Registry:** Curated list of Sui contracts with category tags (YAML/JSON).
2. **Indexer Worker (Sui):** Using `@mysten/sui.js` + RPC/indexer to scan wallet tx/events and compute **unique dApp count** per window.
3. **Rule Engine:** Applies window (e.g., 14 days), dedupes, maps to **count buckets** (≥3, ≥5, ≥10).
4. **Attestation Service:** Generates & signs SL attestations; stores only snapshot hashes.
5. **Verifier API/SDK (TS):** `verifyAttestation(att, rule)` for partner dApps.
6. **Web Dashboard:** Connect Sui wallet → view progress → request attestation → export to partner demo.

### Stack
- **Frontend:** React (Next.js) + `@mysten/sui.js` + wallet-standard (Sui Wallet/Surf/Ethos)
- **Backend:** Node.js (TypeScript)
- **Blockchain:** **Sui Testnet** + Soundness Layer
- **Storage:** Postgres (cache); metadata/snapshots → IPFS/WALRUS (optional)

### Features
- **F1. Explorer Tracker:** Progress to K unique dApps in last N days.
- **F2. Threshold Attest:** Issue “≥K unique dApps in window” attestations (with category bits).
- **F3. Partner Verifier:** Tiny lib for partners to validate and grant perks (rebates/allowlists/XP).
- **F4. Anti-gaming Heuristics:** Cooldowns for rapid churn; optional wallet-age checks.

---

## Timeline
### PoC (2–4 weeks)
- Build dApp registry (10–20 contracts).
- Index & snapshot unique dApps per wallet (14-day window).
- Generate & verify **basic SL attestation**.
- Minimal UI to connect wallet & request attestation.

### MVP (4–8 weeks)
- Category bits + multiple windows (7/14/30 days).
- Partner demo dApp (allowlist/XP).
- Auto-revocation on detected fraud; dashboard & analytics.

---

## Innovation
- **Portable exploration reputation** across Sui, privacy-preserving.
- **Low integration friction**: simple verifier call; no KYC.
- **Sybil-resistant signals** via unique-dApp counts & windows.

---

## Contact
- **Preferred:** Discord DM (ID: 950397008366174249)
- **Updates:** GitHub issues/PRs + Soundness Discord build channel

---

## Checklist before submitting
- [x] All fields completed
- [x] GitHub username matches PR author (`mustofin11`)
- [x] SL integration explained (attestations + verifier; threshold/range proofs roadmap)
- [x] Realistic timeline (PoC 2–4w, MVP 4–8w)
