# vApp Submission: beermoneycrypto-dice

## Verification
- github_username: "renaldhy07"
- discord_id: "andirenaldi"
- timestamp: "2025-08-27"

## Developer
- Name: Andi Renaldi
- GitHub: @renaldhy07
- Discord: andirenaldi
- Experience: Web3 enthusiast with background in smart contract development and decentralized applications.

## Project
### Name & Category
- Project: beermoneycrypto-dice  
- Category: gaming  

### Description
beermoneycrypto-dice is an on-chain dice game that guarantees fairness and transparency using zkProofs.  
It solves the common problem of *trust* in online gambling, where users often cannot verify if dice results are manipulated.  
By leveraging the Soundness Layer, randomness can be proven and verified publicly.

---

## SL Integration
- Use **SP1 zkVM** to generate random numbers with verifiable proofs.  
- Soundness Layer validates the zkProofs to ensure dice results are fair.  
- Store proof outputs and game state on **Walrus storage** for availability.  
- Only verified results from SL are accepted by the game’s smart contract.

---

## Technical

### Architecture
1. Player interacts with frontend (React web app).  
2. Transaction goes to the smart contract on Sui blockchain.  
3. RNG generated inside SP1 zkVM → produces dice result + proof.  
4. Proof is sent to Soundness Layer for verification.  
5. Verified result stored on Walrus and returned to smart contract.  
6. Contract executes payout based on result.  

### Stack
- **Frontend**: React  
- **Backend**: Node.js  
- **Blockchain**: Sui + Soundness Layer  
- **Storage**: Walrus  

### Features
- Fair dice roll with zkProof RNG  
- On-chain betting & payout  
- Transparent verification via Soundness Layer  

---

## Timeline
### PoC (2–4 weeks)
- Implement smart contract for dice betting  
- Integrate SP1 zkVM RNG + proof generation  
- Basic React UI for gameplay  

### MVP (4–8 weeks)
- Full gameplay features (bet sizes, multiple players)  
- Full SL integration (Walrus storage + verification)  
- User testing and feedback loop  

---

## Innovation
Unlike centralized dice apps, beermoneycrypto-dice provides **provably fair gameplay**.  
Users don’t have to trust the developer – fairness is mathematically guaranteed by zkProofs and Soundness Layer verification.  
This builds trust in Web3 gaming and opens the door for scalable, transparent gambling applications.

---

## Contact
- Preferred contact: Discord (andirenaldi)  
- Updates will be shared via GitHub and Discord community.  

---

## Checklist
- [x] All fields completed  
- [x] GitHub username matches PR author (`renaldhy07`)  
- [x] SL integration explained  
- [x] Timeline is realistic  
