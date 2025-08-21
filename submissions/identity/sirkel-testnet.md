# vApp Submission: [Your Project Name]

## Verification

```yaml
github_username: "sirkel-testnet"
discord_id: "585472166427099156"
timestamp: "2025-08-21"
```

## Developer

- **Name**: Ghalz
- **GitHub**: @sirkel-testnet
- **Discord**: its_ghalz
- **Experience**: Full-stack developer with 5+ years of experience specializing in Python and Node.js for building scalable backend systems and APIs. Eager to apply my backend expertise to web3 to create practical, privacy-preserving tools for online communities.

## Project

### Name & Category

- **Project**: Sound Proof
- **Category**: identity

### Description

Sound Proof is a verification tool that allows users to prove ownership of an asset (like an NFT or token) to get a special role on Discord, without having to publicly connect or reveal their wallet address. This solves the privacy problem where many community members are uncomfortable linking their social identity (Discord) with their financial activity (wallet).

### SL Integration

Integration with Soundness Layer is the core of this vApp.

1. Proof of Ownership: Users will generate a client-side (in-browser) ZK proof that attests their wallet contains a required asset (e.g., an NFT from a specific collection).
2. On-Chain Verification: This proof will be verified by a smart contract on the Soundness Layer. The contract will never know the user's wallet address; it only validates that the proof is correct.
3. Issuing Attestation: Upon successful verification, the contract will issue a verifiable signature or "credential" which the user can then use to claim their role via a Discord bot.

## Technical

### Architecture

The architecture is very straightforward: a frontend web app (React) where users generate proofs, a smart contract on Soundness Layer for verification, and a simple Discord bot connected to a Node.js backend to grant roles.

### Stack

- **Frontend**: React, ethers.js
- **Backend**: Node.js (for the Discord bot and a simple API), Python (for supporting scripts)
- **Blockchain**: Soundness Layer
- **Storage**: No complex data storage is needed.

### Features

1. Select Asset & Role: The user connects their wallet (locally) and chooses the Discord role they want to claim (e.g., "NFT Holder").
2. Generate Proof: With a single click, the vApp will generate the ZK proof of asset ownership.
3. Claim Role: The user copies the generated credential and sends it to a Discord bot to automatically receive their role.

## Timeline

### PoC (1-2 weeks)

- [ ] Basic functionality to prove ownership of a single, predetermined NFT type.
- [ ] SL integration for proof verification.
- [ ] A very simple UI.

### MVP (3-4 weeks)

- [ ] A system where community admins can register their own NFT collections or tokens.
- [ ] A fully functional Discord bot.
- [ ] Clear user guides.

## Innovation

Its innovation lies in its simplicity and focus on privacy for a very common use case. Instead of building a complex identity system, Sound Proof provides a "one-click" solution to a real problem faced by nearly every web3 community on Discord. It's a practical tool that people will use immediately.

## Contact

discord and my telegram community or x

**Checklist before submitting:**

- [ ] All fields completed
- [ ] GitHub username matches PR author
- [ ] SL integration explained
- [ ] Timeline is realistic
