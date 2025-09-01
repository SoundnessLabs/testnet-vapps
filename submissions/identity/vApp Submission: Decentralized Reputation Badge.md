# vApp Submission: vApp Submission: Decentralized Reputation Badge

## Verification
```yaml
github_username: "tukangtambal1"
discord_id: "974910354121441330"
timestamp: "2025-09-02"
```

## Developer
- **Name**: bobox
- **GitHub**: @tukangtambal1
- **Discord**: anjingbae
- **Experience**: Full-stack Web3 developer with 3+ years of experience in building decentralized applications (dApps), smart contracts, and DAO tooling. Contributed to several open-source projects across Ethereum and Layer 2 ecosystems. Skilled in Solidity, EVM-compatible chains, React, Node.js, and various zk-based identity protocols. Familiar with GitHub OAuth, Discord bot development, and verifiable credential standards. Passionate about decentralized identity, contributor reputation, and improving transparency in online communities

## Project

### Name & Category
- **Project**: Decentralized Reputation Badge
- **Category**: identity

### Description
A verifiable on-chain badge system for rewarding and showcasing community reputation. Users earn badges for meaningful contributions (e.g. GitHub PRs, Discord moderation, content creation) which are minted as soulbound NFTs.
These badges can be used across ecosystems to prove identity, unlock roles, or build trust — acting as a decentralized reputation layer for Web3 communities and DAOs.

### SL Integration  
- Uses Soundness CLI to verify user identity and ensure badge claims are authentic.
- Integrates Soundness proof engine to validate cross-platform contribution (e.g. GitHub + Discord) before badge minting.
- All badges are minted only after successful SL verification to prevent abuse or spoofing.

## Technical

### Architecture
High-level system design and approach:
1. Activity Tracker Bot pulls data from GitHub and Discord.
2. Proof Engine (backed by Soundness) validates contribution data and links it to a verified identity.
3. Badge Contract mints soulbound NFTs on successful verification.
4. Frontend Dashboard lets users view badges and verify them publicly.

### Stack
- **Backend**: Node.js (Express)
- **Smart Contract**: Solidity (ERC-721 with soulbound logic)
- **Frontend**: React + Tailwind
- **SL Integration**: Soundness CLI + webhook validation

### Features
1. Verifiable badge minting (soulbound NFTs)
2. Cross-platform reputation tracking (GitHub + Discord)
3. Public badge viewer with on-chain proof

## Timeline

### PoC (2-4 weeks)
- [ ] Define badge types and criteria
- [ ] Implement Discord + GitHub data collector
- [ ] Integrate Soundness CLI for identity verification

### MVP (4-8 weeks)  
- [ ] Develop smart contract for badge minting
- [ ] Build frontend dashboard for users to view badges
- [ ] Deploy testnet version with working badge issuance

## Innovation
- Unlike centralized reputation systems, this project creates portable, on-chain proof of contribution. By integrating Soundness, it guarantees that only verified, unique individuals can claim badges, reducing spam and building trust across communities.

## Contact
- Discord: anjingbae
- GitHub: @tukangtambal1


**Checklist before submitting:**
- [X] All fields completed
- [X] GitHub username matches PR author  
- [X] SL integration explained
- [X] Timeline is realistic
