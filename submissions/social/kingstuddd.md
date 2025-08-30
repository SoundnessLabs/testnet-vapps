# vApp Proposal: ProofTix

## 1. Project Overview

* **Project Title:** ProofTix
* **Category:** social
* **GitHub Username:** [kingstuddd]
* **Discord ID:** [1374779756725272704]

### Short Description
ProofTix is a decentralized event ticketing platform that uses NFTs to combat scalping and create lasting value for attendees. By issuing tickets as smart NFTs, organizers can control secondary market prices through on-chain rules. After an event, scanned tickets are automatically converted into a permanent, collectible Proof of Attendance (POAP), transforming a disposable ticket into a valuable digital asset.

## 2. Problem & Solution

### Problem
The modern event ticketing industry is fundamentally broken. Firstly, it is dominated by scalpers who use bots to hoard tickets and resell them at exorbitant prices, creating an unfair and frustrating experience for genuine fans. Secondly, traditional tickets—whether physical or digital—become worthless the moment an event ends. They offer no lasting value, community connection, or proof of attendance beyond a simple memory.

### Solution
ProofTix solves these two problems simultaneously using NFT technology.
1.  **Anti-Scalping:** We empower event organizers to issue tickets as NFTs with built-in rules for resale. They can either make tickets non-transferable or allow them to be resold only on our secure, integrated marketplace with a programmable price cap (e.g., not exceeding 120% of the original price).
2.  **Lasting Value:** Upon entry verification via a QR scan, the ticket NFT is marked as "attended." After the event, this NFT's metadata is updated, transforming it into a unique Proof of Attendance (POAP). This POAP serves as a permanent digital collectible and a verifiable credential that can grant holders access to exclusive communities, future airdrops, or special merchandise.

## 3. Technical Architecture & SL Integration

Our architecture is designed for scalability, security, and a seamless user experience for both organizers and attendees.

* **Frontend:** A responsive web application built with **React (Next.js)**, providing distinct interfaces for event organizers (to create events and mint tickets) and attendees (to purchase, manage, and view their ticket/POAP collection).
* **Backend:** A **Node.js (Express.js)** service will handle off-chain logic, such as managing event details, generating QR codes for tickets, and providing a secure endpoint for scanners at the event venue to verify ticket ownership.
* **Decentralized Storage (IPFS):** All ticket and POAP images and metadata will be stored on IPFS to ensure their permanence and decentralization, preventing broken links or lost media.

### SL Integration
The SL blockchain is the backbone of ProofTix, providing the trust and programmability needed for our core features.
1.  **NFT Ticket Contract (ERC-1155 Standard):** We will use the ERC-1155 standard as it is highly efficient for creating multiple types of tickets (e.g., 1000 General Admission, 100 VIP) under a single event contract.
2.  **Marketplace Smart Contract:** A dedicated smart contract will govern the secondary market. This contract will contain the logic to enforce the price caps set by the event organizer. Any attempt to sell a ticket through this contract above the set price will be automatically rejected.
3.  **On-Chain Verification & POAP Transformation:**
    * When an attendee's QR code is scanned at the venue, a verifier (authorized by the organizer) signs a message.
    * This signature is used to call a `checkIn` function on the NFT contract, marking that specific ticket token as "used."
    * After the event concludes, the organizer calls a final function on the contract, which updates the `tokenURI` for all "used" tickets, pointing them to the new POAP metadata on IPFS. This effectively transforms the tickets into collectibles.

## 4. Development Timeline

We propose a realistic 8-week timeline to build a fully functional Proof-of-Concept (PoC).

* **Week 1-2: Smart Contract Foundation**
    * Design and develop the ERC-1155 ticket contract and the price-capped marketplace contract.
    * Write comprehensive tests to ensure security and functionality, especially for the resale logic.

* **Week 3-4: Backend & Organizer Portal**
    * Build the backend services for event management and QR code generation.
    * Develop the frontend portal for organizers to create events, define ticket tiers (price, quantity, resale cap), and mint the initial NFT tickets.

* **Week 5-6: Attendee Portal & Marketplace**
    * Build the public-facing interface for users to browse events, purchase primary-sale tickets, and view their collection.
    * Implement the secondary marketplace UI, allowing users to list and buy tickets according to the on-chain rules.

* **Week 7-8: QR Verification, POAP Logic & Deployment**
    * Develop the scanner application/interface for event staff.
    * Implement the final `checkIn` and POAP transformation logic, integrating the full lifecycle of a ticket.
    * Conduct end-to-end testing on a public testnet and deploy the PoC for demonstration.
