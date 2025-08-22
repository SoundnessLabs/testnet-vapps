# vApp Submission: zkFreelanceProofs

## Verification
```yaml
github_username: "dandyds"
discord_id: "396156237672218626"
timestamp: "2025-08-22"
```
## Developer
- **Name**: Dandy Dwi Septiadi
- **GitHub**: @DBSFoundation
- **Discord**: dandyds
- **Experience**: Crypto native since 2013 and Full Stack Web Developer. Currently learning Rust and Web3 development, with a focus on building zkApps and verifiable, privacy-preserving systems.

---

## Summary

zkFreelanceProofs is a **verifiable credentials application for freelancers**.  
It allows freelancers to prove their portfolio and achievements through **zero-knowledge proofs (ZKPs)** without exposing sensitive details such as client data, project files, or contract terms.  

---

## Motivation

- **Current problem**: Freelancers often struggle to prove their skills or experience objectively. Screenshots, portfolio links, or testimonials can be faked or may reveal private client information.  
- **Solution**: With ZK proofs, achievements can be validated (e.g., certifications, completed projects, average ratings, income milestones) without leaking the underlying data.  
- **Value add**: Builds trust in the freelancing market in a privacy-preserving, secure, and on-chain verifiable way.  

---

## Design

1. **Proof Generation**  
   - Freelancers upload credential evidence (e.g., certificates, project ratings, payment receipts).  
   - The app generates a commitment hash and a ZK proof attesting to validity without revealing raw details.  

2. **Storage & Verification**  
   - Proofs are stored on **Walrus**.  
   - Verification is performed via the **Soundness Layer** testnet using Sui smart contracts.  

3. **Client Interaction**  
   - Clients can check if a freelancer meets certain claims (e.g., “Completed ≥ 20 projects with rating ≥ 4.5/5”).  
   - Clients see only the verified proof, never the sensitive underlying data.  

---

## Example Use Case

- **Alice** is a freelance graphic designer.  
- She has completed 50 projects with an average rating of 4.8.  
- Using zkFreelanceProofs, Alice can prove the claim “≥ 40 completed projects with rating ≥ 4.5” through a ZK proof, without revealing project details or client names.  
- **Bob**, a potential client, verifies Alice’s proof on the Soundness testnet and can trust her claim with confidence.  

---

## Technical Details

- **Proof System**: Ligero / other zk scheme supported by Soundness  
- **Storage**: Walrus for proof persistence  
- **Verification**: Sui contract integrated with Soundness Layer testnet  
- **Frontend**: Simple dashboard for freelancers to generate proofs & share verifiable credential links  

---

## Roadmap

**Phase 1 – PoC (2-3 weeks)**  
- Implement a basic proof of “number of completed projects ≥ N”.  
- Store proof in Walrus, verify on testnet.  
- Basic UI for generating and sharing proofs.  

**Phase 2 – Expansion (3-4 weeks)**  
- Add more proof types (ratings, income milestones, certifications).  
- Build a client dashboard for verification.  
- Improve UX with one-click proof sharing.  

**Phase 3 – Scaling (future)**  
- Integrate with existing freelance platforms.  
- Build standardized zkCredential schemas for broader adoption.  

---

## Expected Impact

- Demonstrates how ZKPs can enhance **trust & privacy** in digital work.  
- Opens pathways to similar applications in **education (degrees/certificates)**, **employment (work history)**, and **social reputation**.  
- Pushes zkApps closer to real-world adoption.  
