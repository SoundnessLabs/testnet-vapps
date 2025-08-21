# vApp Submission: NGLSound

## Verification
```yaml
github_username: "bromiums"
discord_id: "923151122246557726"
timestamp: "2025-08-21"
```

## Developer
- **Name**: Bagas Rizki Gunardi
- **GitHub**: @bromiums (@w1th0ut is my Github main account)
- **Discord**: bromiums
- **Experience**: Full-stack blockchain developer with experience in Solidity smart contracts, React/Next.js frontend development, and Web3 integration. Skilled in building decentralized applications with focus on user privacy and security.

## Project

### Name & Category
- **Project**: NGLSound
- **Category**: social

### Description
NGLSound is a Web3 anonymous messaging platform that solves the problem of privacy in digital communication. Similar to NGL.link but built on Soundness Layer blockchain technology, it enables users to send and receive anonymous messages without revealing their identity. The platform provides a secure, decentralized alternative to traditional anonymous messaging services by leveraging SL's privacy features and client-side encryption. Users can register once, get their unique shareable link, and receive encrypted anonymous messages that only they can decrypt using their wallet address as the decryption key.

### SL Integration  
NGLSound will integrate with Soundness Layer to enhance the security and verification of anonymous messages. Specific SL features to be utilized:

1. **Message Integrity Verification**: Using SL's cryptographic proofs to ensure messages haven't been tampered with during transmission
2. **Zero-Knowledge Message Validation**: Implementing SL's ZK capabilities to verify message authenticity without revealing sender identity
3. **Decentralized Identity Verification**: Leveraging SL's identity layer to verify user registration without compromising anonymity
4. **Cross-Chain Message Verification**: Using SL's interoperability features to enable secure message verification across different blockchains
5. **Privacy-Preserving Analytics**: Implementing SL's privacy features to provide platform analytics without compromising user privacy

## Technical

### Architecture
NGLSound follows a decentralized architecture with three main layers:

1. **Smart Contract Layer (Soundness Layer)**:
   - Contract: Core contract handling user registration, message storage, and fee management
   - Registration fee: fee per user (one-time)
   - Message fee: fee per message
   - Username mapping system for user-friendly addressing

2. **Frontend Layer (Next.js)**:
   - React-based SPA with responsive design
   - Web3 wallet integration (MetaMask, Rabby)
   - Client-side AES encryption/decryption
   - Real-time message fetching and display

3. **Encryption Layer**:
   - Client-side encryption using CryptoJS AES
   - Wallet address as encryption/decryption key
   - Messages encrypted before blockchain storage
   - Only recipient can decrypt using their private wallet

### Stack
- **Frontend**: Next.js, React, TailwindCSS
- **Smart Contracts**: Solidity, Foundry framework for testing and deployment
- **Blockchain**: Soundness Layer Testnet with native SL integration
- **Web3 Integration**: Ethers.js, Wagmi, RainbowKit
- **Encryption**: CryptoJS for client-side AES encryption
- **Storage**: On-chain storage via smart contracts, no external databases required

### Features
1. **Anonymous User Registration**: One-time registration with automatic username generation based on wallet address
2. **Custom Username System**: Users can set custom usernames for easier sharing and recognition
3. **Encrypted Message Storage**: All messages encrypted client-side before on-chain storage using recipient's wallet address
4. **Shareable User Links**: Each user gets a unique shareable link (domain.com/msg?username=user) for receiving messages
5. **Message Dashboard**: Personal dashboard for viewing all received messages with pagination
6. **Pay-per-Message**: Fee per message sent to prevent spam
7. **Multi-Wallet Support**: Compatible with MetaMask, Rabby, and other Web3 wallets
8. **Responsive UI**: Modern, animated interface with parallax effects and mobile optimization
9. **Real-time Updates**: Live message fetching and wallet connection status updates

## Timeline

### PoC (2-4 weeks)
- [ ] Basic smart contract with registration and messaging
- [ ] Core encryption/decryption functionality
- [ ] Simple web interface with wallet connection
- [ ] Initial SL integration for message verification
- [ ] Username system implementation

### MVP (4-8 weeks)  
- [ ] Complete feature set (registration, messaging, dashboard)
- [ ] Production-ready smart contracts with comprehensive testing
- [ ] Polished UI with animations and responsive design
- [ ] Multi-wallet support and error handling
- [ ] Full SL integration with ZK proofs
- [ ] Advanced privacy features using SL

## Innovation
NGLSound brings several unique innovations to the anonymous messaging space:

1. **True Decentralization**: Unlike centralized services like NGL.link, all data is stored on-chain with no central servers that can be compromised or censored

2. **Wallet-Based Encryption**: Uses the recipient's wallet address as the encryption key, ensuring only the intended recipient can decrypt messages without requiring separate key management

3. **Economic Spam Prevention**: Small message fees naturally prevent spam while keeping costs minimal for legitimate users

4. **Username Flexibility**: Users can choose between wallet addresses or custom usernames for sharing, providing both privacy options and user-friendly alternatives

5. **Soundness Layer Integration**: Planned integration with SL for advanced privacy features and cross-chain capabilities, setting it apart from simple messaging apps

6. **Self-Sovereign Identity**: Users maintain complete control over their data and privacy without relying on third-party services

People will use NGLSound because it offers the convenience of anonymous messaging with the security guarantees of blockchain technology, ensuring their communications cannot be censored, monitored, or controlled by centralized entities.

## Contact
Preferred contact method and where you'll share updates.

Discord: @bromiums

X: @bromiums

**Checklist before submitting:**
- [x] All fields completed
- [x] GitHub username matches PR author  
- [x] SL integration explained
- [x] Timeline is realistic
