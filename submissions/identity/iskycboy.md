# vApp Submission: CrossChain ID Verifier

## Verification

```yaml
github_username: "iskycboy"
discord_id: "723159078473564171"
timestamp: "2025-08-26"
```

## Developer

- **Name**: iskycboy
- **GitHub**: @iskycboy
- **Discord**: "723159078473564171"
- **Experience**: Web3, Laravel (PHP), Python, Android development, community tools, privacy-preserving apps

## Project

### Name & Category

- **Project**: Sound Proof of Humanity Lite
- **Category**: identity

### Description

Sound Proof of Humanity Lite solves the Sybil attack problem by ensuring “one person = one account” without requiring KYC or exposing wallet addresses.
Users generate a unique zero-knowledge proof (e.g., hashed Discord ID + secret) and submit it to the backend for verification. Each proof can only be used once, preventing spam and multiple fake accounts.

### SL Integration

1. Uses Soundness Layer to generate and verify zero-knowledge proofs of uniqueness.
2. Proofs validate identity without revealing Discord IDs or wallet addresses.
3. Provides a privacy-preserving coordination layer for communities and DAOs.

## Technical

### Architecture

1. User enters Discord ID + secret → hashed → proof generated via Soundness CLI or mobile SDK.
2. Proof submitted to backend (Laravel or Python API).
3. Backend verifies proof with Soundness Layer.
4. Valid proofs stored in DB; reused proofs rejected.
5. Website + Android app display verified user list.

### Stack

- **Frontend**: React (website for registration + verified list)
- **Backend**: Laravel (PHP) / Python (Flask or FastAPI)
- **Blockchain**: Soundness Layer (proof verification)
- **Storage**: SQLite / MySQL
- **Mobile**: Android app (Kotlin/React Native)

### Features

1. One person = one account (Sybil resistance)
2. Privacy-preserving proof of uniqueness
3. No KYC required
4. Website demo for easy access
5. Android app for Discord-heavy communities

## Timeline

### PoC (3 weeks)

- [x] Proof generation with Soundness CLI
- [x] Backend API (Laravel/Python) for proof verification
- [x] Simple React website for registration + verified users list
- [x] Basic Android app: generate + submit proof

### MVP (6 weeks)

- [ ] Discord bot integration for role assignment
- [ ] Admin dashboard (web) to manage verified users
- [ ] Production-ready website & Android app
- [ ] Push notification on Android app when verified

## Innovation

Unlike traditional KYC or wallet-based verification, Sound Proof of Humanity Lite is:

1. Lightweight → simple PoC, fast to build
2. Privacy-preserving → no wallet or identity exposure
3. Multi-platform → works on both website and Android app
4. Directly solves Sybil resistance for DAOs and Web3 communities

## Contact

Preferred contact: Discord (723159078473564171)
Updates will be shared regularly in the Soundness community channels.
