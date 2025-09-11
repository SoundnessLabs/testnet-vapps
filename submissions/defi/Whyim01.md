vApp Proposal

Applicant Info
- GitHub Username: Whyim01
- Discord ID: 916784041628434482
- Category: DeFi

vApp Name
SoundSwap – Confidential DEX

Short Description
SoundSwap is a decentralized exchange (DEX) leveraging the Soundness Layer to keep order books and swap intents confidential.  
The goal is to prevent front-running, sandwich attacks, and trading data leaks.

Problem Statement
- Traditional blockchains expose all transactions publicly → vulnerable to MEV and manipulation.  
- Traders require privacy when executing swaps or providing liquidity.  

Proposed Solution
- Use Soundness Proofs to validate swaps without revealing token amounts until execution is finalized.  
- Liquidity remains transparent, but transaction details (amount and user address) stay encrypted.  
- Support for multiple asset types (ERC20 and stablecoins).  

Technical Architecture
1. Frontend: React + wagmi, wallet integration.  
2. Backend: Node.js API → relays transactions to Soundness CLI.  
3. Smart Contracts: Solidity + Soundness Layer SDK.  
4. Proof Generation: All swap intents are processed via Soundness CLI → proof → verified on-chain by smart contract.  

Development Timeline
- Week 1–2: Repo setup, wallet integration, base swap contract.  
- Week 3–4: Integrate Soundness CLI for proof handling.  
- Week 5–6: Testnet deployment and internal audits.  
- Week 7: PoC deployment and public documentation.  

Expected Outcomes
- Live PoC on testnet with at least 2 token pairs.  
- Working demo DEX with confidential swap transactions.  
- Developer documentation and onboarding guide.  
