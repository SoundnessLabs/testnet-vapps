# vApp Proposal: AudiMint

## 1. Project Overview

* **Project Title:** AudiMint
* **Category:** social
* **GitHub Username:** [potakika]
* **Discord ID:** [1382810894563807304]

### Short Description
AudiMint is a decentralized music platform that redefines the artist-fan relationship. By tokenizing music as limited-edition NFTs, artists gain greater control and revenue streams. Fans are transformed from passive listeners into active stakeholders, rewarded for their early support through a unique on-chain royalty sharing system. It's a new music economy that rewards discovery and direct patronage.

## 2. Problem & Solution

### Problem
The current music streaming model is broken for emerging artists, offering minuscule payouts that make a sustainable career nearly impossible. Simultaneously, fans who discover and champion artists early—acting as grassroots marketers—receive no financial benefit from the artist's eventual success. This creates a misalignment of incentives, where the most passionate supporters have no direct stake in the value they help create.

### Solution
AudiMint aligns the incentives of artists and fans. Artists can release their music as limited-edition NFTs, creating a new, direct-to-fan monetization channel that offers far greater revenue potential than traditional streams. For fans, AudiMint introduces a "Support-to-Earn" model. By purchasing a music NFT early, fans acquire a stake in that song's future success. A portion of revenue from all subsequent sales is automatically distributed to these early backers via a smart contract, rewarding them for their taste-making and early belief in an artist.

## 3. Technical Architecture & SL Integration

Our architecture focuses on creating a seamless experience for both creating and consuming music, with a robust on-chain foundation.

* **Frontend:** A modern, visually engaging web application built with **React (Next.js)**. It will feature artist profiles, a music discovery portal, a collection manager for users, and a token-gated audio player.
* **Backend:** A **Node.js** server will handle non-critical tasks such as managing user profiles, caching metadata, and processing audio file uploads before they are pinned to decentralized storage.
* **Decentralized Storage (IPFS/Arweave):** The core assets—audio files (.wav, .mp3) and album art—will be stored on a permanent, decentralized network like IPFS or Arweave to ensure longevity and censorship resistance.

### SL Integration
The SL blockchain is the engine that drives the entire AudiMint economy.
1.  **Music NFT Contract (ERC-1155 Standard):** We will use the ERC-1155 standard on SL, which is ideal for minting multiple editions of a single song (e.g., 100 "Gold" editions, 1000 "Silver" editions) within one efficient smart contract.
2.  **Royalty Distributor Contract:** This is the heart of our innovation. Each NFT collection will be linked to a unique Royalty Distributor smart contract. This contract collects a percentage from every primary and secondary sale and holds these funds. It contains the logic to allow early NFT holders (e.g., the first 10% of minters) to claim their proportional share of these royalties over time.
3.  **On-Chain Marketplace & Token Gating:** The platform will feature its own marketplace smart contract to facilitate the trading of music NFTs, ensuring that the royalty fee is always collected upon resale. The audio player on the frontend will be "token-gated," meaning it will verify with the blockchain that the user's connected wallet holds the specific music NFT before granting access to high-fidelity streaming.

## 4. Development Timeline

We have planned an 8-week timeline to develop a robust Proof-of-Concept (PoC).

* **Week 1-2: Smart Contract Development**
    * Design and write the ERC-1155 music NFT contract and the Royalty Distributor contract.
    * Develop extensive tests in a Hardhat/Foundry environment to ensure the royalty calculation and distribution logic is flawless and secure.

* **Week 3-4: Backend & Artist Onboarding Flow**
    * Build the backend API for handling artist profiles and audio uploads to IPFS/Arweave.
    * Create the frontend interface for artists to connect their wallet, set up their profile, and go through the process of minting a new track as an NFT.

* **Week 5-6: Music Discovery & Player**
    * Develop the core user experience for fans: browsing new releases, exploring artist pages, and purchasing primary sale NFTs.
    * Implement the token-gated audio player that checks for NFT ownership before enabling playback.

* **Week 7-8: Marketplace, Royalties & Deployment**
    * Build the UI for the secondary marketplace where users can list, buy, and sell music NFTs.
    * Create the "My Royalties" dashboard for eligible fans to view and claim their earnings from the Royalty Distributor contract.
    * Deploy all components to a public testnet and conduct a full end-to-end test of the entire lifecycle.
