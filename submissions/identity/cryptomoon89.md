# Project Proposal: zkProof-of-Human (zkPoH)

## Category
identity

---

## Description

zkProof-of-Human (zkPoH) is a privacy-preserving identity verification application powered by zero-knowledge proofs. It enables users to prove they are a unique human—without disclosing personal information such as email, phone number, or biometrics.

It solves key problems such as:  
- Preventing Sybil attacks in DAO voting  
- One-person-per-airdrop filtering  
- On-chain KYC-compatible proof without data retention  

---

## Soundness Layer Integration

The Soundness Layer will be integrated as the trustless backend for proof verification and registry syncing.  

Key SL usage includes:  
- Submitting zkProofs to SL nodes for off-chain verification  
- Referencing the verified proof hash on-chain via contract bridge  
- Using SL SDK to streamline proof submission and integrity guarantees  

---

## Technical Architecture

### Stack

- **Frontend:** Next.js, wagmi, ethers.js  
- **Backend:** Node.js (optional for webhook/API)  
- **Blockchain:** Soundness Layer + Ethereum (Sepolia)  
- **Storage:** Off-chain registry, minimal on-chain hash mapping  

### Features

- **Zero-knowledge circuit** that validates user uniqueness/liveness  
- **PoHRegistry smart contract** to store verified proof hashes  
- **Soundness SDK** integration for seamless proof flow  
- **Real-time event tracking** via GraphQL subgraph and listener  
- **Captcha & entropy input** to improve proof robustness  

---

## Timeline

| Phase | Weeks | Deliverables |
|-------|--------|--------------|
| PoC   | 2–4    | Circuit, Verifier contract, Basic UI, SL integration |
| MVP   | 4–8    | Full functionality, Frontend deployed, Subgraph & monitoring, User testing |

---

## Innovation

zkPoH enables decentralized identity without compromising privacy. Unlike traditional KYC systems, it stores no user data. It is fully zk-based and uniquely leverages Soundness Layer to achieve scalable, censorship-resistant human verification usable across DAOs, airdrops, and decentralized apps.

---

## Contact

- **GitHub:** https://github.com/cryptomoon89  
- **Discord:** 461199708036136961  
- **Updates:** Project updates will be posted via GitHub and Discord  

---

## Checklist

- [x] All fields completed  
- [x] GitHub username matches PR author  
- [x] SL integration explained  
- [x] Realistic timeline included  

---

**GitHub Username:** cryptomoon89  
**Discord ID:** 461199708036136961  
