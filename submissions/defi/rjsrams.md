# Soundness Mini Faucet — vApp Proposal

**Category:** defi
**Project Type:** vApp (Verifiable Application)
**Author (GitHub):** @rjsrams
**Discord ID:** *706881269577023560*

---

## 1) Summary

A minimal **token faucet** for Soundness testnet that mints a capped amount of test tokens per address with a 24-hour cooldown. Each mint action is executed and **proven** through Soundness Layer (SL) so anyone can verify that the mint rules were enforced (cap + cooldown) without trusting the operator.

**Why now?** Many early builders need test tokens to experiment. This vApp provides a simple, auditable source of tokens with **verifiable proofs** for each mint.

---

## 2) Problem Statement

Developers need a reliable, rate-limited token faucet. Traditional faucets are opaque (no verifiable guarantees) and prone to abuse. We want a faucet where each token distribution is **provably** compliant with the rules.

---

## 3) Proposed Solution

* Smart contract `MiniFaucet` with:

  * `mint()` that mints fixed `MINT_AMOUNT` if caller hasn’t minted in the last `COOLDOWN` period.
  * Emits `Minted(address user, uint256 amount, uint256 timestamp)`.
* All `mint()` calls are paired with **proof generation** via Soundness CLI; proofs can be checked to ensure policy compliance.
* Optional lightweight UI to trigger `mint()` and display latest proof references.

---

## 4) Architecture Overview

**On-chain:**

* Solidity contract `MiniFaucet` (Solidity ^0.8.24) deployed on Soundness testnet.
* Storage: `lastMint[address]` timestamp; constants for `MINT_AMOUNT` and `COOLDOWN`.

**Off-chain (vApp workflow):**

1. User triggers `mint()`.
2. Transaction is submitted to Soundness testnet.
3. **Soundness CLI** generates/attaches a proof for the execution (policy rules enforced).
4. Proof reference (e.g., blob id) is logged and can be verified by anyone.

**Data Flow:**
User → Contract `mint()` → Tx executed → SL Proof Generated → Proof ID stored/logged → Verifiers/Explorers validate.

---

## 5) Soundness Layer (SL) Integration

* Use **Soundness CLI** for:

  * Contract deployment (testnet).
  * Proof generation after each `mint()`.
  * Retrieval of proof references (blob id) for explorer/bot.
* Provide `examples/` scripts and README commands to:

  * `deploy` contract
  * `call:mint`
  * `proof:generate` and `proof:verify`

---

## 6) Milestones & Timeline (Realistic)

**Week 1**

* Scaffold Hardhat/Foundry repo, write `MiniFaucet.sol`.
* Local tests for cooldown & cap logic.
* Deploy to Soundness testnet via CLI.

**Week 2**

* Integrate proof workflow with CLI (document blob id handling).
* Produce `examples/` commands and troubleshooting section.

**Week 3**

* Optional: Minimal Next.js UI to call `mint()` and display latest proof IDs.
* Community testing + finalize docs.

---

## 7) Deliverables

* `contracts/MiniFaucet.sol`
* `scripts/` or `examples/` with CLI commands (deploy, mint, proof)
* README usage guide + troubleshooting (blob id, proof timing)
* (Optional) `app/` minimal UI

---

## 8) Success Criteria / KPIs

* ≥ 30 successful `mint()` proofs generated & verified by different addresses.
* Zero successful bypass of cooldown in tests.
* Clear explorer/bot logs referencing proof IDs.

---

## 9) Risks & Mitigations

* **Sybil attempts:** Cooldown + per-address cap; optionally require social/GitHub verification for UI.
* **Proof generation delays:** Document retry/backoff; queue requests.
* **RPC/CLI issues:** Provide troubleshooting & fallbacks in README.

---

## 10) Team

* **Lead:** @rjsrams
* (Open to collaborators from Soundness\_Dev playground)

---

## 11) Links

* GitHub: [https://github.com/rjsrams](https://github.com/rjsrams) *(portfolio/profile)*
* (Repo to be created upon approval)

---

## 12) Notes

* This vApp is intentionally minimal to serve as a canonical example for **verifiable distribution flows** on Soundness Layer.
* Ready to extend to: variable quotas, role-based admin, faucet replenishment hooks, or attestations.

