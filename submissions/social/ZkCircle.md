# vApp Submission: ZkCircle

## Verification
- **github_username:** "alfi-wqi"  
- **discord_id:** "843639170027094017"  
- **timestamp:** "2025-08-24"  

---

## Developer
- **Name:** Odik Sodikin  
- **GitHub:** @alfi-wqi  
- **Discord:** bakwan#2436  
- **Experience:** 3+ years in web3 development, previously worked on Ethereum-based dApps, zkSNARK integration, and decentralized identity systems.  

---

## Project
- **Name:** ZkCircle  
- **Category:** Social  

### Description  
ZkCircle is a privacy-preserving social network built on decentralized identity.  
- Problem: Current social platforms collect too much personal data and often tie identities to centralized providers. Web3 social apps exist, but most lack privacy.  
- Solution: ZkCircle enables people to connect, post, and join communities while proving credibility and uniqueness through zero-knowledge proofs—without revealing personal information.  
- Value: Users gain freedom of expression and verified trustworthiness without giving up privacy.  

---

## SL Integration
ZkCircle will leverage **Soundness Layer (SL)** for:  
- **zk-identity:** To allow users to prove uniqueness (one-human-one-account) without exposing personal data.  
- **Privacy-preserving messaging:** Direct messages will use SL’s zk-encryption features.  
- **Decentralized reputation:** Reputation scores stored on SL, updatable via zk-proofs of activity and community contribution.  

---

## Technical

### Architecture  
1. **Frontend (Next.js + React):** User-facing interface for feed, posts, profiles, and messaging.  
2. **Backend (Node.js + SL SDK):** Handles SL proof verification, messaging relays, and reputation updates.  
3. **Blockchain (Soundness Layer + Ethereum):**  
   - SL for identity and reputation proofs  
   - Ethereum for optional token-based governance  
4. **Storage (IPFS + WALRUS):** Posts, images, and community content stored off-chain, referenced by SL proofs.  

### Stack  
- **Frontend:** Next.js (React + Tailwind)  
- **Backend:** Node.js + Express + SL SDK  
- **Blockchain:** Soundness Layer + Ethereum  
- **Storage:** WALRUS (structured data) + IPFS (media)  

### Features  
1. **Privacy-preserving social feed:** Users can post and interact pseudonymously while proving uniqueness.  
2. **ZK-verified communities:** Communities can require zk-proofs to join (e.g., proof-of-human, proof-of-membership).  
3. **Decentralized reputation system:** Earn reputation points for posting, engaging, and governance participation.  

---

## Timeline  

### PoC (2-4 weeks)  
- Build minimal social feed (text-only posts)  
- Integrate SL for zk-identity (basic login & uniqueness check)  
- Simple UI for posting & viewing  

### MVP (4-8 weeks)  
- Add comments, likes, and communities  
- Production-ready SL integration (identity + reputation)  
- Encrypted DMs via SL messaging  
- User testing and feedback loops  

---

## Innovation  
- Unlike existing Web3 social platforms, ZkCircle prioritizes **privacy with verifiable trust**.  
- Combines **social interaction + zk-identity** in a way that preserves anonymity but prevents spam/sybil attacks.  
- Users can build **reputation without linking personal data**, enabling safe communities, DAOs, and governance.  

---

## Contact  
Preferred contact method and where you'll share updates.  

---

## Checklist ✅
- [x] All fields completed  
- [x] GitHub username matches PR author  
- [x] SL integration explained  
- [x] Timeline is realistic  
