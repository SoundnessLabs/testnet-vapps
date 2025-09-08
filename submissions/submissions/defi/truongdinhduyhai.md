Proposal: CrossChainLend

Category: defi

GitHub Username: truongdinhduyhai
Discord ID: ngochieu99

📌Project Overview

CrossChainLend is a decentralized lending protocol that enables users to deposit collateral on one chain (e.g., EVM) and borrow assets on another (e.g., Solana VM or Cosmos) through the Soundness Layer AVS.

It leverages zero-knowledge proofs (zkApps) for secure collateral verification and ensures consistent liquidity availability across supported chains.

🔧 Technical Architecture

Frontend: React + Next.js dApp interface

Smart Contracts: Solidity (EVM) + Move (Aptos / Sui) for chain-specific logic

Soundness Layer Integration:

- zk proof generation for cross-chain state verification

- AVS module for transaction soundness and settlement

Database: Off-chain indexing with The Graph / SubQuery

🎯 Features

Deposit collateral on EVM

Borrow stablecoins on Solana VM or Cosmos

zkProofs for secure cross-chain verification

Automated liquidation if collateral ratio < threshold

📆 Development Timeline

Week 1–2: Smart contract skeleton + integration with Soundness Layer SDK

Week 3–4: Frontend + wallet connection

Week 5: zkProof flow implementation

Week 6: Internal testing + bug fixes

Week 7: Testnet deployment
