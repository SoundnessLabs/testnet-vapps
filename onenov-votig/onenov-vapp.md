# OneNov Voting dApp

A **decentralized, verifiable voting dApp** powered by **Soundness Layer**, offering transparent, privacy-preserving governance through ZK proofs.

---

## Overview

**OneNov Voting dApp** enables communities or DAOs to conduct voting that is:
- **Verifiable** — each vote comes with a zero-knowledge proof (`ZKP`)  
- **Private** — results are calculated without revealing voter identity  
- **Immutable** — proofs are stored on Walrus and verified via Soundness Layer  

By leveraging Soundness integration, voting becomes fast, decentralized, and low-cost on-chain.

---

## Soundness Layer Integration

- **Generate proof** when a user votes via Soundness CLI or client-side prover like Ligero  
- **Upload proof** to **Walrus**, then retrieve the `blob ID` as a reference  
- **Submit blob ID** via a smart contract on the Soundness Layer (Move) → verified by Sui validators, with on-chain attestation  
  - This process is inexpensive, fast, and does not require storing full voting data on-chain2  
- The frontend/backend utilizes the proof for transparent and audit-friendly verification

---

## Architecture

[Frontend (React)] ── cast vote → generate ZKP(s) → └─ proof → Walrus → blob ID → Soundness Layer contract (on Sui)

- **Smart Contract**: Move-based (Soundness Layer), receives blob ID and stores attestation
- **Backend/CLI**: generates proof and submits to Soundness
- **Frontend**: interface to create proposals, vote, and display results based on attested proof

---

## Stack

| Layer       | Tech / Tool                                  |
|-------------|----------------------------------------------|
| Frontend    | React + Tailwind                             |
| Proof       | Soundness CLI / Ligero                       |
| Storage     | Walrus (decentralized data)                  |
| Blockchain  | Soundness Layer contract (Move on Sui)       |
| dApp Host   | Soundness Layer Testnet (developer access)   |

---

## Features

1. **Submit voting proposal** on the frontend  
2. **Cast vote → generate ZKP → store to Walrus**  
3. **Submit blob ID to Soundness Layer**, store attestation on-chain  
4. **View tally & proof status**, transparent and auditable by anyone

---

## Timeline

**PoC (2-4 weeks)**  
- Integrate Soundness CLI & generate proof  
- Store proof on Walrus → retrieve blob ID  
- Submit to Soundness Layer contract + simple React UI  

**MVP (4-8 weeks)**  
- Add multi-choice voting  
- Auth (Discord/Github) + UI polish  
- User testing & proof explorer

---

## Contact

- **Website**: [onenov.xyz](https://onenov.xyz)  
- **GitHub**: [@OneNov0209](https://github.com/OneNov0209)  
- **Discord**: suryaone#6296 (ID: 956188958071607348)  
- **Telegram**: [t.me/OneNov02](https://t.me/OneNov02)

---

## Why Soundness?

- **Fast finality & low cost**: ideal for real-time voting  
- **Decentralized & censorship-resistant**: proofs are not controlled by a single party3  
- **Cross-chain readiness**: proofs can be verified across blockchains  
- **Proof auditability**: all votes can be checked by third parties without compromise
