App Submission: zkVoice – Private & Fair Social Voting

Verification
github_username: "anakmmhh"
discord_id: "923221612147388416"
timestamp: "2025-09-02"

Developer
Name: anakmmhh
GitHub: @anakmmhh
Discord: anakmmhh
Experience: Web3 developer with focus on decentralized social apps, zkSNARK circuits, and DAO tools.

Project
Name & Category
Project: zkVoice – Private & Fair Social Voting
Category: social

Description
zkVoice is a governance and social voting platform for communities and DAOs.  
It enables members to participate in polls and decision-making with **privacy-preserving proofs**, ensuring fairness and transparency.  
Problem solved: most on-chain voting is public (no privacy) or easily manipulated. zkVoice provides **private yet verifiable voting**, preventing Sybil attacks and ensuring trust in community decisions.

SL Integration
- SL verifies zkProofs of votes without exposing voter identity.  
- Prevents double voting while maintaining anonymity.  
- Features used: proof verification, identity binding, Sybil resistance.

Technical
Architecture
Flow: User casts vote → zk circuit generates proof → SL verifies proof → Vote counted in aggregate → Results published on dashboard.

Stack
- Frontend: React + Tailwind (voting UI)  
- Backend: Node.js + zk circuits (vote proof generation)  
- Blockchain: Soundness Layer (testnet), optional Ethereum for DAO integration  
- Storage: IPFS for storing proof metadata and results  

Features
- Core Feature 1: Private voting with zkProofs.  
- Core Feature 2: Sybil-resistant community governance.  
- Core Feature 3: Public results dashboard with verifiable proof trail.  

Timeline
- PoC (2-4 weeks)  
  - Basic voting system  
  - zk circuit for single-choice voting  
  - SL proof verification integration  
  - Simple results UI  

- MVP (4-8 weeks)  
  - Multi-choice voting  
  - Sybil resistance integration  
  - Results explorer + public verification  
  - User testing with DAOs/communities  

Innovation
- Unique: combines **social voting + zkProofs** for fairness and privacy.  
- Brings trust to DAOs, communities, and social groups.  
- Can extend to reputation-based governance and cross-platform integrations.

Contact
- Preferred: Discord DM (@anakmmhh)  
- Updates: GitHub repo (public) + Soundness Layer Discord.
