# zk-Onboard + SSO (Google/GitHub) for Web3 Onboarding

## Team
- GitHub: @muhamadrifansyah
- Discord: asepkarbu.

## Category
identity

## Title
Privacy-Preserving Web3 Onboarding via SSO (Google/GitHub) + Soundness Layer

## Description
Enable Web2 → Web3 onboarding using familiar SSO (Google/GitHub) while preserving user privacy.  
After a user authenticates with an SSO provider, the app generates a zero-knowledge proof that the authentication is valid (e.g., “valid Google account holder”) **without revealing PII** (email, name, etc.).  
The proof is verified on **Soundness Layer** so dApps can trust onboarding events with low cost and low latency.

## Technical Architecture
1) User selects SSO (Google/GitHub) and authenticates (OAuth).  
2) Backend derives minimal attestable claims (e.g., provider=google, timestamp, nonce).  
3) ZK circuit produces a proof over those claims; raw identifiers are never revealed.  
4) Proof is submitted and verified via **Soundness Layer**; dApp receives a verifiable onboarding credential/flag.  
5) Optional: store anonymized credential refs (e.g., blob id) for re-use/refresh.

**Soundness Integration**
- Use Soundness CLI to submit/verify proofs.
- Use Soundness Layer verifier endpoints for on-chain/near-chain verification.
- (Optional) Persist proof artifacts via blob ids for auditability.

## Development Timeline (realistic PoC)
- Week 1: Define minimal claim schema + OAuth flow; draft zk constraints.
- Week 2: Implement SSO login (Google first) + dummy circuit; local proof gen.
- Week 3: Integrate Soundness CLI for proof submission & verification.
- Week 4: Frontend demo (Login → Get “Verified via Soundness” badge/credential).
- Week 5: Add GitHub SSO + basic docs & test scripts.

## Deliverables (PoC)
- Minimal web demo (frontend + backend) showing SSO → proof → verify on Soundness.
- Scripts/README to reproduce with Soundness CLI.
- Example verifier response + logs.

## Risks & Mitigations
- SSO provider changes → keep claim schema minimal & provider-agnostic.
- Circuit complexity → start with minimal claim set; iterate.
- UX → cache/refresh proofs to avoid re-auth spam.

## Contact
- GitHub: @<username-github-kamu>
- Discord: <discord-id-kamu>
