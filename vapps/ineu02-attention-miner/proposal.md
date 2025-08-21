# Attention Miner

## Summary
A VAPP for “attention mining”: users earn points from measurable activities (tweets, quality engagement), verified via Soundness attestations, then converted into rewards (badges/allocations).

## Problem Statement
- Hard to measure quality of attention (not just spam).
- Many campaigns are vulnerable to bots/Sybil attacks.
- Need a way to verify reputation without exposing personal data.

## Solution (How it works)
- Users connect accounts (wallet + X handle).
- Engagement data is processed → attestation generated (quality score, unique user).
- Soundness verifier validates the proof.
- Score → leaderboard → claim rewards (badge/NFT/alloc).

## Integration with Soundness
- Use Soundness CLI to submit/verify off-chain proofs → on-chain attestations.
- Verifier ensures uniqueness and quality thresholds.
- Output: attestation hash readable by reward smart contract.

## Architecture Overview
- Client (Next.js) → Backend worker (job queue) → Soundness layer (prove/verify/attest) → DB/Indexing for leaderboard → Smart contract for mint/claim.

## UX & Security
- Anti-spam/Sybil: proof of uniqueness, rate limiting, minimum reputation.
- Privacy: data minimization, only hashes/attestations stored publicly.

## Technical Plan
- Stack: TypeScript/Next.js, Node worker, Solidity (rewards), Soundness CLI/API.
- MVP (M1): attestation scheme + verifier + CLI flow.
- M2: frontend leaderboard + wallet connect + claim flow.
- M3: referral integration & advanced anti-Sybil.

## Demo/Mock
- Routes: `/claim`, `/leaderboard`.
- Example CLI script for submitting proofs (to be documented).

## Roadmap & Needs
- M1 (1–2 weeks): PoC verifier + attestation.
- M2: UI MVP & reward contract.
- M3: mini audit + stress test.
- Support needed: verifier template examples, best practices for attestation format.
