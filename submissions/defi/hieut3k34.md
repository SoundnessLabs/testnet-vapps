vApp Submission: YieldX

Verification
 github_username: "hieut3k34"
 discord_id: "886785911629692979"
 timestamp: "2025-08-29T08:00:00Z"

Developer
 Name: Hieu
 GitHub: @hieut3k34
 Discord: hieut3k
 Experience: Smart contract developer, previously contributed to DeFi protocols on Cosmos and Ethereum, Node.js

Project
Name & Category
 Project: YieldX
 Category: DeFi
 
Description
 YieldX is a cross-chain yield optimizer that helps users automatically find the best staking/lending returns across multiple chains. It solves the problem of fragmented DeFi opportunities by aggregating yields and routing liquidity intelligently.
 
SL Integration
 Cross-chain yield verification via Soundness Layer
 SL oracle feeds for interest rates and staking rewards
 Secure bridging of assets with SL proofs

Technical
Architecture
 graph TD
  A[Frontend Next.js] --> B[Backend Rust Service]
  B --> C[SL for cross-chain proofs]
  B --> D[(WALRUS Storage)]
  C --> E[Cosmos/EVM DeFi protocols]

Stack
 Frontend: Next.js + Tailwind
 Backend: Rust (Actix)
 Blockchain: SL + Cosmos + Ethereum L2
 Storage: WALRUS

Features
 Cross-chain yield aggregator
 Automated liquidity routing
 SL-based yield verification proofs

Timeline
PoC (3 weeks)
 Basic yield aggregation on 2 chains
 SL proof-of-yield integration
 Simple dashboard
 
MVP (6 weeks)
 Multi-chain yield routing
 User vaults
 Governance + incentives

Innovation
 YieldX is the first DeFi app with SL-verified yield proofs, ensuring users can trust reported APYs across chains.

Contact
Discord: hieut3k
Updates: GitHub project repository + community Discord

Checklist before submitting:
- [x] All fields completed  
- [x] GitHub username matches PR author  
- [x] SL integration explained  
- [x] Timeline is realistic  
