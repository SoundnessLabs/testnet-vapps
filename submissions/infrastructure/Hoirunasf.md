# vApp Submission: [Your Project Name]

## Verification
```yaml
github_username: "Hoirunasf"
discord_id: "1090318024248152104"
timestamp: "2025-09-01"
```

## Developer
- **Name**: 0xboys
- **GitHub**: @Hoirunasf
- **Discord**: 0xboys#0
- **Experience**: Web3 researcher & developer with background in blockchain infrastructure, zk systems, and Ethereum tooling

## Project

### Name & Category
- **Project**: Styx Light Client
- **Category**: infrastructure

### Description
Current Soundness Layer proposals focus on zkApps and dApps, but there is no standardized light client implementation.
This project introduces a Styx-based light client to provide efficient verification of SL state without requiring a full node.
The client uses Merkle proofs and committee-based validation for block headers, enabling secure lightweight participation on mobile, browsers, or constrained environments.

### SL Integration  
Integrates directly with Soundness Layer consensus to verify finalized blocks.

Provides developers with a standard API to query verified chain data.

Enhances decentralization by reducing dependency on centralized RPC providers.

## Technical

### Architecture
Consensus header fetcher – retrieves block headers from SL peers.

Proof verifier – validates headers using Merkle/committee proofs.

State query module – lightweight interface for dApps.

Client SDK – exposes JS/Rust bindings for web/mobile.

### Stack
- **Frontend**: React (demo dashboard)
- **Backend**: Rust (light client core), Node.js (API service)  
- **Blockchain**: Soundness Layer
- **Storage**: In-memory DB + optional IPFS persistence

### Features
1. Core feature 1 Trustless block header verification via Styx rules
2. Core feature 2 Lightweight RPC replacement for dApps
3. Core feature 3 SDK for easy developer integration

## Timeline

### PoC (2-4 weeks)
- [x] Basic functionality
- [x] SL integration
- [x] Simple UI

### MVP (4-8 weeks)  
- [x] Full features
- [x] Production ready
- [x] User testing

## Innovation
First standardized light client for Soundness Layer.

Reduces reliance on centralized infra providers.

Enables mobile/web-native SL applications with trustless verification.

## Contact
Discord: 0xboys#0

GitHub: @Hoirunasf

Updates will be shared via GitHub repo + Discord.


**Checklist before submitting:**
- [x] All fields completed
- [x] GitHub username matches PR author  
- [x] SL integration explained
- [x] Timeline is realistic
