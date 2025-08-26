# vApp Submission: Daily Check-in Badge

## Verification
```yaml
github_username: "0xiChl"
discord_id: "367628501123334144"
timestamp: "2025-08-27"
```

## Developer
- **Name**: 0xiChl  
- **GitHub**: @0xiChl  
- **Discord**: 0xiChl  
- **Experience**: Beginner in Web3, currently learning through testnet participation and simple vApp ideas.

## Project

### Name & Category
- **Project**: Daily Check-in Badge  
- **Category**: social  

### Description
A simple vApp that rewards users with a daily badge whenever they check in by connecting their wallet.  
Each check-in generates a proof through Soundness Layer, and the badge can be minted if the proof is valid.  
This system encourages community engagement and gamifies daily participation.

### SL Integration
- User connects wallet and requests a “check-in proof.”  
- Proof is validated by Soundness Layer verifier contract.  
- If valid, the user can mint a daily badge NFT (soulbound).  

## Technical

### Architecture
1. User opens app and connects wallet.  
2. Frontend triggers check-in request → sends proof to SL verifier.  
3. Verifier Contract validates daily proof.  
4. Badge Contract mints a new badge for that day.  

### Stack
- **Frontend**: Simple React/Next.js app with wallet connect  
- **Backend**: Optional (for logging streaks), but not required  
- **Blockchain**: Soundness Layer testnet  
- **Storage**: IPFS for badge metadata  

### Features
1. Daily check-in with wallet  
2. Badge minting per check-in (soulbound, non-transferable)  
3. Calendar-style UI to show streaks  

## Timeline

### PoC (2-4 weeks)
- [ ] Basic check-in flow (wallet connect → proof → mint badge)  
- [ ] Soulbound badge contract  
- [ ] Simple frontend (1-button check-in)  

### MVP (4-8 weeks)
- [ ] Streak tracking (7 days, 30 days, etc.)  
- [ ] Leaderboard or community showcase  
- [ ] Polished frontend with calendar view  

## Innovation
This vApp gamifies participation: by checking in daily, users collect on-chain proof of activity.  
It shows how Soundness Layer can power lightweight proofs of action (daily activity) without complex logic.

## Contact
- **Preferred**: Discord @0xiChl  
- Updates will be shared on GitHub (pull requests, issues).
