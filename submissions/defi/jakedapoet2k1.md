MovePay – Cross-chain Payment Gateway

Category: DeFi

GitHub Username: jakedapoet2k1
Discord ID: haidinhduytruong

Overview

- MovePay is a decentralized cross-chain payment dApp built on Soundness Layer (Sui). It enables users to send and receive payments seamlessly across different blockchains (EVM ↔ Sui ↔ SolanaVM ↔ Cosmos). By leveraging zkProofs, MovePay ensures instant, low-cost, and secure payment verification.

Key Features

- Multi-chain Payments
  + Supports stablecoins (USDT/USDC) across multiple chains.
  + Users only need a single Soundness wallet for cross-chain transactions.
- zkProof Verification
  + Integrates zkProofs to guarantee transaction validity and prevent double-spending.
- Merchant API
  + Provides a simple API for merchants to accept crypto payments via Soundness.
- Low Transaction Fee
  +Uses Sui’s MoveVM efficiency to minimize transaction costs.

Technical Architecture

- Frontend: React + Sui Wallet Adapter (Soundness integration).
- Smart Contracts: Written in Move, deployed on Sui with Soundness Layer hooks.
- zkProof Module: Ensures secure and trustless verification of cross-chain payments.
- Backend (optional): Node.js service to provide merchant-facing API.

Roadmap & Milestones

- Month 1: Develop Move smart contracts and deploy on Sui devnet.
- Month 2: Integrate zkProof for secure cross-chain transactions.
- Month 3: Release merchant API beta.
- Month 4: Expand support to more chains (EVM, Aptos, Cosmos, Solana, Avalanche).
