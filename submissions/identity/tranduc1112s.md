# vApp Proposal – Proof-of-Yield

**Category:** defi  
**GitHub:** https://github.com/tranduc1112s  
**Discord ID:** 812619850189897768  

## 1) Problem & Users
- **Problem:**  
  Yield farmers and lenders want to prove their earnings (for rewards, governance participation, or lending credit scores) without exposing their entire wallet history or sensitive balance data. Current approaches are slow, privacy-invasive, and inefficient.  

- **Users:**  
  - Yield farmers claiming rewards from DAOs or protocols.  
  - DAOs distributing incentives to top-performing users.  
  - Lending protocols evaluating user creditworthiness based on yield performance.  

## 2) Solution Overview
- **Proof-of-Yield** leverages the **Soundness Layer** to generate zero-knowledge proofs of farming/lending earnings.  
- Instead of sharing raw wallet data, users submit a proof + `blob id` that is instantly verifiable on-chain.  

**Key Features:**  
- Privacy-preserving: only yield results are revealed, not wallet history.  
- Fast verification: on-chain proof checks take seconds.  
- Interoperable: DAOs, DeFi apps, and lending platforms can all integrate.  

**Example Use Case:**  
A DAO rewards users who earned at least 100 USDC in yield from a liquidity pool. Instead of analyzing wallet transactions, the DAO requests a proof:  
- User generates proof with `soundness-cli` → “Earn ≥100 USDC from Pool A.”  
- User submits proof to the DAO contract.  
- Contract verifies via Soundness Layer and automatically distributes rewards.  

## 3) Technical Architecture (with Soundness Layer)
- **Frontend:** Next.js dApp for proof generation and submission.  
- **Backend Worker:** Node.js service running `soundness-cli` for proof + `blob id`.  
- **Smart Contract (EVM L2):**  
  - `YieldVerifier` contract to verify proofs and emit events.  
  - DAOs can subscribe to these events to trigger reward distribution.  

**Data Flow:**  
1. User selects a pool and generates proof of yield.  
2. Worker calls `soundness-cli proof generate --data yield.json`.  
3. Proof + `blob id` are returned.  
4. User submits to `YieldVerifier`.  
5. Contract verifies and emits an event.  
6. DAO contracts use the verified event to automate reward distribution.  

## 4) Milestones & Timeline
- **Week 1:** Develop `YieldVerifier` smart contract and mock verification.  
- **Week 2:** Build backend worker with CLI integration.  
- **Week 3:** Develop frontend UI for proof submission.  
- **Week 4:** Launch public testnet demo with sample pools and DAO integration.  

## 5) Team
- Trần Đức – Fullstack / DeFi Developer (solo dev).  

## 6) Risks & Mitigations
- **Complex proof logic:** implement batch verification for scalability.  
- **Blob id errors:** add retries and monitoring.  
- **User adoption challenge:** design a simple wizard flow to generate proofs.  

## 7) Success Metrics
- ≥100 yield proofs generated during testnet.  
- On-chain verification time ≤3 seconds.  
- ≥50 unique users participating in demo pools.  

## 8) Links
- Demo repository / testnet contracts (to be added).
