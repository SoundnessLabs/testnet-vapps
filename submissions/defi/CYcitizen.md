# vApp Submission: PredictX

## Verification
```yaml
github_username: "CYcitizen"
discord_id: "pinktu"
timestamp: "2025-08-23"
```

## Developer
- **Name**: CYcitizen
- **GitHub**: @CYcitizen
- **Discord**: pinktu
- **Experience**: Active Web3 community contributor, with background in DeFi products and user research. Experienced in product design and ecosystem growth.

## Project

### Name & Category
- **Project**: PredictX (Decentralized Prediction & Reputation Market)
- **Category**: defi

### Description
PredictX is a decentralized prediction platform where users can forecast project outcomes (e.g., token launch valuation, roadmap delivery, adoption metrics) and earn reputation based on accuracy. It solves the problem of trust in crypto discussions by attaching measurable prediction performance to each user’s profile, creating a transparent reputation layer.

### SL Integration  
Use Soundness Layer (SL) for verifiable credentials and reputation scores.
Each prediction outcome is cryptographically signed via SL.
Reputation points are stored and validated on SL, making them portable across other dApps.

## Technical

### Architecture
Users make predictions → stored on-chain with SL integration.
SL validates predictions after official outcomes are published.
Reputation score updated → displayed in user profile and can be queried by other apps.

### Stack
- **Frontend**: React + Tailwind
- **Backend**: Node.js (API + event tracking)  
- **Blockchain**: SL + Ethereum (settlement layer)
- **Storage**: IPFS (for predictions & outcomes), SL for credentials

### Features
1 On-chain prediction markets with low fees
2 Verifiable reputation linked to user accuracy
3 Public profile dashboard to showcase past performance

## Timeline

### PoC (2-4 weeks)
- [ ] Basic prediction input & storage
- [ ] SL credential issuance for users
- [ ] Simple web UI for submitting forecasts

### MVP (4-8 weeks)  
- [ ] Complete prediction & resolution system
- [ ] Reputation score with SL validation
- [ ] User testing & onboarding flow

## Innovation
Unlike typical prediction markets, PredictX builds a long-term reputation system. Instead of only betting for profit, users earn credibility, which can be reused in DAOs, DeFi lending, or token-gated communities. This transforms crypto speculation into measurable social capital.

## Contact
Updates will be shared in GitHub repo


**Checklist before submitting:**
- [ ] All fields completed
- [ ] GitHub username matches PR author  
- [ ] SL integration explained
- [ ] Timeline is realistic
