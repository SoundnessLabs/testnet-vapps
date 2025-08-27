# vApp Submission: [Privacy Voting]

## Verification
```yaml
github_username: "Sunewbie"
discord_id: "820395564272713739"
timestamp: "2025-08-27"
```

## Developer
- **Name**: Sunewbie
- **GitHub**: @Sunewbie
- **Discord**: Sunewbie
- **Experience**: Newbie developer exploring zk and Move on SUI

## Project

### Name & Category
- **Project**: Privacy Voting
- **Category**: social/other

### Description
This proposal introduces a Privacy Voting vApp enabling secure, private, and verifiable voting mechanisms on Soundness, deployed over the SUI network.
Voters’ identities and selections remain confidential, while results are auditable via zero-knowledge proofs.
This is suitable for DAO governance, protocol upgrades, and confidential community polls.

### SL Integration  
Privacy Voting leverages Soundness Layer for:

zk-proof verification of voter eligibility and valid ballots.

Identity commitments & nullifiers to ensure one vote per identity.

Private state commitments so votes are hidden but verifiable.

zk-verified tallying, proving results are correct without revealing individual votes.

## Technical

### Architecture
The Privacy Voting vApp is designed around a commit–reveal scheme enhanced by zk-proofs and Soundness Layer integration.

Voter generates a vote commitment and zk-proof off-chain.

Frontend sends the commitment + proof to the contract.

Soundness Layer verifies proof validity and prevents double voting via nullifiers.

Tally Phase aggregates commitments, runs zk-verified tally, and publishes a verifiable proof of results.

This ensures anonymity, integrity, and auditability without revealing individual votes.

### Stack
- **Frontend**: React (with SUI wallet integration)
- **Backend**: Rust (for zk proof generation)
- **Blockchain**: Soundness Layer on SUI
- **Storage**: On-chain commitments + IPFS for tally reports

### Features
1. Private voting using zk-proofs (anonymity + integrity).
2. One vote per identity with commitments + nullifiers.
3. Verifiable tallying with zk-proofs for correct results.

## Timeline

### PoC (2-4 weeks)
- [ ] Basic zk circuit design
- [ ] SL integration for proof verification
- [ ] Simple UI for vote submission

### MVP (4-8 weeks)  
- [ ] Full voting workflow (commit, nullifier, tally)
- [ ] Production-ready Move contracts on SUI testnet
- [ ] User testing + result dashboard

## Innovation
Unlike standard on-chain voting, Privacy Voting ensures full voter anonymity while keeping results publicly verifiable.
It combines zk-proofs with Soundness Layer on SUI to deliver governance that is private, transparent, and tamper-proof.

## Contact
GitHub: SuNewbie
Discord: Sunewbie
Updates: GitHub repo + Soundness Discord


**Checklist before submitting:**
- [ ] All fields completed
- [ ] GitHub username matches PR author  
- [ ] SL integration explained
- [ ] Timeline is realistic
