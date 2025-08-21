# vApp Submission: zk Fair Raffle Using Public Beacons

## Verification
```yaml
github_username: "R1ghTsS"
discord_id: "703264642847342612"
timestamp: "2025-08-21"
```

## Developer
- **Name**: Marvin
- **GitHub**: @R1ghTsS
- **Discord**: @rightsss
- **Experience**: I am a beginner in Web3 programming. I am currently learning zero-knowledge proofs and the Soundness stack. My goal is to gain practical experience by building a simple but unique zk raffle dApp that integrates Walrus storage and the Soundness Layer for decentralized verification.

## Project

### Name & Category
- **Project**: zk Fair Raffle Using Public Beacons
- **Category**: social

### Description
A fair raffle where the winners are computed deterministically from:
`winner_i = SHA256(secret || public_beacon || i) mod N`  
with rejection sampling to avoid duplicates.  
- **Proof**: Computed in SP1 zkVM (export Groth16 BN254) or Ligero.  
- **Storage**: Participants list, public inputs, and proof uploaded to Walrus.  
- **Verification**: Soundness Layer validators fetch blobs, verify proof, and issue attestation.  

### SL Integration  
- **Walrus**: Store participants.csv, proof, and public.json. Validators re-hash CSV to check `H_participants`.  
- **Soundness Layer**: Attest that `{beacon, H_participants, N, K, indices}` was computed correctly.  

## Technical

### Architecture
1. Organizer precommits to participants list and secret.  
2. After cutoff, use unpredictable public beacon (e.g., drand hash).  
3. Prover computes K unique winner indices and generates proof.  
4. Proof + public.json uploaded to Walrus.  
5. Soundness Layer attests proof and outputs attestation ID.  

## Stack
- **Front End** React & Vite
- **SP1 zkVM** (Rust) → Groth16 BN254 proof export.  
- **Ligero** alternative (lighter, no on-chain verifier).  
- **Walrus CLI** for blob storage.  
- **Soundness CLI** for attestation submission. 

### Features
- Trustless raffle with public randomness.  
- Unique winners (no duplicates).  
- Decentralized attestation for fairness.  

## Timeline

### PoC (2-4 weeks)
- Implement SP1 function (hash + mod + rejection sampling).  
- Generate proof and verify locally.  
- Upload artifacts to Walrus.  
- Submit to Soundness Layer and receive attestation.
- Simple UI

### MVP (4-8 weeks)  
- Add CSV → indices mapping and simple viewer.  
- Automate CLI flow for organizer.  
- Document README with screenshots.  
- Stretch: Merkle-rooted participant list or auto payouts.

## Innovation
- Combines **commit–reveal** with a **public beacon** for fairness.  
- Proof‑agnostic: works with SP1 or Ligero.  
- Fully decentralized attestation and no trust in organizer’s code execution.

## Contact
- Discord: @rightsss  
