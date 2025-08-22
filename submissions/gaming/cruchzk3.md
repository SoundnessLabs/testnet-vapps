# vApp Submission: \[One Shot]

## Verification

```yaml
github_username: "cruchzk3"
discord_id: "patricktechy"
timestamp: "2025-08-22"
```

## Developer

* **Name**: Patrick Camaing
* **GitHub**: @cruchzk3
* **Discord**: patricktechy
* **Experience**: 8 years in blockchain (DigiByte researcher, Monero moderator), Unity FPS prototyping, Solidity/dApp work (incl. Solana). Founder of HiveChain (AI + swarm-inspired L1).

---

## Project

### Name & Category

* **Project**: **One Shot — Soundness Arena Battle Royale**
* **Category**: **gaming** (with DeFi-style economy)

### Description

**Problem.** On-chain games often have clunky onboarding, unstable economies, and poor retention.
**Solution.** One Shot is a wallet-only, 2D battle royale with a simple, sustainable economy and verifiable results.

* **Entry:** 10 SOUND
* **Tax:** 10% (50% to treasury, 50% to champion bonus)
* **Rewards:** 9 SOUND per **player kill**; exit allowed at **≥ 2 player kills**
* **Champion:** last-man-standing gets **50% of tax pool**
* **Withdrawal tax:** configurable (default 2%)
* **Minions:** \~50 neutral NPCs, **0.5 SOUND** each (do **not** count toward exit rule)

**Ecosystem Value.** Onboards real players to the Soundness economy (fast wallet entry), grows treasury via predictable flows, and creates a visible, verifiable funnel attractive to investors and builders.

---

### SL Integration

**Goal:** verifiable off-chain gameplay, on-chain settlement.

* **Authoritative server → attestation:** server signs kill/minion/winner data; submits a **Soundness Layer (SL)** attestation (hash/commitment of match logs).
* **On-chain verify:** Sui Move contract checks server signature **and** SL attestation reference before paying per-kill, exits, and champion bonus.
* **Why SL:** proof-backed outcomes without heavy on-chain compute; fast, cheap, and trustable.

---

## Technical

### Architecture

1. **Client (Phaser + wallet)**: real-time play; calls join/exit/finalize.
2. **Server (Node + Colyseus)**: authoritative state; signs outcomes (ed25519); posts match summary to SL; exposes minimal API to client.
3. **Soundness Layer (SL)**: issues attestations for match summaries; reference included in settlement txs.
4. **Sui Move contracts**: round pools, entry/tax, kill accounting with **budget guard**, exit/withdraw, champion payout, sweep unclaimed.

### Stack

* **Frontend:** Phaser.js + React + Sui wallet adapter
* **Backend:** Node.js (Colyseus, ed25519)
* **Blockchain:** Sui + SL attestations
* **Storage:** Walrus/IPFS (match summaries), PostgreSQL (analytics/leaderboards)

### Features

1. **Battle Royale Economy**: entry/tax/kill/exit/champion with minions & withdrawal tax.
2. **SL-Verified Settlement**: payouts require server sig + SL attestation.
3. **Budget Anti-Exploit**: global **kills budget** so rewards ≤ pool.
4. **Low-Cost Anti-Bot (Free)**

   * **Behavioral heuristics** (mouse/keyboard timing) – server-side only.
   * **Dynamic human prompts** (rare, randomized mini-actions) for suspected bots.
   * No paid third-party anti-cheat needed.
5. **Player-for-Hire (Free)**

   * **On-chain escrow**: owner stakes entry; **delegate** gameplay to a pro (address-allowlist on Ticket).
   * **Auto split** of rewards via smart-contract percentages.
   * **Reputation**: simple on-chain rating (1–5) + counts.
6. **Provably-Fair RNG (Free)**

   * **Commit–reveal** randomness (server commit; round collective hash reveal) for drops/spawns.
   * Avoids oracle fees; verifiable in contracts.
7. **Optional Proof-of-Human (Free/Optional)**

   * **BrightID/PoH badge check** only for **tournaments**/boosted rewards.
   * Regular quick play stays wallet-only (no extra friction).

---

## Timeline

### PoC (2–4 weeks)

* [ ] Sui Move: `Round` object, `join`, `finalize_round`, pools & tax
* [ ] SOUND test token + faucet wiring
* [ ] Server signing + SL attestation (happy path)
* [ ] Phaser demo (movement/shoot/shrink zone)

### MVP (4–8 weeks)

* [ ] `submit_kill_batch`, `exit_withdraw`, withdrawal tax, kills budget
* [ ] Minions + **minion\_reward\_pool**
* [ ] **Anti-bot v1** (heuristics + dynamic prompts)
* [ ] **Hire-a-Pro v1** (escrow + split + allowlist)
* [ ] Commit–reveal RNG for spawns/loot
* [ ] Public testnet; dashboards & feedback loop

---

## Innovation

* **Growth flywheel for SL**: wallet-only onboarding + verifiable matches → more players, more attestations, stronger investor signal.
* **Hire-a-Pro (job pathway)**: in-game, on-chain escrow & revenue split—legit, transparent, and free to run.
* **AI-resistant fairness**: behavioral anti-bot + dynamic human prompts; optional human-uniqueness for high-stakes modes.
* **Zero-fee verifiability**: SL attestations + commit–reveal RNG (no paid oracles).
* **Treasury sustainability**: predictable tax flows, minion side-pool, and withdrawal tax tuning.

---

## Contact

* **Discord:** @patricktechy
* **Updates:** PRs/issues on fork; Soundness Discord #gaming

---

**Checklist**

* [x] All fields completed
* [x] PR author = `cruchzk3`
* [x] SL integration explained
* [x] Realistic timeline (PoC→MVP)
* [x] Costs minimized (free/OSS first)
