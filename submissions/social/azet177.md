### vApp Proposal: EchoProof — Verifiable On-Chain Voice

**GitHub Username**: Azet17
**Discord ID**: 462592550402916352  
**Category**: social  

---

#### Technical Architecture
EchoProof is a social vApp that enables users to **prove the authenticity of their voice or audio content** using the Soundness Layer without exposing the raw recording.  
- **Core Idea**: Convert short audio clips into zk-proofs that verify *ownership + timestamp* while keeping the content private.  
- **Frontend**: Next.js + Web Audio API — user records or uploads audio directly in the browser.  
- **Backend**: Node.js + IPFS integration for encrypted audio storage.  
- **Integration**:  
  - Soundness CLI generates zero-knowledge proofs tied to the audio hash.  
  - Proofs are verified on Soundness Layer, ensuring the clip is original and unaltered.  
  - Users can share the proof hash as a "badge of authenticity" across social platforms.  
- **Flow**:  
  1. User records voice → system hashes and encrypts.  
  2. Soundness Layer generates zk-proof of authenticity.  
  3. User receives a verifiable "EchoProof" badge (NFT or simple proof token).  
  4. Anyone can verify authenticity without hearing the actual voice file.

---

#### Development Timeline
- **Week 1**: Setup repo, integrate Soundness CLI, basic audio recording UI.  
- **Week 2**: Build hash + encryption module, store encrypted audio on IPFS.  
- **Week 3**: Implement zk-proof generation & verification flow with Soundness Layer.  
- **Week 4**: Deploy proof badge system, run end-to-end demo, document.  

---

#### Expected Impact
- **Combat AI Deepfakes**: Users can prove their voice clips are genuine without exposing raw data.  
- **New Layer of Trust**: Adds privacy-preserving verification for podcasters, streamers, and communities.  
- **Social Utility**: Proof badges can be embedded into posts or shared in Discord/Twitter to authenticate identity.  
- **Novel Showcase**: Demonstrates Soundness Layer’s ability to extend beyond finance into social credibility and content authenticity.

---
