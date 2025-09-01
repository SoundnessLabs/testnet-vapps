# vApp Submission: [Sybils Checker for Bot Users]

## Verification
```yaml
github_username: "dandypst"
discord_id: "dandyst"
timestamp: "2025-08-27"
```

## Developer
- **Name**: Richman77
- **GitHub**: @dandypst
- **Discord**: dandyst#8154
- **Experience**: Become a Node Validator and Bot Creator

## Project

### Name & Category
- **Project**: Sybils Checker for Bot Users 
- **Category**: identity

### Description
What problem does your vApp solve? What does it do?
The rapid growth of decentralized ecosystems is plagued by Sybil attacks, where a single entity creates a multitude of fake identities ("bot users") to unfairly influence governance voting, claim excessive airdrops, manipulate decentralized social graphs, and distort on-chain analytics. Manually identifying these coordinated bots is inefficient, non-scalable, and often impossible.

Sybils Checker is an analytic vApp that detects and scores the likelihood of an Ethereum Virtual Machine (EVM) address being part of a Sybil cluster. It does this by:

1.  On-Chain Behavioral Analysis: We analyze transaction history, including patterns of interaction (common funding sources, token swaps, NFT mints), gas usage, time-of-day activity, and connections to known exchange addresses and smart contracts.
2.  Graph Network Analysis: We map relationships between addresses. Sybil clusters often show dense, star-shaped transaction graphs funded from a common source, which is a key red flag.
3.  Scoring & Reporting: We assign a "Sybil Likelihood Score" to a queried address and provide a clear report highlighting the reasoning (e.g., "Funded by same source as 50+ other addresses," "Identical transaction pattern across multiple new wallets").
4.  API & Integration: Developers and DAOs can integrate our API to screen addresses automatically for their platforms (e.g., before a governance vote or a token claim).

In short, I turn messy on-chain data into actionable intelligence to help projects protect their communities and resources from automated abuse.

### SL Integration  
How will you use Soundness Layer? What specific SL features?
We intend to use the Soundness Layer as the **trustless and verifiable backbone for our entire data processing pipeline**. Specifically, we will leverage these features:

1. Trustless Off-Chain Computation (The Core Feature):
Our analysis algorithms are computationally intensive and cannot be run on-chain due to gas costs. We will use SL's zkWASM to run our proprietary detection algorithms off-chain and generate a zero-knowledge proof (ZKP) that the computation was executed correctly without revealing the algorithm's secret logic.

Benefit: Users and integrators don't have to trust us ("Sybils Checker"). They only need to trust the cryptographic truth of the Soundness Layer. They can verify that a score was generated correctly by our promised algorithm, even though the code itself remains our private intellectual property.

2. Verifiable Data Feeds (Price Oracles):
Our analysis incorporates token prices and values at specific times (e.g., to filter out negligible-value transactions used to create fake history). We will use SL's verifiable price oracles to ensure the financial data we use in our calculations is accurate and tamper-proof.

Benefit: It adds another layer of verifiable integrity to our input data, making our final output score even more reliable and resistant to manipulation.

3. The `isValid` Function for On-Chain Verification:
This is the crucial feature for integration. After we generate the ZK proof off-chain for a specific address analysis, we will:
    - Post the proof and the resulting **Sybil Score** to the blockchain.
    - A smart contract (our own or our client's) can then call SL's `isValid` function to instantly and inexpensively verify the proof.

Use Case Example: A DAO's voting contract can require users to interact with our verifier contract before casting a vote. If `isValid` returns `true` for a user's address and the associated score is below a certain threshold (e.g., "Low Sybil Risk"), the vote is counted. This creates a seamless, trustless, and automated gate against Sybil attacks directly in smart contracts.


## Technical

### Architecture
High-level system design and approach
1. Data Acquisition Layer
Components: Blockchain RPC nodes (e.g, QuickNode, Alchemy), The Graph Protocol subgraphs, and decentralized oracles (e.g., Soundness Layer's oracles for price data).
Function: This layer is responsible for ingesting raw, on-chain data. It fetches comprehensive transaction histories, token transfers, smart contract interactions, and current/historical price data for any given Ethereum Virtual Machine (EVM) address. The data is cleaned and formatted for processing.

2. Computation Layer (Core Analytics Engine)**
Components: Proprietary Analysis Algorithms, Graph Database (e.g., Neo4j), Soundness Layer zkWASM.
Function: This is where the core analysis happens.
Behavioral Module: Analyzes the formatted data for patterns indicative of bot activity (e.g., transaction timing, gas price preferences, common counterparties).
    *   Graph Module: Maps relationships between addresses to identify clusters funded by common sources, forming the basis of Sybil detection.
    *   Scoring Module: Synthesizes findings into a definitive "Sybil Likelihood Score."
    *   Soundness Layer Adapter: This critical component prepares the computation and passes the input data (the address's transaction history) to the zkWASM runtime. Our proprietary algorithm runs here, generating a zero-knowledge proof that the correct computation was performed without revealing its internal logic.

3. Verification & Application Layer (On-Chain Trust)
Components: Smart Contracts (Our Verifier Contract, SL's `isValid` function), REST API, Web Frontend.
Function: This layer handles user interaction and on-chain verification.
    *   API Gateway:** Receives requests from users or integrated dApps to check an address.
    *   Smart Contract Verifier:** The generated proof and resulting score are committed on-chain. Our smart contract interacts with the Soundness Layer's verifier contract to call the `isValid` function, providing immutable, public proof that the computation is correct.
    *   **Frontend Dashboard:** Displays the verified score and a detailed, human-readable report to the end-user.


### Stack
- **Frontend**: Vue
- **Backend**: Rust/Node.js/Python  
- **Blockchain**: SL
- **Storage**: WALRUS

### Features
1. On-Chain Behavioral Profiling
Our engine performs a deep analysis of wallet transaction history, examining patterns often invisible to the naked eye. This includes analyzing gas usage, time-between-transactions, interaction with known smart contracts, and sequences of actions to create a unique behavioral fingerprint for each address and identify automated bot-like activity.

2. Sybil Cluster Graph Analysis
We don't just look at addresses in isolation. By mapping transaction histories and funding sources, we identify clusters of addresses that are behaviorally linked and likely controlled by a single entity. This network-based approach is fundamental to exposing large-scale, coordinated Sybil attacks.

3. Verifiable Sybil Likelihood Score
Each address receives a clear, easy-to-interpret risk score (e.g., Low/Medium/High) powered by zero-knowledge proofs from the Soundness Layer. This allows users and smart contracts to trust that the score was computed correctly by our algorithm without having to reveal our proprietary IP, making the result both actionable and cryptographically verifiable.

4. Seamless API & Smart Contract Integration
We provide a simple API for developers to query scores and a verifier contract for on-chain use. This allows projects to integrate Sybil resistance directly into their dApps for use cases like weighted governance voting, fair airdrop distributions, and gated access, all in a automated and trustless manner.

## Timeline

### PoC (2-4 weeks)
- [ ] Basic functionality
- [ ] SL integration
- [ ] Simple UI

### MVP (4-8 weeks)  
- [ ] Full features
- [ ] Production ready
- [ ] User testing

## Innovation
What makes this unique? Why will people use it?
 **What makes this unique?**

Sybils Checker's uniqueness doesn't come from just doing Sybil detection, but from *how* we do it and the fundamental **shift from trust to verification** that we enable. Our key differentiators are:

1.  **Cryptographic Verifiability, Not Just Claims:** Every other Sybil detection tool provides a result you must take on faith. They are "black boxes." Our integration with the **Soundness Layer** allows us to generate a zero-knowledge proof for every analysis. This means anyone can cryptographically verify that our proprietary algorithm ran correctly on the given data, without us ever revealing the secret sauce. We replace "trust us" with "verify for yourself."

2.  **On-Chain Programmability for dApps:** This is the game-changer. We are not just a website with a dashboard; we are infrastructure. By providing a verifiable on-chain score, we allow smart contracts to **programmatically react to Sybil risk.** This makes Sybil resistance a new primitive that can be built directly into DeFi, governance, and social applications. No other solution offers this for real-time, on-chain use.

3.  **Privacy-Preserving Analysis:** Because the proof verifies the computation without revealing the inputs or logic, users and projects can check an address without exposing its entire transaction history to us or any other central party. It maintains a higher degree of privacy than solutions requiring full data export.

 **Why will people use it?**

**1. For Developers & DAOs (Our Primary Users):**
*   **To Automate Protection:** They will use our API and smart contract integration to automatically filter out Sybils from governance votes, token claims, and allowlists, saving immense time and resources on manual checks.
*   **To Ensure Fairness:** They need to protect their community and treasuries from being drained or manipulated by bots. Using our verifiable system adds a layer of legitimacy and fairness to their programs.
*   **To Build Better dApps:** They will use us as core infrastructure to create novel applications that require proof-of-personhood or Sybil resistance, giving them a competitive edge.

**2. For Investors & Analysts:**
*   **For Due Diligence:** They will use our tool to audit the health of a token's holder base or a DAO's governance structure before investing, checking if it's organic or dominated by a few entities using bots.

**3. For End-Users:**
*   **To Self-Verify:** A user can check their own wallet before participating in an airdrop or vote to ensure they won't be falsely flagged by the project's own system.
*   **To Gain Trust:** A user can generate a verifiable "Sybil-Free" proof to build reputation in a community or platform.


## Contact
Preferred contact method and where you'll share updates.
email: dandy.pujist@gmail.com

**Checklist before submitting:**
- [v] All fields completed
- [v] GitHub username matches PR author  
- [v] SL integration explained
- [v] Timeline is realistic
