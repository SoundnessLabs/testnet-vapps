# vApp Proposal: ImpactQuest

## 1. Project Overview

* **Project Title:** ImpactQuest
* **Category:** social
* **GitHub Username:** [usmanlahab]
* **Discord ID:** [1383732583568900166]

### Short Description
ImpactQuest is a "Proof-of-Impact" platform that connects on-chain incentives with real-world positive actions. By allowing organizations to fund verifiable 'Quests'—like planting trees or cleaning beaches—and empowering a decentralized network of Verifiers to confirm their completion, we transform social and environmental good deeds into rewarding, collectible, and verifiable on-chain credentials.

## 2. Problem & Solution

### Problem
There is a fundamental disconnect between the desire to create positive change and the ability to verify and reward it. "Slacktivism" (low-effort online activism) is common, but it rarely translates to real-world impact. For charitable campaigns, there is often a lack of transparency and accountability—donors cannot be certain their funds were used as promised. How can we prove a tree was actually planted or a community workshop was held?

### Solution
ImpactQuest solves this by creating a trust layer for real-world impact, built on a "Perform, Prove, Prosper" model.
1.  **Perform:** Organizations (NGOs, DAOs, companies) create and fund "Impact Quests" with specific, measurable real-world goals.
2.  **Prove:** Participants complete the quest and submit cryptographic proof—typically a photo or video with timestamp and GPS metadata—to the platform.
3.  **Prosper:** This proof is then validated by a decentralized network of human "Verifiers" who are economically incentivized to be honest. Upon successful verification, the participant is automatically rewarded with cryptocurrency and a unique, non-transferable "Impact Badge" NFT, building an immutable on-chain portfolio of their real-world contributions.

## 3. Technical Architecture & SL Integration

Our architecture is designed to manage off-chain data (proofs) and coordinate a network of human verifiers with a secure on-chain settlement layer.

* **Frontend:** A web application built with **React (Next.js)** providing three key interfaces:
    1.  A **Quest Board** for participants to discover and join quests.
    2.  An **Organizer Dashboard** for sponsors to create quests and deposit reward funds.
    3.  A **Verifier Portal** for registered verifiers to review and vote on submissions.
* **Backend:** A **Node.js** service to handle user accounts, manage the submission queue (including uploads of large media files), process geolocation data, and orchestrate the verification process before settling on-chain.
* **Decentralized Storage (IPFS):** All submitted proof-of-impact evidence (images, videos) and the media for the Impact Badge NFTs will be stored on IPFS for permanent, auditable public access.

### SL Integration
The SL blockchain is the core settlement and trust layer that makes the ImpactQuest model possible.
1.  **Quest Escrow Contract:** When an organizer creates a quest, the reward funds are locked in a dedicated smart contract escrow on SL. This guarantees to participants that the rewards are real and available.
2.  **Verifier Staking & Curation Contract:** This contract governs the network of verifiers. To become a verifier, a user must `stake` the protocol's native token. The contract manages the registry of verifiers, assigns them tasks, and includes logic for rewarding honest verification and potentially slashing the stake of malicious verifiers.
3.  **Impact Badge SBT Contract:** A smart contract for minting non-transferable Soulbound Tokens (SBTs) that serve as the "Impact Badges." These are minted directly to a participant's wallet upon successful quest verification.
4.  **On-Chain Settlement:** Once a submission is approved by the required threshold of verifiers, a transaction is sent to the Quest Escrow Contract, which automatically releases the reward tokens to the participant and distributes small fees to the verifiers, ensuring immediate and trustless payment for all parties.

## 4. Development Timeline

Our 8-week Proof-of-Concept (PoC) will focus on demonstrating the core loop: Quest creation, proof submission, simplified verification, and reward payout.

* **Week 1-2: Core Smart Contracts**
    * Design and develop the Quest Escrow contract and the Impact Badge SBT contract.
    * Create a basic, on-chain registry for verifiers (staking/slashing can be simplified for the PoC).
    * Write thorough tests for the fund-locking and reward-release mechanisms.

* **Week 3-4: Backend & Quest Creation Flow**
    * Build the backend infrastructure for managing quest data and handling image/video uploads to IPFS.
    * Develop the frontend portal for organizers to create a quest, define its proof requirements, and fund the escrow contract.

* **Week 5-6: Participant & Verifier Portals**
    * Build the UI for participants to browse the Quest Board and submit their proof (including file upload and metadata).
    * Create the dashboard for registered Verifiers to view pending submissions and cast a simple "Approve" or "Reject" vote.

* **Week 7-8: Integration & Reward Settlement**
    * Integrate the full loop: A submission receives enough "Approve" votes, which triggers a backend call to the Quest Escrow contract.
    * The contract successfully pays out the rewards and mints the Impact Badge SBT to the participant's wallet.
    * Deploy and conduct end-to-end testing on a public testnet.
