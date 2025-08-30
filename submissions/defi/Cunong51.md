# vApp Submission: [defi]

## Verification
```yaml
github_username: "@Cunong51"
discord_id: "650569999685255168"
timestamp: "2025-08-29"
```

## Developer
- **Name**: cunong
- **GitHub**: @Cunong51
- **Discord**: kvcbas
- **Experience**: Brief background

## Project

### Name & Category
- **Project**: DeFi Auto-Stake & Claim Router
- **Category**: defi

### Description
The DeFi Auto-Stake & Claim Router is a vApp that makes it easy for users to:
- Claim staking rewards automatically
- Directly transfer claimed proceeds to a secure wallet (e.g., a cold wallet or a new smart wallet)
- Reduce the risk of wallet drain by separating the execution address and the receiving address

### SL Integration  

## Motivation

Many DeFi users have lost assets due to private key compromises or wallet drains.
This solution helps with:
- Auto-claim staking/airdrops without exposing the primary wallet
- Direct transfers to different recipient addresses
- ZK-proof support for secure claim verification without the need to disclose sensitive data

## Technical Approach

Smart conract `AutoClaimRouter` smart contract that:
1. Calls claim functions (`claim`, `claimAndDelegateMultiple`, etc.)
2. Routes claim results directly to the user-specified recipient wallet
- Integration with the Soundness Layer for verification:
- zkApp proof of ownership (from the old wallet)
- Transaction delegation to the new wallet for gas payments
- The vApp will be built with the following stack:
- Solidity/Vyper for smart contracts
- Hardhat for deployment & testing
- Simple frontend (Next.js + wagmi) for the user interface

## Potential Impact

- Reduces security risks for DeFi users
- Becomes the standard middleware for staking and airdrop claims
- Can be adopted by other protocols for cross-chain auto-claims
- Supports CeFi-2-DeFi scenarios (protected reward claims)


## Technical

### Architecture
High-level system design and approach

### Stack
- **Frontend**: React/Vue/etc
- **Backend**: Rust/Node.js/Python/etc  
- **Blockchain**: SL + others
- **Storage**: Database/WALRUS/IPFS/etc

### Features
1. Auto-Claim Rewards → automatic staking/airdrop claims without manual transactions.
2. Secure Routing → claim results are immediately transferred to a secure wallet (cold wallet / new smart wallet).
3. zk-Proof Verification → claims are verified with zero-knowledge proof without revealing sensitive data.
4. DeFi Middleware → can be integrated into various staking or airdrop protocols on the Soundness Layer.

## Timeline

### PoC (2-4 weeks)
- [ ] Basic functionality
- [ ] SL integration
- [ ] Simple UI

### MVP (4-8 weeks)  
- [ ] Full features
- [ ] Production ready
- [ ] User testing

## Innovation
Gas sponsorship → allows third parties or new wallets to cover gas costs instead of legacy wallets.

Composable middleware → can be used across staking/airdrop protocols on the Soundness Layer.

As a result, users gain an extra layer of security, a more streamlined experience, and a solution that could become the new standard in DeFi reward claims.

## Contact
 X : https://x.com/karteldnx


**Checklist before submitting:**
- [ ] All fields completed
- [ ] GitHub username matches PR author  
- [ ] SL integration explained
- [ ] Timeline is realisticThe main innovation of the DeFi Auto-Stake & Claim Router is the separation of claim rights from reward receipt.
With this concept, users can still use legacy wallets (e.g., those that have been compromised or are vulnerable) to prove claim ownership, while rewards are sent directly to secure wallets without having to transit through risky wallets.

Additionally, it includes:

zk-Proof → verifying claim ownership without exposing private data.