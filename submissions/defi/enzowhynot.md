# GhostPay

## Summary
GhostPay is a private, peer-to-peer payment app built on the Soundness Layer. It enables users to send and receive crypto without exposing balances or transaction histories.

## Category
defi

## Motivation
While many DeFi apps offer financial services, few prioritize **privacy**. GhostPay gives users full control over their funds while leveraging zero-knowledge proofs to ensure transaction confidentiality and unlinkability.

## Architecture
- **Frontend**: Built with React + TailwindCSS
- **Backend**: Minimal backend for session auth (Node.js)
- **Soundness Integration**: 
  - zkProofs to hide transaction details  
  - User wallet reputation through Soundness credentials
  - Direct contract interaction via Soundness SDK

## Timeline
- Week 1: UI Design & Wallet Integration  
- Week 2: zk transaction logic implementation  
- Week 3: Connect to Soundness Layer testnet  
- Week 4: Deploy, test, and document PoC

## GitHub
https://github.com/enzowhynot/testnet-vapps

## Discord ID  
1316787669694545945
