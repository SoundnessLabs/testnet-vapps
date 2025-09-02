# vApp Submission: [ProofMe]

## Verification
```yaml
github_username: "Vertigoxx"
discord_id: "836448307185975346"
timestamp: "2025-09-02"
```

## Developer
- **Name**: Cristian Daren
- **GitHub**: @Vertigoxx
- **Discord**: Crsst.d
- **Experience**: Full-stack blockchain developer with experience in smart contract development, ZK-proofs integration, and Web3 frontend frameworks.

## Project

### Name & Category
- **Project**: ProofMe
- **Category**: identity

### Description
ProofMe is a decentralized identity verification platform that lets users prove things about themselves (age, student status, DAO membership, or event attendance) without revealing sensitive personal data.
Instead of uploading an ID to a central server, users generate zero-knowledge proofs that confirm facts like:

"I’m over 18"

"I own this NFT"

"I attended this conference"
without revealing their actual ID, wallet balance, or other private details.

This solves the problem of over-disclosure of identity in Web2 apps, and enables Web3 projects, dApps, and communities to integrate safe, privacy-preserving identity checks.

### SL Integration  
1. **Zero-Knowledge Proof Verification**: Using SL’s ZK capabilities to confirm user claims (e.g., age > 18, token ownership) without exposing raw data.
2. **On-Chain Attestation Registry**: Leveraging SL smart contracts to store minimal attestations for universal proof-of-identity across dApps.
3. **Privacy-Preserving Identity Layer**: Utilizing SL’s identity framework to allow verified credentials to be reused without deanonymizing users.
4. **Cross-App Credential Portability**: Harnessing SL’s interoperability to ensure a proof generated once can be trusted across multiple vApps.
5. **Aggregate Privacy Analytics**: Implementing SL’s privacy features to provide dApps with verified user statistics without compromising individual identities.

## Technical

### Architecture
ProofMe follows a decentralized architecture with four main layers:

1. **Smart Contract Layer (Soundness Layer)**:
   - Credential registry smart contract for storing minimal attestations
   - ZK verification contract for validating proofs on-chain
   - Role-based access control to manage different verifier types
   - Cross-app credential portability via SL interoperability

2. **Frontend Layer (Next.js)**:
   - Web app with user-friendly dashboard for creating and managing proofs
   - Web3 wallet integration (MetaMask, Rabby, WalletConnect)
   - Proof generation UI for age verification, DAO membership, and event badges
   - API widget generator for external dApps to integrate ProofMe easily

3. **Encryption LayerProof Generation Layer (Client-Side)**:
   - Local ZK circuit compilation and proof creation in the browser or client app
   - Selective disclosure logic (e.g., “over 18” without exposing DOB)
   - Lightweight Node.js service for optional off-chain proof generation
   - No sensitive raw data leaves the client device

4.  **Credential Layer (Storage & Reuse):**:
   - On-chain attestation records stored on SL
   - Optional credential metadata on IPFS (non-sensitive only)
   - Reusable proofs across different dApps and games in the SL ecosystem
   - Privacy-preserving analytics for aggregate stats without deanonymization 

### Stack
- **Frontend**: Next.js, React, TailwindCSS
- **Smart Contracts**: Solidity, custom ZK circuits, Hardhat/Foundry for testing & deployment
- **Blockchain**: Soundness Layer Testnet with native ZK integration, optional EVM compatibility
- **Web3 Integration**: Ethers.js, Wagmi, RainbowKit
- **Proof System**: zk-SNARKs / zk-STARKs for client-side proof generation and verification
- **Storage**: On-chain attestation registry via SL smart contracts, optional metadata on IPFS

### Features
1. **Anonymous Proof Generation**: Users can generate cryptographic proofs without exposing their identity.
2. **On-Chain Attestation Registry**: All proofs are stored immutably on-chain for transparency and verification.
3. **Zero-Knowledge Validation**: Proof authenticity verified using ZK circuits without revealing sensitive data.
4. **Customizable Proof Types**: Supports multiple use cases like KYC attestation, event participation, and skill verification.
5. **User Dashboard**: Personal dashboard for managing proofs, submissions, and verification history.
6. **Pay-per-Proof Model**: Small fee for each proof generation to prevent spam and ensure network sustainability.
7. **Cross-Chain Support**: Verifications can be validated across different blockchains via SL interoperability.
8. **Multi-Wallet Integration**: Works with MetaMask, Rabby, Rainbow, and other popular Web3 wallets.
9. **Responsive UI/UX**: Built with modern design, animations, and mobile-first optimization.
10. **Privacy-Preserving Analytics:**: Platform usage insights without compromising user anonymity.

## Timeline

### PoC (2-4 weeks)
- [ ] Basic smart contract for proof generation and storage
- [ ] Core ZK circuit for anonymous proof validation
- [ ] Simple web interface with wallet connection
- [ ] Initial SL (Soundness Layer) integration for verification
- [ ] User dashboard prototype with proof listing

### MVP (4-8 weeks)  
- [ ] Complete feature set (proof generation, on-chain registry, dashboard)
- [ ] Production-ready smart contracts with audits & testing
- [ ] Polished UI with animations and responsive design
- [ ] Multi-wallet integration (MetaMask, Rabby, Rainbow, etc.)
- [ ] Full SL integration with cross-chain proof validation
- [ ] Privacy-preserving analytics system

## Innovation
ProofMe introduces several key innovations in the decentralized verification space:

1. **Anonymous Proof Generation**: Users can prove authenticity without exposing personal details.

2. **Zero-Knowledge Validation**: ZK circuits ensure verification without revealing sensitive data.

3. **On-Chain Registry**: All proofs stored immutably, resistant to censorship or manipulation.

4. **Cross-Chain Verification**: SL integration allows proofs to be recognized across multiple blockchains.

5. **Economic Spam Prevention**: Small fees prevent abuse while keeping costs minimal.

6. **User-Centric Dashboard**: Manage, share, and track all proofs in a single interface.

7. **Self-Sovereign Identity**: Users fully control their proofs without third-party dependencies.

People will use ProofMe because it offers the security of blockchain, privacy of ZK, and ease of Web3 UX—ensuring verification is both trustless and user-friendly.

## Contact
Preferred contact method and where you'll share updates.

Discord: @crsst.d

X: @AnonNyid

**Checklist before submitting:**
- [x] All fields completed
- [x] GitHub username matches PR author  
- [x] SL integration explained
- [x] Timeline is realistic
