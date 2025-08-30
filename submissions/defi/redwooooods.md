# vApp Proposal: SquadVest

## 1. Project Overview

* **Project Title:** SquadVest
* **Category:** defi
* **GitHub Username:** [redwooooods]
* **Discord ID:** [1374782944677199924]

### Short Description
SquadVest is a DeFi platform that simplifies group investment for crypto assets. It enables small groups to easily create and manage shared, multi-signature wallets, allowing them to securely pool funds and collectively decide on investments through a transparent, on-chain proposal and voting system.

## 2. Problem & Solution

### Problem
Informal group investments among friends or small communities are common but fraught with issues. They typically rely on a single person holding the pooled funds, creating a single point of failure and requiring immense trust. Tracking contributions and making collective decisions is often done through messy spreadsheets and chat groups, which is inefficient, non-transparent, and prone to disputes.

### Solution
SquadVest provides a trustless and transparent solution by leveraging smart contracts. We offer a user-friendly dApp that allows any group to create their own "Squad Wallet"—a multi-signature smart contract that acts as a shared, decentralized bank account. Funds are held securely by the contract, and any action, such as buying an NFT or investing in a token, must be approved by a majority of the members through an on-chain vote. This democratizes group investment and replaces messy manual coordination with automated, secure on-chain governance.

## 3. Technical Architecture & SL Integration

Our architecture is built with security and user experience as top priorities.

* **Frontend:** A responsive and intuitive web application built with **React (Next.js)**. We will use libraries like **Wagmi** and **Viem/Ethers.js** to handle wallet connections and smart contract interactions seamlessly.
* **Backend (Optional/Light):** A lightweight **Node.js** server can be used for caching on-chain data (like proposal lists and transaction history) and sending notifications, but the core logic will remain entirely on-chain for the PoC.
* **Smart Contracts:** The core of the application will be written in Solidity and deployed on the SL blockchain.

### SL Integration
The entire functionality of SquadVest is powered by SL smart contracts, ensuring security and decentralization.
1.  **Factory Contract:** A single, central smart contract will be deployed to act as a "factory." Its sole purpose is to create and keep track of new, unique Squad Wallet contracts for each group, making the creation process efficient and standardized.
2.  **Multi-Signature Wallet Contract (The Squad Contract):** Each group's wallet is an independent smart contract instance deployed by the factory. This contract will hold all the pooled funds and contain the core logic, including:
    * A list of member addresses and their contribution amounts.
    * Functions like `deposit()`, `createProposal(target, value, data)`, `voteOnProposal(proposalId)`, and `executeProposal(proposalId)`.
    * Rules for governance, such as requiring a >50% majority vote to execute a proposal.
3.  **On-Chain Governance:** All critical actions—from adding funds to executing an investment—are governed by on-chain logic. Every proposal and vote is a transaction on the SL blockchain, creating an immutable and fully transparent history of the group's activities.

## 4. Development Timeline

We propose a realistic 8-week timeline to build a fully functional Proof-of-Concept (PoC).

* **Week 1-2: Smart Contract Development**
    * Design, write, and thoroughly test the Factory and Multi-Signature Wallet smart contracts using a development environment like Hardhat or Foundry.
    * Focus on security, gas optimization, and core functionality.

* **Week 3-4: Core Frontend & Wallet Integration**
    * Develop the user interface for wallet connection, squad creation, and inviting members.
    * Build the functionality for users to deposit funds into their newly created Squad Wallet.

* **Week 5-6: Proposal & Voting System**
    * Implement the most complex part of the UI: a dashboard to create, view, and vote on investment proposals.
    * Ensure the frontend correctly interacts with the smart contract to submit votes and display proposal statuses in real-time.

* **Week 7-8: Integration, Testing & Deployment**
    * Conduct rigorous end-to-end testing of the entire user flow on a public testnet.
    * Refine the UI/UX based on feedback.
    * Deploy the smart contracts to the mainnet/testnet and the frontend application to a hosting provider like Vercel.
