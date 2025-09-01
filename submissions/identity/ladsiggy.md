# vApp Submission: Proof of Community Contribution (PoCC)

## Verification
```yaml
github_username: "ladsiggy"
discord_id: "845394209346551843"
timestamp: "2025-09-01"
```

## Developer
- **Name**: Iggy Amanda  
- **GitHub**: @ladsiggy  
- **Discord**: ladsiggy#0000  
- **Email**: ladsiggy@gmail.com  
- **Experience**: Open source contributor with experience in web development, blockchain integrations, and community management.  

## Project

### Name & Category
- **Project**: Proof of Community Contribution (PoCC)  
- **Category**: identity / social / infrastructure  

### Description
PoCC is a vApp powered by zk-proofs that allows users to prove their community contributions without revealing private data.  
Examples:  
- A developer can prove they have merged ≥5 pull requests on GitHub without exposing the repository details.  
- A community member can prove event participation on Discord without exposing dates or locations.  

The result is a **zk-credential** that is portable across platforms for reputation, voting, hiring, and community participation.  

### SL Integration  
- Using **Soundness Layer CLI** to generate zero-knowledge proofs from contributions.  
- Deploying a **zkApp verifier contract** on SL testnet to validate proofs.  
- Proofs can be used across multiple vApps in the SL ecosystem without exposing sensitive data.  

## Technical

### Architecture
1. User login (GitHub/Discord).  
2. Proof Generator creates zk-proof of contributions.  
3. Proof is sent to Soundness Layer → verified by zkApp contract.  
4. UI displays PoCC credential that can be used cross-platform.  

### Stack
- **Frontend**: Next.js + Wagmi + Tailwind  
- **Backend**: Node.js (API wrapper for GitHub/Discord)  
- **Blockchain**: Soundness Layer testnet  
- **Storage**: IPFS for credential metadata  

### Features
1. GitHub contribution proof (merged PRs ≥ threshold).  
2. Discord participation proof (event attendance, roles).  
3. Portable zk-credential for cross-platform reputation.  

## Timeline

### PoC (2-4 weeks)
- [ ] GitHub API integration + proof generation  
- [ ] Basic SL CLI integration  
- [ ] Simple credential UI  

### MVP (4-8 weeks)  
- [ ] Discord proof integration  
- [ ] zkApp verifier contract + frontend integration  
- [ ] User testing + documentation  

## Innovation
- **Unique**: Community contribution proof powered by zk, not just tokens/NFTs.  
- **Privacy-preserving**: No personal data exposure.  
- **Wide benefits**: Useful for DAOs, open-source projects, online communities, and education.  

## Contact
- Discord: ladsiggy#0000 (ID: 845394209346551843)  
- Email: ladsiggy@gmail.com  
- Updates: GitHub repository (to be shared once PoC is ready).  

---

**Checklist before submitting:**
- [x] All fields completed  
- [x] GitHub username matches PR author  
- [x] SL integration explained  
- [x] Timeline is realistic ✅  

