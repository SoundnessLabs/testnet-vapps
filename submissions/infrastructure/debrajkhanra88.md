# vApp: Multi-Chain Attestation Relay

## Category
infrastructure

## Author
- GitHub: @Debrajkhanra88
- Discord: debraj2422

## Summary
A lightweight relay that listens to Soundness Layer attestations and makes them usable across multiple chains and apps.  
The idea is to have one simple service that dapps can call instead of writing custom attestation-handling logic.

## Problem
Right now, proofs/attestations are powerful but a bit hard to plug directly into dapps.  
Every team has to figure out how to fetch, verify, and re-broadcast them. That creates duplicated effort and slows adoption.

## Solution
- A relay service that:
  - Listens to new attestations (via Soundness CLI or API)
  - Validates them locally
  - Stores them in a small DB for quick access
- A REST endpoint + webhook for dapps to consume
- Writers that can push attestations to at least two chains (start with EVM → Starknet)
- A small explorer UI to view proof IDs, blob IDs, and status

## Architecture
- **Collector**: subscribes to Soundness events
- **Validator**: checks signatures + schema
- **DB**: Postgres or SQLite for indexing
- **API**: REST `/attestations/:id`, simple webhook support
- **Writers**: chain modules (start with Ethereum + test L2)
- **UI**: minimal Next.js explorer

## Soundness Layer Integration
- Uses the CLI for ingestion
- Tracks proof ID → attestation mapping
- Indexes blob IDs for quick lookup

## Milestones
- Week 1: Set up collector + DB, verify attestations locally
- Week 2: Add REST API + webhook, build minimal explorer
- Week 3: Implement EVM writer, push attestations onchain
- Week 4: Add Starknet writer + polish docs

## Deliverables
- Open-source repo (MIT)
- Docker-compose for easy run
- Minimal explorer UI
- Example dapp showing how to consume attestations

## Team & Background
I’ve been running infra + validator nodes and doing dev work with Docker, monitoring stacks, and some web3 tooling.  
This project is a way to make Soundness easier to adopt by other builders.

## Risks
- Proof times vary → handle async + retries
- Blob IDs may change → index by proof root + attestation hash
- Different chains → modular writer approach

## Support Requested
- Testnet access + attestation cost coverage for stress testing
- Feedback on best practices for re-broadcasting attestations

## License
MIT

