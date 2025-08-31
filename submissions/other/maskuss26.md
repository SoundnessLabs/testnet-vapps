# Project: zkCryptoNews

## TL;DR
A decentralized crypto news portal leveraging the Soundness Layer to verify the authenticity of news sources on-chain.  
Users can read news with a “verified” status, reducing hoaxes and misinformation.

---

## Problem
The crypto industry moves very fast and is prone to misinformation. Fake or manipulative news often spreads across social media and unverified blogs.  
There is no easy way for readers to confirm if the news actually comes from credible sources.

---

## Solution
Build a crypto news aggregator that:
- Fetches news from trusted sources (CoinDesk, CoinTelegraph, etc.).
- Generates a hash of the news content along with the publisher’s signature.
- Submits the proof to the **Soundness Layer** for verification.
- Stores the verification results in a smart contract.
- Displays news with a **Verified / Unverified** status on the dApp UI.

---

## Architecture
- **Frontend dApp (Next.js/React):** User interface for reading news & verification status.  
- **Aggregator Service (Node.js):** Fetches news from APIs → generates hashes.  
- **Soundness Layer Proof:** Sends proof to SL to verify hash & signature.  
- **Smart Contract (EVM compatible):** Stores news hashes & verification results.  
- **UI Layer:** Displays crypto news with verification indicators.

---

## SL Integration
- Proof submission: news content hash is submitted to the SL.  
- Verification by SL: only news with valid signatures from trusted publishers are approved.  
- Callback to smart contract: updates the “Verified” status.  
- Frontend dApp displays only news with a valid status, so users can trust the content.

---

## Milestones & Timeline
- **Week 1:** Setup frontend dApp + backend aggregator (fetching APIs like CoinDesk).  
- **Week 2:** Generate content hashes & implement proof submission to the Soundness Layer.  
- **Week 3:** Deploy smart contract to store verification results.  
- **Week 4:** Integrate frontend with the smart contract + display “verified” status.  
- **Week 5:** Test the PoC on testnet + finalize documentation.  

---

## Team
- **GitHub:** @maskuss26  
- **Discord:** mymass  

---

## Risks & Mitigations
- **API downtime** → use multiple providers & caching.  
- **Large proof sizes** → optimize with hash batching.  
- **Publishers not yet supporting signatures** → temporarily rely on hashing & a whitelist of trusted media outlets.  
