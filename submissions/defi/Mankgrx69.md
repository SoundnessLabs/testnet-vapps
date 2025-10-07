# vApp Submission: SoundVault

## Verification
```yaml
github_username: "Mankgrx69"
discord_id: "881554897839988746"
timestamp: "2025-10-07"
```

## Developer
- **Name**: Shinji
- **GitHub**: @Mankgrx69
- **Discord**: yhwach#7704
- **Experience**: Web3 user, DeFi explorer, and early builder focused on zk reputation, soundness proofs, and testnet-layer experimentation.

## Project

### Name & Category
- **Project**: SoundVault
- **Category**: defi

### Description
SoundVault is a decentralized vault and lending system built on the Soundness Layer.
It uses zk reputation proofs to let users borrow assets based on verified credibility instead of over-collateralizing their funds.

The main problem in DeFi lending is that users need to lock huge amounts of collateral, making it inefficient and unattractive for small participants.
SoundVault fixes that by allowing verified reputation (through Soundness proofs) to act as trust signals.
Higher reputation means higher borrowing power, while privacy remains protected.

### SL Integration  
SoundVault connects directly with the Soundness Layer for:

Generating zk proofs using the Soundness CLI.  
Verifying proofs on-chain through the Soundness SDK.  
Recording loan-related attestations in the Soundness Layer for transparency.  
Running a local CLI demo that simulates proof creation, verification, and borrowing flow.

## Technical

### Architecture
A user creates a zk reputation proof with the Soundness CLI.  
The proof is sent to the VaultManager smart contract.  
The contract verifies the proof using a Soundness verifier adapter.  
Once verified, the contract assigns a borrowing limit based on the user’s reputation class (A/B/C).  
The transaction and attestation record are stored in the Soundness Layer for auditability.

### Stack
- **Frontend**: Next.js with Ethers.js  
- **Backend**: Node.js for CLI automation and RPC calls  
- **Blockchain**: Ethereum Sepolia + Soundness Layer  
- **Storage**: IPFS for proof and metadata

### Features
1. Reputation-based lending where credibility defines loan size  
2. Private multi-asset vault for ETH and ERC-20 tokens  
3. Integration with Soundness proof verification  
4. CLI demo that walks through proof → verify → borrow  

## Timeline

### PoC (2-4 weeks)
The Proof of Concept will focus on getting the base system running and verifying simple proofs.

- [x] Create and deploy basic VaultManager contract for deposits and withdrawals  
- [x] Integrate Soundness CLI for generating mock reputation proofs  
- [ ] Build simple CLI interface to simulate the full borrow flow  
- [ ] Run local testing on Sepolia to confirm proof verification works  
- [ ] Prepare PoC report for Soundness team review  

### MVP (4-8 weeks)
The MVP will expand the core features and integrate fully with the Soundness Layer.

- [ ] Build minimal Next.js dashboard for deposits, borrowing, and repayment  
- [ ] Replace mock verifier with real Soundness Layer SDK integration  
- [ ] Add multiple reputation classes (A, B, C) that affect loan-to-value (LTV) ratios  
- [ ] Support multiple tokens (ETH, USDC, and custom ERC-20s)  
- [ ] Deploy to testnet and register proof events in the Soundness Layer  

---

## Innovation
SoundVault introduces the concept of **reputation as credit** in DeFi.  
Instead of users proving trust with locked funds, they prove it with zk-attested credibility verified by the Soundness Layer.  

This approach reduces over-collateralization, keeps sensitive data private, and demonstrates how zk proofs can work in real lending environments.  
It’s a unique step toward privacy-first financial identity systems — allowing DeFi to evolve from collateral-based to reputation-based trust.

Users will use SoundVault because:  
- It’s simpler and cheaper to borrow  
- It rewards consistent, positive on-chain behavior  
- It maintains full user privacy through zk verification  

---

## Contact
**Discord:** yhwach#7704  
**GitHub:** [@Mankgrx69](https://github.com/Mankgrx69)  
**Preferred Contact:** Direct message on Discord or tag in Soundness Dev Playground  
**Updates:** Progress and demo results will be shared in GitHub commits and Discord threads during PoC and MVP phases  

---

**Checklist before submitting:**  
- [ ] All fields completed  
- [ ] GitHub username matches PR author  
- [ ] SL integration explained  
- [ ] Timeline is realistic  
