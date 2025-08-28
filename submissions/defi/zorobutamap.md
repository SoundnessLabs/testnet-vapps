# vApp Submission: SoundSaver

## Verification
```yaml
github_username: "zorobutamap"
discord_id: "962275480709517372"
timestamp: "2025-08-28"
```

## Developer
- **Name**: Zoro Juro
- **GitHub**: @zorobutamap
- **Discord**: namibeban#8171
- **Experience**: New to zkApps, but experienced in React and Node.js

## Project

### Name & Category
- **Project**: SoundSaver
- **Category**: defi

### Description
A micro-savings app where users can save small amounts daily (e.g., $1–$5) without revealing their balance or address. Users can prove they’ve saved a certain amount without exposing their identity. Perfect for DAOs or privacy-focused communities.

### SL Integration  
* Use **zkApp** to verify savings without revealing the amount or user address.
* Store encrypted metadata on IPFS for auditability.
* Use SL SDK for login via zero-knowledge proofs (no wallet needed).

## Technical

### Architecture
Frontend (React) ↔ Backend (Node.js) ↔ Soundness Layer (zkApp) ↔ IPFS

### Stack
- **Frontend**: React + Tailwind CSS
- **Blockchain**: Soundness Layer (zkApp)
- **Storage**: IPFS
- **Security**: Encrypted metadata + zk-proof-based login

### Features
1. Daily micro-savings with privacy
2. Prove savings amount without revealing balance
3. DAO-based access control (e.g., only members can join)
4. "Goal Tracker" with encrypted progress

## Timeline

### PoC (2-4 weeks)
- Setup zkApp for savings verification
- Basic UI for deposit and tracking
- Connect to IPFS for encrypted metadata

### MVP (4-8 weeks)  
- Full E2EE savings tracking
- DAO-based access control
- Test with 50 users in a Discord community

## Innovation
SoundSaver is the first micro-savings app that uses **zero-knowledge proofs** to verify savings without exposing user identity or balance. It’s perfect for communities that want to encourage saving without compromising privacy.

## Contact
DM me on Discord : namibeban#8171/ .zorobutamap

