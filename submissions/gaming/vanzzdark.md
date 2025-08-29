# Crypto Fortune Cookie (Random-Only)

**Category:** gaming  
**Author:** @vanzzdark
**Discord:** superma3688

## 1) Summary
A simple vApp: user connect wallet → click “Open Fortune Cookie” → contract returns a random crypto-themed fortune. Fun, viral, and perfect as testnet demo.

## 2) Problem & Motivation
Communities need tiny, delightful on-chain experiences to onboard users. This vApp provides instant gratification (one-click) dan cocok untuk aktivasi komunitas.

## 3) User Stories
- As a user, I connect wallet and click a button to receive a fortune.
- I can see my last fortune anytime.
- (Optional next) I can share a permalink to my last fortune.

## 4) Core Flow
1. Connect wallet
2. Call `openCookie()` → contract picks an index secara pseudo-random
3. Store & return the message; UI shows the fortune
4. `getLastFortune(address)` for fetching

## 5) Architecture
- **Smart Contract (Solidity):** stores predefined fortunes; generates pseudo-random index (time + sender + prevrandao).
- **Frontend:** minimal React (wagmi/ethers) or plain ethers.js; single button “Open Fortune”.
- **Storage:** on-chain array for messages (short strings).
- **No NFTs, no images** (MVP light and fasr).

## 6) Soundness Layer Integration Plan
- **MVP (this submission):** pure vApp on-chain demo to qualify; no ZK proof.
- **Phase-2 (upgrade):** replace pseudorandomness with a verifiable draw:
  - generate randomness off-chain → produce ZK proof (e.g., Ligetron/SP1),
  - store proof as Walrus blob,
  - verify on Soundness Layer and publish verified result back to vApp.
This keeps MVP simple today while aligning with Soundness Layer’s verification path.

## 7) Tech Stack
- Solidity ^0.8.20
- Frontend: React + ethers (or wagmi)
- Tooling: Hardhat/Foundry (any)
- Network: Soundness testnet (per team guidance)

## 8) Milestones
- **Day 1–2:** Contract + minimal UI, deploy to testnet
- **Day 3:** Polish copy + share demo
- **Phase-2:** ZK randomness integration via Soundness Layer

## 9) Risks & Mitigations
- Pseudorandomness not truly fair → addressed in Phase-2 ZK plan
- Spam clicks → simple cool-down per address (optional)

## 10) Team
Solo builder (@vanzzdark). Background: web3 tinkerer, quick prototyper.

