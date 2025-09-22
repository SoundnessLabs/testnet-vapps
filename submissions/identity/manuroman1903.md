# vApp Submission: ProofPocket

## Verification
```yaml
github_username: "manuroman1903"
discord_id: "957791835181420635"
timestamp: "2025-09-22"
```

## Developer
- **Name**: Dary
- **GitHub**: @manuroman1903
- **Discord**: goncharovadashaa7217
- **Experience**: 6+ years in web development and crypto products; built zk-proof and DID wallets, led teams of 4–6 people.

## Project

### Name & Category
- **Project**: ProofPocket
- **Category**: identity

### Description
ProofPocket is a verifiable “attestation wallet” where users can collect and present provable credentials (KYC-light, age 18+, human-not-bot, community membership) without disclosing personal data. The app issues and verifies zk-proofs, simplifying onboarding into dApps and DAOs.

### SL Integration 
- Verification and aggregation of zk-proofs via Soundness Layer (SL) as a trusted verification network.
- Issuance and revocation of attestations using SL claims/attestations API.
- Batch verification for on-chain calls: one confirmed proof packet for multiple integrations.
- Use of SL-powered selective disclosure schemes: reveal only necessary predicates (e.g., age ≥ 18).

## Technical

### Architecture
A mobile/web client generates local zk-proofs and stores encrypted credentials. The backend manages attestation lifecycle, proxies verification into SL, and caches statuses. Smart contracts provide partner-facing interfaces (gates, roles, quotas). Data flow: Issuer → SL → User Wallet → Verifier (dApp/DAO).

### Stack
- **Frontend**: React + Next.js, TypeScript, Wagmi/ethers, PWA
- **Backend**: Node.js (NestJS) + GraphQL/REST, WebSockets
- **Blockchain**: SL + EVM (Base/Polygon), DID-compatible
- **Storage**: PostgreSQL + Redis; user credentials stored locally (WebCrypto) with optional WALRUS/IPFS backup

### Features
1.Non-custodial attestation wallet with local zk-proof generation
2.Attestation issuance/revocation and verification via SL
3.One-time “proof-on-request” links (deep-link/QR) for dApp/DAO integrations

## Timeline

### PoC (2-4 weeks)
- [ ] Basic functionality
- [ ] SL integration
- [ ] Simple UI

### MVP (4-8 weeks)  
- [ ] Full features
- [ ] Production ready
- [ ] User testing

## Innovation
Focus on privacy-by-default and predicate proofs without KYC leakage. Batch verification via SL reduces gas/latency and makes one passport interoperable across many dApps.

## Contact
**Discord**: goncharovadashaa7217

**Checklist before submitting:**
- [ ] All fields completed
- [ ] GitHub username matches PR author  
- [ ] SL integration explained
- [ ] Timeline is realistic
