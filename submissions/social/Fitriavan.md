# vApp Proposal: Decentralized Community Reputation System

## Category
social

## Description
This vApp introduces a decentralized reputation system for online communities.  
Users can generate zkProofs of their participation (e.g., messages, contributions, or activity logs) and verify their reputation without exposing raw data.  
This ensures fairness, privacy, and verifiability for community engagement.

## Technical Architecture
- Community activity data is fetched (from APIs or provided logs).  
- Data is hashed and verified using **Soundness CLI**.  
- zkProofs are generated to confirm authenticity of user contributions.  
- Reputation points are calculated off-chain, while proofs are verifiable on **Soundness Layer**.  
- Future extension: integration with Discord bots for automatic proof submission.

## Timeline
- **Week 1:** Setup repo & connect Soundness CLI with mock data  
- **Week 2:** Implement basic proof generation & reputation scoring  
- **Week 3:** MVP: verify user contributions with Soundness Layer  
- **Week 4:** Deployment on testnet + simple dashboard for reputation view  

## GitHub Username
Fitriavan

## Discord ID
fitriavan
