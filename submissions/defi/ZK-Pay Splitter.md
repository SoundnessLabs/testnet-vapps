# vApp Submission: 

## Verification
```yaml
github_username: "huanjinglu"
discord_id: "974910795450286100"
timestamp: "2025-08-31"
```

## Developer
- Name: huanjing
- GitHub: @huanjinglu
- Discord: totit7985
- Experience: I have experience in blockchain and decentralized application (dApp) development, particularly with Solidity smart contracts, zero-knowledge proof systems, and privacy-preserving protocols.
My past work includes building DeFi tools, DAO governance modules, and Web3 infrastructure integrations.
I am familiar with zk-SNARK/zk-STARK implementations, Hardhat testing, and frontend frameworks such as React and Next.js.
This background enables me to design privacy-first financial coordination tools with seamless integration into existing Web3 ecosystems.

## Project

### Name & Category
- Project: ZK-Pay Splitter
- Category: defi

### Description
- ZK-Pay Splitter is a decentralized payment coordination tool that allows multiple participants to contribute to a shared expense without revealing individual contributions.
It enables DAOs, event organizers, guilds, and communities to split payments (subscriptions, rent, group buys) while preserving financial privacy.

- Each participant contributes funds privately, and only a zk-proof of the total required payment is revealed. This ensures fairness and trust while preventing exposure of sensitive payment information.

### SL Integration
- Generate zero-knowledge proofs of individual contributions.
- Aggregate proofs to confirm the total required amount is reached.
- Verify proofs on-chain before executing the final payment.
This ensures the system remains private, verifiable, and tamper-proof.

## Technical

### Architecture
1. Smart Contract Deployment – Contract is created for a shared payment goal (e.g., 10 ETH).
2. Private Contribution – Each participant submits funds and SL generates zk-proof of contribution.
3. Proof Aggregation – SL aggregates proofs into a single verifiable statement of total contributions.
4. Payment Execution – Once verified, the smart contract automatically executes the payment to the recipient.

### Stack
- Solidity (smart contracts)
- Soundness Layer zk-proofs
- Hardhat (dev & testing framework)
- React / Next.js frontend
- Web3.js or Wagmi (wallet connection)

### Features
1. Anonymous contributions (hide who paid what).
2. Automatic execution of group payments when target is reached.
3. DAO/guild-friendly for shared expenses and treasury payments.
4. Public verifiability that payment is fulfilled without doxxing contributors.

## Timeline

### PoC (2-4 weeks)
- [ ] Write basic smart contract for payment pooling
- [ ] Integrate SL proof generation for single contributor
- [ ] Local demo of pooled contributions

### MVP (4-8 weeks)  
- [ ] Full zk-proof aggregation for multiple contributors
- [ ] On-chain verification of total contribution proofs
- [ ] Frontend UI for group creation & contribution tracking

## Innovation
- Unlike traditional split-payment tools, ZK-Pay Splitter introduces privacy-preserving group payments powered by zk-proofs.
It is the first approach where the total sum is verifiable but individual contributions remain hidden, enabling trustless coordination for DAOs, communities, and decentralized events.

## Contact
- Discord: totit7985
- GitHub: huanjinglu

---

Checklist before submitting:
- [x] All fields completed  
- [x] GitHub username matches PR author  
- [x] SL integration explained  
- [x] Timeline is realistic  

