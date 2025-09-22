# vApp Submission: [FlowLend]

## Verification
```yaml
github_username: "hosganini"
discord_id: "chenchen8289"
timestamp: "2025-08-29"
```

## Developer
- **Name**: hosganini
- **GitHub**: @hosganini
- **Discord**: chenchen8289
- **Experience**: Familiar with DeFi protocols and liquidity mechanics, with hands-on experience analyzing user flows and pain points in lending/borrowing platforms.

## Project

### Name & Category
- **Project**: FlowLend
- **Category**: defi

### Description
FlowLend is a micro-lending protocol that allows users to borrow small amounts of stablecoins instantly against their on-chain activity, without traditional over-collateralization.

It focuses on short-term liquidity needs (gas fees, bridging fees, small trades) that are underserved by current DeFi protocols. Instead of depositing large collateral, users get credit limits based on reputation and verifiable actions on Soundness Layer.

### SL Integration  
Use SL credentials to verify user history (e.g. completed trades, no defaults).
Generate a trust score that determines borrowing limits.
Secure settlement layer ensures micro-loans are enforced and reputation updates are tamper-proof.

## Technical

### Architecture
User requests a micro-loan.
FlowLend checks SL credentials → calculates trust score.
Loan issued directly from liquidity pool.
Repayment updates user’s reputation via SL.

### Stack
Frontend: React web app for loan dashboard
Backend: Rust microservice for scoring logic
Blockchain: SL for verification + EVM for settlement
Storage: WALRUS for loan history

### Features
Instant micro-loans for small amounts
Reputation-based credit scoring
Automated repayment + penalty enforcement

## Timeline

### PoC (2-4 weeks)
Single-pool micro-loan test
Basic trust score integration with SL
Simple UI for request/repayment

### MVP (4-8 weeks)  
Multi-pool lending support
Dynamic trust score updates
Community pilot + risk adjustment

## Innovation
Most DeFi lending today demands over-collateralization, which locks away liquidity and excludes small users. FlowLend flips this by using reputation as collateral. This opens DeFi to more users, supports micro-transactions, and builds a credit layer natively on-chain.

## Contact
Github:@hosganini

**Checklist before submitting:**
- [ ] All fields completed
- [ ] GitHub username matches PR author  
- [ ] SL integration explained
- [ ] Timeline is realistic
