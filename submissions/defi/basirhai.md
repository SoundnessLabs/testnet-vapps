### Description
zkMicrogrants is a platform for transparent but privacy-preserving micro-funding.  

- Donors fund grant pools on-chain.  
- Builders request funds by submitting a ZK proof that:  
  - They deployed code/content/resources matching their proposal.  
  - They used funds within the rules (without revealing private details).  
- Soundness Layer verifies proofs, ensuring correct usage before funds are released.  

---

### SL Integration
- Use **Soundness CLI** to generate proofs that link grant requests to on-chain actions.  
- Verify **ZK proofs** on the **Soundness Layer**, ensuring fund usage rules are followed.  
- Store proof blobs using **Walrus** for auditability and transparency.  
- Integrate with **Sui blockchain** for grant escrow and automated disbursement once proofs are verified.  

---

## Technical

### Architecture
1. **Frontend (React dApp)** → UI for donors to fund pools and builders to request microgrants.  
2. **Backend (Rust/Node.js API)** → Handles grant applications, proof submission, and integration with Soundness CLI.  
3. **Soundness Layer** → Verifies proofs of grant usage and ensures compliance with funding rules.  
4. **Walrus Storage** → Stores proof blobs for external verification and auditability.  
5. **Sui Blockchain** → Manages on-chain escrow contracts and disburses funds once proofs are valid.  

### Stack
- **Frontend**: React + TailwindCSS  
- **Backend**: Node.js (Express) or Rust (Axum)  
- **Blockchain**: Soundness Layer + Sui  
- **Storage**: Walrus (for proof blobs) + PostgreSQL (for metadata)  

### Features
1. Create and fund microgrant pools (donor side).  
2. Submit ZK proof of deliverables to request payout (builder side).  
3. Automatic on-chain verification + escrow release when proofs are valid.  

---

## Timeline

### PoC (2-4 weeks)
- [ ] Smart contract for grant escrow  
- [ ] Soundness CLI integration (basic proof flow)  
- [ ] Simple React UI for donors + builders  

### MVP (4-8 weeks)
- [ ] Full feature set (funding pools, requests, disbursement)  
- [ ] Walrus proof storage integration  
- [ ] User testing + feedback cycle  
- [ ] Deploy on Soundness testnet  

---

## Innovation
- Brings **trustless transparency** to micro-funding while preserving privacy.  
- Empowers small builders to raise funds without exposing sensitive details.  
- Uses **Soundness Layer proofs** to guarantee compliance with funding rules, making it safer for donors.  
- Bridges **DeFi + ZK + public goods funding** in a unique way.  

---

## Contact
- Discord: `basirhai`  
- GitHub: `basirhai`  
- Updates shared via: GitHub repo + Soundness Discord  

---

**Checklist before submitting:**  
- [x] All fields completed  
- [x] GitHub username matches PR author  
- [x] SL integration explained  
- [x] Timeline is realistic  
