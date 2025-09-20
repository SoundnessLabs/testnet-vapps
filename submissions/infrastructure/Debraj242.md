# vApp: Proof Explorer Dashboard

## Category
infrastructure

## Author
- GitHub: @Debraj242
- Discord: debraj24

## Summary
A lightweight web dashboard that shows all recent Soundness proofs and attestations in a clean timeline view. Instead of CLI output, devs can open a browser and filter by address, timestamp, or proof type.

## Problem
The CLI is good for testing, but it’s hard to keep track of multiple proofs at once. There’s no quick visual way to see what happened recently without running commands repeatedly.

## Solution
- Simple web UI built with React or Svelte
- Backend service fetches proofs via API/CLI
- Timeline-style display with filters:
  - by wallet address
  - by proof type
  - by block/time
- Export to JSON/CSV for debugging or sharing

## Architecture
- Listener service (Node.js or Go)
- REST API layer
- Frontend dashboard (React/Svelte)
- Lightweight DB (SQLite) to cache events

## Milestones
- Week 1: Setup service to pull proofs
- Week 2: Build minimal REST API
- Week 3: Frontend with timeline + filters
- Week 4: Export function + polish UI

## Deliverables
- Open-source repo
- Deployment guide (Docker + local run)
- Config for API keys

## Soundness Layer Integration
- Pulls proof/attestation data from Soundness API
- Displays it in an interactive, searchable dashboard
- Makes it easier for devs to debug or showcase integrations

## Risks
- API changes may break fetcher
- Large proof sets could slow UI (needs pagination)
- Hosting frontend securely (CORS issues)

## Support Requested
- Feedback on what proof/attestation fields matter most
- Access to long-running test data for performance testing

## License
Apache-2.0
