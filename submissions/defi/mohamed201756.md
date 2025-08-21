# vApp Submission: zk-Vest

## Verification
```yaml
github_username: "Mohamed201756"
discord_id: "898933354165043222"
timestamp: "2025-8-21"
```

## Developer
- **Name**: Mohamed
- **GitHub**: @Mohamed201756
- **Discord**: 0xMohamed
- **Experience**: 2 years of experience developing in the Web3 space with a focus on smart contracts and decentralized applications

## Project

### Name & Category
- **Project**: zk-Vest
- **Category**: defi

### Description
zk-Vest is a private payroll and token vesting platform for DAOs and crypto companies. It lets organizations manage token schedules for their teams without putting sensitive info on-chain. Employees can use ZK proofs to confirm they're owed a certain amount of tokens at a specific time, all without revealing their identity or entire compensation package

### SL Integration  
We'll use the Soundness Layer to create and verify the proofs for employment and vesting status. This makes sure that only the right person can claim their tokens and keeps all important payroll details completely private and secure

## Technical

### Architecture
A web-based dApp that interacts with smart contracts on the Soundness Layer. Employers will use a dashboard to create vesting schedules which are stored off-chain. Employees will connect their wallets to the dApp to generate a ZK proof of their eligibility which is then verified by our smart contracts to release the vested tokens.



### Stack
- **Frontend**: React & TypeScript
- **Backend**: Node.js for off-chain logic and data management  
- **Blockchain**: SL + Ethereum
- **Storage**: IPFS for storing encrypted vesting agreements + PostgreSQL for metadata

### Features
1. Private Vesting Schedules: Employers can create and manage confidential token vesting plans.
2. On-Chain Verification: Employees can anonymously verify and claim their vested tokens using ZK proofs.  
3. DAO Treasury Management: A simple interface for DAOs to manage their payroll and token distribution.

## Timeline

### PoC (2-4 weeks)
- [x] Smart contract for verifying a basic vesting proof on SL
- [x] Basic SL integration for proof generation
- [x] Simple UI for an employee to connect a wallet and claim tokens


### MVP (4-8 weeks)  
- [x] Full-featured dashboard for employers to create vesting schedules
- [x] Support for multiple tokens and complex vesting conditions
- [x] User testing with a few initial DAOs to get feedback

## Innovation
Payroll is a huge headache for DAOs and Web3 companies and current solutions often expose sensitive salary data. zk-Vest is the first platform to use ZK proofs for this process making it completely private and secure. Companies will use it to protect their team's privacy and employees will feel safer knowing their compensation isn't public.

## Contact
You can reach me on Discord (0xMohamed) or email me at Mohamedx2773@gmail.com


**Checklist before submitting:**
- [x] All fields completed
- [x] GitHub username matches PR author  
- [x] SL integration explained
- [x] Timeline is realistic
