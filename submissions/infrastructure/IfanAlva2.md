# vApp Submission: [Your Project Name]

## Verification
```yaml
github_username: "IfanAlva2"
discord_id: "780608657356357652"
timestamp: "2025-09-05"
```

## Developer
- **Name**: Ifan Alva
- **GitHub**: @IfanAlva2
- **Discord**: Alzion_7
- **Experience**: Still learning blockchain development but already familiar with smart contracts, backend APIs, and exploring zero-knowledge integrations through testnets and small projects.

## Project

### Name & Category
- **Project**: zkThrottle — ZK-Based API Rate-Limiting
- **Category**: infrastructure

### Description
zkThrottle solves the problem of API abuse and privacy trade-offs in current rate-limiting solutions.
It provides a system where users can prove, using zero-knowledge proofs, that they have not exceeded their API usage quota (e.g., 100 calls/day) without exposing detailed usage logs.
API providers can enforce trustless, privacy-preserving rate-limits via the Soundness Layer + Sui, reducing abuse and centralization risks.

### SL Integration  
Use Soundness Layer to verify ZK proofs that counter ≤ limit.

Store verification results on Sui smart contracts for API backends to query.

Optionally integrate Walrus for storing usage commitments (counter states) for auditability.

## Technical

### Architecture
Client → generate ZK proof (counter ≤ limit)
   ↓
Soundness Layer → verify proof
   ↓
Sui Contract → log valid API calls
   ↓
Backend API → serves only if valid flag exists
   ↓
Walrus → optional commitment storage

### Stack
- **Frontend**: React (demo UI for developers)
- **Backend**: Node.js (API gateway), Rust (prover service)
- **Blockchain**: Soundness Layer + Sui
- **Storage**: Walrus (commitments), local DB (counters)

### Features
1. ZK proof of API quota compliance.
2. Trustless verification via Soundness Layer.
3. Sui contract registry for API call validation.
4. Optional Walrus audit trail for counter commitments.

## Timeline

### PoC (2-4 weeks)
- [ ] Build simple circuit (counter ≤ N)
- [ ] Integrate with SL verifier
- [ ] Deploy basic ThrottleRegistry Move Contract
- [ ] Demo API backend + simple UI

### MVP (4-8 weeks)  
- [ ] Add Walrus commitments
- [ ] Release JS SDK (zkthrottle-client)
- [ ] Expanded Move contract with quota/NFT management
- [ ] Public test with ≥20 users

## Innovation
What makes this unique? Why will people use it?
First ZK-powered rate-limiting system for APIs.
Moves away from centralized API key systems and log-based monitoring.
Practical use-case for developers of AI, DeFi, or any high-demand service needing fair-use access.
Balances privacy + trustless enforcement.

## Contact
Preferred: Discord (alzion_7)


**Checklist before submitting:**
- [x] All fields completed
- [x] GitHub username matches PR author  
- [x] SL integration explained
- [x] Timeline is realistic
