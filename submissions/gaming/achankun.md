
---

````markdown
# ProofPlay — Skill-Verified NFT Gaming Platform

## Verification
```yaml
github_username: "achankun"
discord_id: "837002312077017168"
timestamp: "2025-08-21"
````

## Developer

* **Name**: Achan
* **GitHub**: [@YOUR\_GITHUB\_USERNAME](https://github.com/achankun)
* **Discord**: YOUR\_DISCORD\_USERNAME
* **Experience**: 3+ years of experience in Web3, node operation, and building blockchain-based dApps/NFTs. Participated in ML, DeFi, and NFT recommendation competitions on platforms like Kaggle & CryptoPond.

## Project

### Name & Category

* **Project**: ProofPlay
* **Category**: gaming

### Description

ProofPlay is an NFT gaming platform that verifies player skills using **Soundness Layer** and zero-knowledge proofs (zkProofs). Players can prove achievements without revealing raw gameplay data, ensuring skill reputation and anti-cheat measures are secured on-chain.

### Soundness Layer Integration

* Use **SL zkApp** to generate proofs for scores & achievements.
* **Proof attestation** in Soundness Layer for each milestone.
* **SL Identity Binding** for cross-game skill reputation.
* On-chain proof verification integrated into the NFT reward smart contract.

## Technical

### Architecture

1. **Game Client** (Unity/WebGL) collects score & gameplay events.
2. **Proof Generator** creates zkProofs using Soundness Layer SDK.
3. **Smart Contract** verifies proofs and mints NFT rewards if valid.
4. **NFT Marketplace** enables trading rewards with on-chain skill reputation.

![ProofPlay Architecture](proofplay_architecture.png)

### Stack

* **Frontend**: Unity (WebGL) + React dashboard
* **Backend**: Node.js + Rust (proof generation)
* **Blockchain**: Soundness Layer + EVM-compatible chain
* **Storage**: IPFS (NFT assets), WALRUS (optional metadata)

### Features

1. zkProof-based skill verification to prevent cheating.
2. Unique NFT rewards for in-game achievements.
3. Cross-game skill reputation linked to the player's wallet.

## Timeline

### PoC (2-4 weeks)

* [ ] Basic SL SDK integration for proof generation
* [ ] Smart contract for proof verification
* [ ] Simple UI dashboard to submit proof & mint NFT

### MVP (4-8 weeks)

* [ ] Full gameplay implementation with proof-of-skill
* [ ] NFT marketplace integration
* [ ] User testing & optimization

## Innovation

* **Privacy & Anti-Cheat**: No raw gameplay data is revealed.
* **On-Chain Skill Reputation**: Usable across multiple games.
* **Truly Earned Rewards**: Only players with valid skill proofs can claim NFTs.

## Contact

* Telegram: [@YOUR\_HANDLE](https://t.me/cnidium)
* Twitter: [@YOUR\_HANDLE](https://twitter.com/achankun45)
* Email: Ichsanbit45@gmail.com

```

