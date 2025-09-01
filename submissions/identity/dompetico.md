# vApp Submission: Proof-of-Participation Badge

## Verification
```yaml
github_username: "dompetico"
discord_id: "915990315519836221"
timestamp: "2025-09-01"
```

## Developer
- **Name**: Kiki
- **GitHub**: @dompetico
- **Discord**: dompetico#1058
- **Experience**: Fullstack dev, 2+ years in Web3 & smart contracts

## Project

### Name & Category
- **Project**: PoP Badge vApp
- **Category**: identity

### Description
Communities need a simple way to verify authentic participation in events.
This vApp uses Soundness Layer (SL) to validate proofs and issue non-transferable Proof-of-Participation (PoP) badges.

### SL Integration  
Use SL proof verification endpoint
Send { proof, context } to check validity
On success, mint PoP badge (PoC = in-memory)

## Technical
1. User submits proof + wallet
2. Backend verifies proof with SL
3. Badge issued & stored (PoC only in-memory)

### Architecture
High-level system design and approach

### Stack
- **Frontend**: React (future MVP)
- **Backend**: Node.js/Python 
- **Blockchain**: Soundness Layer testnet
- **Storage**: In-memory (PoC), IPFS/WALRUS later

### Features
1. Claim participation badge with valid proof
2. Retrieve badges by wallet
3. PoC demo with REST API

## Timeline

### PoC (2-4 weeks)
Minimal API, SL integration, claim badge endpoint

### MVP (4-8 weeks)  
Persistent storage
Basic frontend explorer
User testing

## Innovation
PoP badges are non-transferable & lightweight, enabling communities to reward verified participation without heavy infra.

## Contact
Discord DM: dompetico#1058
GitHub Issues


**Checklist before submitting:**
- [ ] All fields completed
- [ ] GitHub username matches PR author  
- [ ] SL integration explained
- [ ] Timeline is realistic