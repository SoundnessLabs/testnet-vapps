# vApp Submission: DeFi Identity Vault

## Verification

github_username: "humorsingkat8"
discord_id: "1029244069425205258"
timestamp: "2025-08-29"

## Developer

Name: Humsing
GitHub: @humorsingkat8
Discord: mfsal77#0
Experience: 6+ years in smart contract development, contributor to DeFi protocols, background in security engineering and web3 infrastructure.

## Project

### Name & Category

* **Project:** DeFi Identity Vault
* **Category:** defi / identity

### Description

DeFi Identity Vault addresses the problem of identity and on-chain reputation in DeFi. Currently, DeFi users struggle to prove identity, reputation, or creditworthiness without sacrificing privacy. This vApp provides a decentralized identity vault where users can store verification attestations, DeFi activity history, and reputation scores that can be consumed by other DeFi protocols to unlock benefits such as undercollateralized credit or higher trading limits.

### SL Integration

Soundness Layer will be used to store and verify identity and reputation attestations.
Walrus: stores encrypted identity metadata & proof commitments.
Sui Sequencer: ensures fast finality for vault transactions (identity deposit, reputation updates).
Attestation Service: provides verifiable proofs of DeFi activity (e.g., trading volume, repayment history) consumable by third-party protocols in a trust-minimized way.

## Technical

### Architecture

1. User onboarding → submits encrypted identity/reputation proofs.
2. Vault Smart Contract on Sui stores pointer + hash to data in Walrus.
3. Attestation Engine generates verified activity & reputation attestations.
4. Consumer dApps (DEX, lending) read attestations from SL to determine user eligibility.

### Stack

Frontend: React + Tailwind, wallet adapter for Sui.
Backend: Node.js (API & indexer).
Blockchain: Sui + Soundness Layer.
Storage: Walrus for data availability, Postgres for cache/index.

### Features

On-chain identity vault with encrypted commitment storage.
DeFi reputation attestations (e.g., on-time repayment, transaction volume).
Integration with third-party DeFi protocols to unlock premium services.

## Timeline

PoC (2–4 weeks):

  * Deploy basic Vault contract.
  * Integrate Walrus for data commits.
  * Minimal UI to submit identity + read status.

MVP (4–8 weeks):

  * Add reputation & attestation module.
  * Full integration with SL attestation.
  * Demo integration with testnet DEX/lending dApp.
  * User testing & full documentation.

## Innovation

Unique: Combines identity & reputation into DeFi in a trustless way, enabling protocols to build advanced services (e.g., undercollateralized lending) without sacrificing privacy.
Value add: Provides a reusable on-chain reputation layer, acting like a universal "DeFi credit score."

## Contact

Preferred: Discord : mfsal77#0
* Updates will be shared via README and Soundness Labs community channels.

## Checklist before submitting

* [x] All fields completed
* [x] GitHub username matches PR author
* [x] SL integration explained
* [x] Timeline is realistic
