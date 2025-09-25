# vApp Submission: SoundSync

## Verification
```yaml
github_username: "nativetams"
discord_id: "957427118571147385"
timestamp: "2025-09-25"
```

## Developer
- **Name**: Taisia Ivanova
- **GitHub**: @nativetams
- **Discord**: ivanovataisia
- **Experience**: 5 years of full-stack and Web3 development.

## Project

### Name & Category
- **Project**: SoundSync
- **Category**: social

### Description
SoundSync is a decentralized social media platform for sharing and monetizing short-form audio clips. It solves the problem of content ownership and creator monetization by giving creators full control and direct earnings from listeners.

### SL Integration  
We'll use the Soundness Layer (SL) for an immutable and verifiable record of content ownership and engagement. The Verifiable Log will timestamp uploads, and the Shared Global State will track listening events for transparent, automated royalty distribution.



## Technical

### Architecture
The platform will use a modular design. The React frontend interacts with a Rust backend, which handles authentication. Core logic for content registration and monetization is directly tied to the SL. Audio files are stored on IPFS, with SL holding only the immutable metadata.


### Stack
- **Frontend**: React
- **Backend**:  Rust (Rocket)
- **Blockchain**: Soundness Layer (SL)
- **Storage**: IPFS, PostgreSQL

### Features
1. Audio Upload and Timestamping: Clips are timestamped and verified on the SL.
2. Decentralized Monetization: Users can tip creators, with transactions recorded on the SL.
3. Immutable Engagement Metrics: Listening metrics are logged on the SL for transparency.

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
SoundSync's primary innovation is using the Soundness Layer to ensure verifiable ownership and transparent monetization. It empowers creators by removing the need for a centralized intermediary, giving them direct control and fair earnings. People will use it for its trustless and transparent model.

## Contact
Discord: ivanovataisia


**Checklist before submitting:**
- [ ] All fields completed
- [ ] GitHub username matches PR author  
- [ ] SL integration explained
- [ ] Timeline is realistic
