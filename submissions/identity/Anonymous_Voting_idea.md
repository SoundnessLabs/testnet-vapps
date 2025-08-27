# vApp Submission: Verifiable Anonymous Voting

## Verification

```yaml
github_username: "istekhar8966"
discord_id: "1144074814672343111"
timestamp: "2025-08-27"
```

## Developer
- **Name**: Md Istekhar
- **GitHub**: @istekhar8966
- **Discord**: jiot425#1144
- **Experience**: 4+ years in Web3 development, specializing in blockchain protocols, zero-knowledge proofs, and decentralized privacy-preserving applications

## Project

### Name & Category
- **Project**: Verifiable Anonymous Voting
- **Category**: privacy, governance

### Description
Anonymous voting dApp allowing users to join polls and cast votes without revealing their identity. Each vote is validated and attested using Soundness Layer’s zk proof system, ensuring tamper-proof and privacy-preserving verifiable results.

### SL Integration
Uses Soundness Layer for zk attestation of votes. SL smart contracts coordinate vote submissions, store proofs cost-effectively on Walrus, and provide chain-agnostic, publicly verifiable results while preserving voter anonymity.

## Technical

### Architecture
Soundness Layer smart contracts act as the voting coordinator. Votes are submitted via zk proofs; Walrus stores proofs; SL validators verify, attest, and tally results.

### Stack
- **Frontend**: React + TypeScript
- **Backend**: Node.js + Express
- **Blockchain**: SL Testnet + Sui
- **Storage**: Walrus (for proofs), PostgreSQL (for poll metadata)

### Features
1. Anonymous, Sybil-resistant voting  
2. zk proof-based verifiable results  
3. Poll creation + live vote tally  
4. API for integration with existing communities  

## Timeline

### PoC (3 weeks)
- [x] SL vote contract  
- [x] Basic frontend + CLI  
- [ ] End-to-end zk vote attestation  

### MVP (6 weeks)
- [ ] Multi-poll support  
- [ ] Discord/Telegram integration  
- [ ] Mobile-friendly UI  

## Innovation
First simple, privacy-first governance app using Soundness Layer’s zk proof attestation—verifiable, tamper-proof voting with real-world use for DAOs and communities.

## Contact
```yaml
Gmail: istekhar8966@gmail.com
Discord: jiot425
```

