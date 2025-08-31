# vApp Submission: 

## Verification
```yaml
github_username: "budakbuset"
discord_id: "974912958717132821"
timestamp: "2025-08-31"
```

## Developer
- Name: budakbuset
- GitHub: @budakbuset
- Discord: beryadi88
- Experience: 
1. 4+ years in DeFi and smart contract development.
2. Built payroll automation systems on Ethereum & Polygon.
3. Strong background in Solidity, zk-SNARKs, and payroll integrations.
4 . Contributions to DeFi protocols (lending, payments, DAOs).

## Project

### Name & Category
- Project: zkPayroll Finance
- Category: defi

### Description
- A decentralized payroll system that enables organizations to pay employees in crypto, using zk-proofs to verify logged work or completed tasks.
Employers fund payroll contracts, and employees withdraw salaries only after submitting proof-of-work logs validated by Soundness Layer.
This creates a trustless, transparent, and fraud-resistant payroll mechanism.

### SL Integration
- Employees generate zk-proofs for hours worked or task completion.
- Employers verify zk-proofs without accessing raw data (privacy-preserving).
- Smart contracts release payments automatically once proof is validated via Soundness Layer.

## Technical

### Architecture
1. Employer deploys payroll contract with salary schedule.
2. Employee logs work data → zk-proof generated through SL.
3. Payroll contract verifies zk-proof validity.
4. Funds released to employee’s wallet automatically.

### Stack
- Solidity (smart contracts)
- Soundness Layer SDK (zk-proofs)
- Hardhat / Foundry (development & testing)
- Next.js + Tailwind (UI dashboard)
- IPFS/Arweave (optional logs storage)

### Features
1. Automated salary disbursement via crypto.
2. zk-proof validation of employee logs/tasks.
3. Employer dashboard to fund contracts.
4. Employee portal to claim payouts securely.

## Timeline

### PoC (2-4 weeks)
- [ ] Deploy basic payroll smart contract.
- [ ] Integrate SL for proof-of-work submission.
- [ ] Demo flow: employer funds contract → employee claims.

### MVP (4-8 weeks)  
- [ ] Multi-employee payroll contract support.
- [ ] Recurring payroll automation (weekly/monthly).
- [ ] Employer/Employee dashboards with UI.

## Innovation
 - zkPayroll Finance solves the trust problem in remote/crypto work by eliminating disputes. Employers only pay verified work, employees receive guaranteed payouts, all while maintaining privacy with zk-proofs.

## Contact
- Discord: beryadi88
- GitHub: budakbuset

---

Checklist before submitting:
- [x] All fields completed  
- [x] GitHub username matches PR author  
- [x] SL integration explained  
- [x] Timeline is realistic  

