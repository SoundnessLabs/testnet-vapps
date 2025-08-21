# zk Age/Residency Pass + Discord Role-Gating

- Category: identity
- Author: marviano
- Discord: 366120119283351553
- Status: Proposal (Testnet)

## Summary
Privacy-preserving age (18+) and residency (e.g., US) attestations using zk proofs. Users generate a proof locally from committed PII; Soundness Layer verifies on-chain and emits an event/attestation. A Discord bot links wallet↔Discord and role-gates based on on-chain verification.

## Problem
Communities/apps need to enforce 18+ and basic residency gating, but today this leaks PII or relies on unverifiable screenshots.

## Solution
- Off-chain: user inputs DOB and country; client creates commitments.
- ZK: proof that age >= threshold and residency equals target, without revealing PII.
- On-chain (SL): verify proof; record attest event or mint non-transferable testnet badge.
- Discord bot: verify wallet ownership and on-chain attestation; assign Discord role.

## Users
- Communities needing 18+ channels or geo roles
- Apps needing basic compliance gating

## Architecture
- Web dApp (Next.js/TS) + wallet
- ZK circuits (Circom): AgeThreshold, ResidencyEquals
- Prover: browser or small service (snarkjs/groth16)
- SL contract: Verifier + AttestationRegistry (emit event/mint SBT)
- Discord bot (Node/discord.js): nonce sign-in, role assignment

## Soundness Layer Integration
- Deploy verifier and registry contracts on SL testnet.
- Function: verifyProof(bytes proof, bytes32 publicSignals) → store attestation keyed by wallet, type (AGE18, RES_US).
- Emit events for off-chain indexers; optional non-transferable token.

## ZK Design
- AgeThreshold: commit to dob (yyyy-mm-dd as integer); prove dob <= (today - 18y). Use bucketed “year-month” to simplify date math and avoid revealing exact dob.
- ResidencyEquals: commit to ISO country code; prove equality to target (e.g., “US”). Future: Merkle membership for allow/deny lists.
- Public signals: commitment, policy (age 18, country=US), nullifier to prevent spam if needed.

## Data Flow
1) Client: input PII → commitments → generate proof(s)
2) Submit proof to SL → verify → emit attestation
3) Discord bot: link wallet via signed nonce → check attestation → grant role

## Roadmap
- Week 1: Circuits (age, residency), local proving, verifier contracts
- Week 2: Web UI + wallet, SL integration, end-to-end proof verify
- Week 3: Discord bot + role flow, demo + docs
- Stretch: SBT badge, Merkle allowlist, multiple regions, revocation

## Risks
- Date handling edge cases; mitigate by using month buckets
- Residency trust model (self-claim vs. KYC); keep clearly “community gating,” not compliance
- Proof size/perf; pick Groth16, pre-generate keys

## Team
- Only me but still open to any collaborators. Roles: circuits, FE, contracts, bot.

## Success Criteria
- On SL testnet: verify ≥2 proof types (AGE18, RES_US)
- Discord: role assignment flow from new wallet in <2 minutes
- Demo video and docs

## Links
- Repo(s): TBA
- Deployed contracts: TBA
