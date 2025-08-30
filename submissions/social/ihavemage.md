# vApp Submission: [AnonDAO Chat – Privacy-Preserving Voting Messenger]

## Verification
```yaml
github_username: "ihavemage"
discord_id: "373123588376756225"
timestamp: "2025-08-30"
```

## Developer
- **Name**: arie
- **GitHub**: @ihavemage
- **Discord**: ihavemage
- **Experience**: in the crypto industry since 2017 as an investor or having worked on early projects

## Project

### Name & Category
- **Project**: AnonDAO Chat – Privacy-Preserving Voting Messenger
- **Category**: social

### Description
**Problem**

  Current DAO and on-chain communities lack a unified platform that combines **real-time discussion** and **voting**.
Members usually chat on Discord/Telegram and vote separately on Snapshot or other tools, which creates fragmentation, inefficiency, and opens the door for **bot attacks or duplicate accounts**

**What it does**

  AnonDAO Chat provides a **messaging platform integrated with anonymous voting**.

  Members can join group chats, verified by Soundness Layer, without exposing their real identities.
They can participate in polls and DAO voting directly inside the chat.
zkProof ensures each member is legitimate, while votes remain private and secure.

### SL Integration  
  AnonDAO Chat will directly integrate with **Soundness Layer** to handle identity verification, proof generation, and governance logic:
  **Wallet Authentication with SL SDK**
Users log in using their wallet through the Soundness Layer SDK. This ensures only verified members can access the chat.

  **zkProof Verification**
AnonDAO Chat leverages SL’s **zero-knowledge proof capabilities** to validate membership. A user can prove they belong to a DAO or community **without revealing their wallet address or sensitive information**.

  **On-chain Reputation / Membership State**
SL is used to maintain and query on-chain data such as membership credentials, badges, or reputation scores. This prevents bots and duplicate accounts.

  **Governance Smart Contracts**
Polls and votes inside the chat are executed as lightweight contracts on Soundness Layer. SL ensures that results are **transparent, immutable, and tamper-proof**.

  **In short**: Soundness Layer provides the **identity, zkProof privacy, and governance infrastructure** that powers AnonDAO Chat.

## Technical

### Architecture
High-level system design and approach

**Trust & Identity**: All membership/auth is handled by **Soundness Layer**. Users prove DAO membership with **zkProofs** so the app never sees raw wallet identities.

**Messaging Path**: Messages are **end-to-end encrypted** on the client. A lightweight relay only routes encrypted blobs (no plaintext stored). Optional WALRUS/IPFS pinning for ephemeral history.

**Governance Path**: Poll creation, vote casting, and tallying flow through **SL governance contracts**. Votes are anonymous but **uniqueness** is enforced by SL’s proof system (1 member = 1 vote or reputation-weighted).

**State Split**:
   **On-chain (SL)**: membership credentials, poll/vote state, reputation weights.
   **Off-chain**: encrypted message blobs, channel metadata (non-sensitive), presence.

**Privacy by Default**: Wallets stay private; moderators act on proof-backed roles instead of identities.

**Scalability**: Chat scales horizontally at the relay; governance scales via contract design (batch tallies, event indexing).

### Stack
- **Frontend**: Next.js (React), wagmi/viem (or SL wallet hooks), Zustand for state, WebCrypto/Libsodium for E2E, shadcn/ui for components.
- **Backend**: Node.js (NestJS/Fastify) or Rust (Axum) for the relay; Redis for presence/rate-limits.
- **Blockchain**: **Soundness Layer (SL)** for auth/zk/reputation/governance. (Optional: read-only bridges to EVM/L2s for reputation imports.)
- **Storage**: 
    **WALRUS/IPFS** for encrypted message blobs (short TTL by default).
    **PostgreSQL** (minimal) for non-sensitive channel metadata & message indices.
    **Redis** for ephemeral presence, queues, and spam throttling.

### Features
1. **Anonymous, SL-Verified Membership** – Log in with wallet; prove membership via **zkProof** without exposing the address.
2. **End-to-End Encrypted Group Chat** – Clients encrypt; relay only routes ciphertext. Optional WALRUS/IPFS pinning.
3. **In-Chat Governance (Polls & Votes)** – Create polls and cast **zkVotes** directly inside the chat; on-chain tally on SL.
4. **Sybil Resistance & Rate Limits** – Proof-gated posting, per-member quotas, and relay-side throttling backed by SL credentials.
5. **Reputation-Weighted Voting (Optional)** – Use SL on-chain reputation to weight votes while keeping voter identities private.
6. **Role-Gated Channels** – Access controlled by SL proofs (e.g., “Core”, “Contributors”, “Holders”) instead of doxxed identities.
7. **Moderator Tools Without Doxxing** – Mute/kick/ban by proof-bound handles; appeals use fresh proofs.
8. **Auditability with Privacy** – Public poll results and contract events; private voter identities.
9. **Ephemeral Mode** – Auto-expire message blobs; only governance state persists on SL.
10. **Dev-Friendly Hooks** – SDK hooks for: useSLAuth(), useZkMembership(), usePolls(), useZkVote().

## Timeline

### PoC (2-4 weeks)

Implement **basic authentication** with Soundness Layer SDK.

Enable **zkProof-based membership verification.**

Build simple encrypted chat prototype (send/receive messages).

Provide a **minimal UI** for login, chat, and basic poll creation.

### MVP (4-8 weeks)  

Expand to **full feature set**: group chat, integrated polls, anonymous voting, role-gated channels.

Optimize **SL integration** (zkProofs, governance contract, on-chain reputation).

Design a **production-ready UI/UX** with smoother onboarding.

Conduct **user testing** with small communities to validate usability and scalability.

## Innovation
**What makes this unique?**

  **AnonDAO** Chat is not just another messaging app. Its uniqueness comes from **combining encrypted group chat with anonymous, zkProof-based governance.**
  Most platforms split communication (Discord/Telegram) and governance (Snapshot, Tally). AnonDAO Chat unifies them into **one seamless experience.**
  Instead of revealing wallet addresses or personal data, users prove their membership and reputation through **Soundness Layer zkProofs.**
  Voting happens directly inside the chat, with results written on-chain for **transparency and immutability** while maintaining user privacy.

**Why will people use it?**

  **DAO efficiency** → Communities save time and avoid fragmentation by having discussion and decision-making in the same place.
  
  **Privacy & trust** → Members don’t have to expose their wallets or identities, but can still be trusted participants.
  
  **Anti-bot & Sybil resistance** → SL verification ensures that only legitimate members can chat and vote, reducing spam.
  
  **Transparency** → Polls and results are verifiable on-chain, giving communities more confidence in governance outcomes.


## Contact

- **Preferred Contact Method:** Discord (DMs open)
- **Discord ID:** ihavemage
- **GitHub:** @ihavemage


**Checklist before submitting:**
- [x] All fields completed
- [x] GitHub username matches PR author  
- [x] SL integration explained
- [x] Timeline is realistic
