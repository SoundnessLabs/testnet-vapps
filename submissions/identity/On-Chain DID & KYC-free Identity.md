# vApp Submission: On-Chain DID & KYC-free Identity

## Verification
```yaml
github_username: "atyadamara"
discord_id: "852578313448325141"
timestamp: "2025-08-31"
```

## Developer
- Name: abrak
- GitHub: @abrak2
- Discord: abrak_
- Experience: Experienced Web3 developer specializing in decentralized identity, privacy-preserving protocols, and zk-SNARK circuits. Previously built dApps in the DeFi and DAO space, contributed to open-source identity frameworks, and conducted research on zero-knowledge-based authentication systems. Strong background in smart contract development, cryptographic proof systems, and scalable blockchain integrations.

## Project

### Name & Category
- Project: On-Chain DID & KYC-free Identity
- Category: identity

### Description
On-Chain DID & KYC-free Identity provides a self-sovereign digital identity system that allows users to prove uniqueness, age, and residency without exposing personal data.
By leveraging decentralized identifiers (DIDs) and zero-knowledge proofs, users can authenticate across dApps, DAOs, and DeFi protocols while maintaining privacy and avoiding repetitive KYC checks.

### SL Integration
- Anchor user-issued DIDs on-chain with SL smart contracts.
- Store encrypted identity attestations on SL storage.
- Generate zk-proofs for conditions like uniqueness, age > 18, or residency.
- Allow dApps to verify proof validity without accessing raw documents.

## Technical

### Architecture
- DID Creation: User generates decentralized identifier → anchored on SL.
- Proof Generation: User creates zk-SNARK proof for required conditions (age, residency, uniqueness).
- SL Smart Contract: Binds DID and proof validity to user wallet.
- dApp Verification: Any dApp can query the SL contract to validate proofs instantly.

### Stack
- Frontend: React + Next.js
- Backend: Node.js / Express
- Blockchain: Soundness Layer + Solidity smart contracts
- Identity: W3C DID standard + zk-SNARK circuits
- Storage: SL encrypted storage + IPFS/WALRUS anchoring

### Features
- DID-based identity, reusable across multiple ecosystems.
- KYC-free onboarding using zk-proofs (no document leaks).
- Sybil resistance with one-human-one-wallet enforcement.
- Selective disclosure: only reveal necessary facts (e.g., "over 18").

## Timeline

### PoC (2-4 weeks)
- [ ] Smart contract for DID anchoring + zk-proof of uniqueness
- [ ] Basic SL integration for encrypted storage
- [ ] UI for DID creation and zk-proof generation

### MVP (4-8 weeks)  
- [ ] Extended proof circuits (age, residency, uniqueness)
- [ ] Full SL contract integration with dApp APIs
- [ ] Partner testing with DAO/DeFi identity gating use cases

## Innovation
Unlike traditional identity/KYC systems, this project eliminates the need for centralized verification.
Users fully control their DID, selectively disclose attributes, and avoid repetitive KYC processes.
By combining decentralized identifiers with Soundness Layer’s zk-verification, the system enables privacy-preserving, regulatory-friendly, and user-sovereign identity in Web3.

## Contact
- Discord: keju3564  
- GitHub: @atyadamara
-

---

Checklist before submitting:
- [x] All fields completed  
- [x] GitHub username matches PR author  
- [x] SL integration explained  
- [x] Timeline is realistic  
