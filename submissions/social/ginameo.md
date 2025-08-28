# vApp Submission: PrivChat – Private Messaging with zkIdentity

## Verification
```yaml
github_username: "ginameo"
discord_id: "962273830741639189"
timestamp: "2025-08-28"
```

## Developer
- **Name**: Gina Meo
- **GitHub**: @ginameo
- **Discord**: meongkuy#1879
- **Experience**: New to zk-tech and blockchain development, eager to learn and build privacy-focused apps.

## Project

### Name & Category
- **Project**: PrivChat
- **Category**: social

### Description
PrivChat is a private messaging app for Web3 communities. It uses **zkIdentity** from Soundness Layer so users can prove who they are (e.g., “I’m a member of Project X”) without exposing their real identity. Messages are end-to-end encrypted, and no metadata is stored or traceable.

### SL Integration  
* Use **zkApp** to verify user roles or membership (e.g., “Are you part of this DAO?”) without revealing addresses or personal info.
* Leverage **Soundness Layer** for secure, private authentication and access control.
* Integrate SL SDK for seamless login via zero-knowledge proofs instead of wallet signatures.

## Technical

### Architecture
Frontend (React) ↔ Backend (Node.js + WebSocket) ↔ Soundness Layer (zk-proof) ↔ Encrypted Message Relay

### Stack
- **Frontend**: React + Tailwind CSS
- **Backend**: Node.js + WebSocket
- **Blockchain**: Soundness Layer (zkApp)
- **Storage**: IPFS (for encrypted metadata)
- **Security**: End-to-end encryption (E2EE) + zk-proof-based login

### Features
1. Login using zk-proof (no personal data revealed)
2. Join private groups based on verified membership
3. Send encrypted messages that can’t be traced
4. “Burn after read” mode for sensitive messages
5. Privacy-preserving notifications (no metadata leaks)

## Timeline

### PoC (2-4 weeks)
- Set up zkIdentity verification on Soundness Layer
- Basic login with proof
- Simple chat UI
- Connect frontend to backend

### MVP (4-8 weeks)  
- Group creation and access control
- Full E2EE messaging
- IPFS integration for encrypted metadata
- Test with a small community or friends

## Innovation
PrivChat combines **true communication privacy** with **trustless identity verification**. It’s like Signal for Web3 — but instead of phone numbers, you prove your identity with zero-knowledge proofs. No doxxing, no tracking, no compromises.

## Contact
DM me on Discord: meongkuy#1879 
Or email: meongkuy9@gmail.com


