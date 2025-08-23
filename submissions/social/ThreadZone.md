# vApp Submission: ThreadZone

## ✅ Verification
- **github_username**: "fhif312"
- **discord_id**: "828688642385248287"
- **timestamp**: "2025-08-23"

## 👨‍💻 Developer
- **Name**: Tuti Hernawati
- **GitHub**: @fhif312
- **Discord**: 0xdiks
- **Experience**: 2 years building dApps in Ethereum ecosystem; Solidity + fullstack JS dev with experience in Lens Protocol and IPFS.

---

## 🚀 Project

### Name & Category
- **Project**: ThreadZone
- **Category**: social

---

### 📄 Description
ThreadZone is a decentralized microblogging platform where users can create and reply to posts (threads), follow others, and build token-gated communities. It solves the centralization problem in current social media platforms by ensuring all content, reputation, and relationships are onchain and owned by the user.

Users log in with their wallet, post content that’s stored on decentralized storage, and build their onchain identity via reputation. Each community can have its own moderation and monetization rules through DAO governance.

---

## 🔗 SL Integration
ThreadZone will leverage Soundness Layer (SL) for:
- **Onchain post storage** with WALRUS-backed efficient rollup
- **Gas abstraction** so users can post without needing ETH
- **Proof-based moderation** for verifying post authenticity without revealing content (zk moderation)
- **Reputation calculation** using verifiable compute (ZK circuits)

SL allows scalable, censorship-resistant communication with a better UX and lower cost.

---

## ⚙️ Technical

### Architecture
- **Frontend**: Users interact with a web app (React) via wallet login.
- **Backend**: Minimal, stateless backend for indexing and notifications (using SL+Push).
- **Smart Contracts**: Manage posts, profiles, follows, and communities on SL.
- **SL Integration**: Posts and social graphs are written to SL with proof generation.
- **Storage**: Text content stored via WALRUS / IPFS with SL attestation.

### Stack
- **Frontend**: React + RainbowKit + Tailwind
- **Backend**: Node.js + GraphQL
- **Blockchain**: Soundness Layer, optional integration with Polygon
- **Storage**: WALRUS (via SL), IPFS fallback

---

### 💡 Features
- Onchain posts and comments with zk-proof moderation
- Token-gated communities with DAO control
- Reputation and leaderboard system
- Gasless interactions via SL

---

## 🕒 Timeline

### 🔹 PoC (2–4 weeks)
- Post / reply features
- Wallet login
- SL write integration (basic WALRUS post)
- Simple UI

### 🔸 MVP (4–8 weeks)
- Token-gated threads
- Onchain reputation + tipping
- Push Protocol for notifications
- User testing and feedback

---

## ✨ Innovation
Most decentralized social platforms focus on broadcasting (like Twitter onchain). ThreadZone flips the focus to *threaded micro-communities*, making it more personal and community-driven. Using SL's zk features, we can ensure moderation, identity, and content storage is transparent, scalable, and resistant to manipulation.

---

## 📬 Contact
- **Preferred**: Discord (0xdiks)

---

## ✅ Checklist

- [x] All fields completed  
- [x] GitHub username matches PR author  
- [x] SL integration explained  
- [x] Timeline is realistic  

