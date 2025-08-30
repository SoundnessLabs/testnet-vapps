# vApp Proposal: CertiChain

## 1. Project Overview

* **Project Title:** CertiChain
* **Category:** identity
* **GitHub Username:** [kurniawatia]
* **Discord ID:** [1382807447961473106]

### Short Description
CertiChain is a decentralized education and credentialing platform that transforms learning achievements into verifiable on-chain assets. By issuing modular "Skill Badges" and composite "Certificates" as non-transferable Soulbound Tokens (SBTs), CertiChain creates a secure, transparent, and universally verifiable portfolio of a user's abilities, owned and controlled entirely by the user.

## 2. Problem & Solution

### Problem
The current digital credentialing system is broken. Certificates from online courses are often simple, easily forged PDFs siloed within proprietary platforms. They lack three key elements:
1.  **Verifiability:** There is no universal, trustless way to confirm the authenticity of a certificate.
2.  **Granularity:** A single certificate for a broad course fails to represent the specific, granular skills learned within it.
3.  **Portability:** Credentials are "stuck" on platforms like LinkedIn or Coursera; users cannot truly own or compose them across the digital landscape.

### Solution
CertiChain solves this by creating a new standard for on-chain educational credentials.
1.  **Verifiability:** All credentials are Soulbound Tokens (SBTs) minted on the SL blockchain, making them cryptographically secure and impossible to forge.
2.  **Granularity:** Instead of one certificate, users earn "Skill Badges" (SBTs) for each individual module or skill they master.
3.  **Portability & Composability:** Since these SBTs live in the user's personal wallet, they are owned by the user and can be used as verifiable proof of skill across any platform, dApp, or DAO in the Web3 ecosystem. Users can combine badges from different courses to build a unique and comprehensive skill profile.

## 3. Technical Architecture & SL Integration

Our architecture is designed to support a robust learning environment with a secure on-chain credentialing system.

* **Frontend:** A modern learning management system (LMS) interface built with **React (Next.js)**. It will provide dashboards for students (to track progress and view their credential portfolio) and for instructors (to create courses and set assessment criteria).
* **Backend:** A **Node.js** server will manage course content, user progression, and host the assessment engine. When a student passes an assessment, the backend will authorize the minting of a Skill Badge by calling the smart contract.
* **Assessment Engine:** For the PoC, this will be a robust quiz system. In the future, this can be expanded to include sandboxed environments for code testing or peer-review mechanisms.
* **Decentralized Storage (IPFS):** All course materials and the visual assets (images) for the SBTs will be stored on IPFS to ensure permanence.

### SL Integration
The SL blockchain is the core layer of trust for CertiChain's credentials.
1.  **Soulbound Token (SBT) Contract:** We will use a smart contract compliant with non-transferable token standards (like a simplified ERC-721 with transfer functions removed). This contract will handle the minting of all Skill Badges and Certificates. It will contain a function like `mintBadge(studentAddress, badgeId)`.
2.  **Registry & Logic Contract:** This is the "brains" of the platform. It will act as a registry for all official courses and their corresponding Skill Badges. It will contain the logic for "Learning Paths," including a function like `claimCertificate(pathId)`. This function checks if a student's wallet contains all the prerequisite Skill Badge SBTs and, if so, mints the final Certificate SBT.
3.  **Access Control:** The smart contracts will feature role-based access control. Only authorized wallets (representing instructors or the platform itself) will have the permission to call the `mintBadge` function, preventing unauthorized credential issuance.

## 4. Development Timeline

We propose an 8-week timeline to build a fully functional Proof-of-Concept (PoC).

* **Week 1-2: Smart Contract Foundation**
    * Design and develop the SBT contract for badges and certificates on SL.
    * Build the Registry & Logic contract, defining the data structures for courses, modules, and learning paths.
    * Write comprehensive tests, especially for the minting and certificate-claiming logic.

* **Week 3-4: Backend & Instructor Portal**
    * Develop the backend infrastructure for managing course content and user data.
    * Create the frontend portal for instructors to connect their wallet, create a course, define its modules, and link them to assessments.

* **Week 5-6: Student Portal & Learning Experience**
    * Build the student-facing dashboard to browse and enroll in courses, view learning materials, and take assessments.
    * Develop the "My Credentials" page, which reads the user's wallet to display the Skill Badges and Certificates they have earned.

* **Week 7-8: Integration & On-Chain Issuance**
    * Connect the assessment engine to the backend and the backend to the smart contracts, creating a full end-to-end flow.
    * When a student passes a quiz, the backend should successfully trigger the minting of the corresponding SBT to their wallet.
    * Deploy and conduct thorough testing on a public testnet.
