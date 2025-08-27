# vApp Submission: [AquaSwap]

## Verification
```yaml
github_username: "@FanSems1"
discord_id: "840200815142633483"
timestamp: "2025-08-27"
```

## Developer
- **Name**: Samuel Sukarno
- **GitHub**: @FanSems1
- **Discord**: fann#3464
- **Experience**: Fullstack & Web3 developer with strong background in DeFi protocols, zkApps, and decentralized infrastructure. Focused on scaling solutions and integrating zero-knowledge proofs for real-world use cases.
## Project

### Name & Category
- **Project**: AquaSwap
- **Category**: defi

### Description
AquaSwap is a lightweight decentralized exchange (DEX) built on Soundness Layer, designed for micro-trading with ultra-low fees.
Unlike traditional DEXs, AquaSwap leverages zkRollups combined with Soundness blob storage, enabling high throughput while preserving decentralization and security.

### SL Integration  
Proof Generation: All swap transactions will generate zk-proofs through Soundness CLI.

Blob Storage: Settlement batches are compressed and stored on Soundness blobs, reducing costs and ensuring scalability.

Verification: On-chain verifier contracts will validate proofs before finalizing swaps.

Developer Workflow: Local testing with Soundness CLI → Submit proofs → Validate on testnet → Push to mainnet Soundness Layer.

## Technical
Frontend: React + Wagmi + Soundness CLI SDK

Backend: Node.js microservices with PostgreSQL for user state caching

Integration:

Proof generation using Soundness Layer

Blob storage for batch settlement

On-chain verifier for swap pairs

Flow:

User connects wallet → submits order

Orders are bundled & verified with zk-proofs

Batch settlement committed to Soundness Layer

### Architecture
High-level system design and approach

### Stack
Frontend: React + Wagmi + Tailwind

Backend: Node.js microservices with PostgreSQL

Blockchain: Soundness Layer + Ethereum (Sepolia for testing)

Storage: Soundness blob storage + IPFS for metadata

### Features
Connect wallet & swap tokens with near-zero fees

zk-Proof validation for all trades (Soundness CLI integration)

Batch settlement using Soundness blob storage

DAO-based fee governance module (future milestone)

## Timeline

Week 1–2: Proof-of-Concept smart contracts + basic frontend

Week 3–4: Integrate Soundness CLI & proof generation

Week 5–6: UI/UX improvements + internal testnet launch

Week 7: Public testnet deployment + community feedback

## Innovation
Micro-Trading First: Focused on small-scale, high-frequency swaps ignored by larger DEXs.

zk-Proof Powered: Ensures trades are validated securely without leaking sensitive user data.

Low-Cost Architecture: Uses Soundness Layer blobs to reduce storage costs.

Community Driven: Lightweight design suitable for DAOs, creators, and niche communities.

## Contact
X (Twitter): @lingdonjo

Instagram: @semskrn

LinkedIn: Samuel Sukarno

Telegram: @xncekz


## Checklist before submitting:
- [x] All fields completed
- [x] GitHub username matches PR author
- [x] SL integration explained
- [x] Timeline is realistic

