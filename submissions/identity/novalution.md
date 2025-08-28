# Proof of Contribution Tracker

**Category:** identity  
**Author GitHub:** [novalution]  
**Discord ID:** [306756808197013506]  

## Summary
A lightweight vApp that records user contributions (e.g., bug reports, commits, forum posts) and produces verifiable proofs using the Soundness layer. Communities and programs can independently verify that a claimed contribution exists, when it happened, and by whom, supporting fair credit and reputation.

## Problem
Open-source and testnet communities rely on screenshots or mutable spreadsheets to track work. This is error-prone and can be gamed. We need tamper-evident contribution logs with portable verification.

## Proposed Solution
- CLI-driven tracker that appends contribution entries (`user`, `type`, `details`, `timestamp`) to a JSON ledger.
- Generate a proof artifact via Soundness CLI for each batch (e.g., `contributions.json → contribution-proof.json`).
- Verifiers can check the artifact and confirm integrity and timestamps.

## Soundness Integration
- Use **Soundness CLI** to create verifiable proofs from structured JSON inputs.
- Example:
  ```bash
  # Append a contribution locally
  node tracker.js "alice" "bug-report" "Fixed UI alignment issue"

  # Generate a proof for the updated ledger
  soundness proof --input contributions.json --output contribution-proof.json
