# vApp Submission: SoundQuest

## Verification
```yaml
github_username: "papacz"
discord_id: "427825176605097984"
timestamp: "2025-08-22"
```

## Developer
- **Name**: papacz
- **GitHub**: @Ajipur-Oprekers
- **Discord**: papacz
- **Experience**: I graduated from a university in Indonesia, majoring in information technology, and am an active tester in AI inference projects, a trap operator on the Ethereum network, securing the network, and acting as a storage node in the crypto industry.


## Project

### Name & Category
- **Project**: SoundQuest (Proof-of-Learn)
- **Category**: Other

### Description
This is an onboarding platform for the new Soundness ecosystem. Every new project joining the ecosystem will be listed here. New users can learn about the project and use it as a reference for a project to determine whether users are fully familiar with it. The mechanism involves users connecting their wallets, answering a series of random questions, and then generating a zero-knowledge proof that all answers are correct without revealing the answers. If they pass, users can mint an NFT as a "certificate of completion" or a badge of participation in the respective project in the future.

### SL Integration  
1. The client (browser) uses Ligero to generate a zero-knowledge proof that the answers provided match the correct answer.
2. The proof is uploaded to Walrus and verified by the Soundness Layer, which provides attestation.
3. The backend then verifies the attestation through the Soundness CLI. If valid, a smart contract on the Hoodi testnet mints the NFT and embeds the proof ID from Walrus in the metadata.

## Technical

### Architecture
1. Frontend (React) – Handles wallet connection, quiz display, answer input, and function calls to generate the proof.
2. Backend (Node.js + MongoDB) – Provides an API to randomly select questions, store the question bank and quiz results, and call the Soundness CLI to upload proofs/attestations.
3. Blockchain (SL + Hoodi testnet) – The Soundness Layer verifies proofs; an ERC-721 contract on the Hoodi testnet mints completion NFTs.
4. Storage – MongoDB stores questions and user status; Walrus stores proof data; IPFS stores NFT metadata (quiz title, badge image, timestamp).


### Stack
- **Frontend**: React
- **Backend**: Node.js
- **Blockchain**: SL + Hoodi Testnet (Migrate Mainnet SUI after full tested)
- **Storage**: MongoDB + Walrus + IPFS

### Features
1. Personalized quizzes – Users connect their wallets, receive random sets of questions from specific projects, and answer them through an interactive interface.
2. Zero-Knowledge Proof & Verification – After answers are entered, the app generates a ZK proof that all answers are correct and then sends the proof to the Soundness Layer for verification.
3. NFT Certificate of Completion – If the proof is valid, a contract on the Hoodi testnet mints a unique NFT containing project metadata and the proof ID, serving as a reputation badge for the user.

# Timeline

### PoC (2-4 weeks)
- [ ] Develop a simple UI for the quiz (10 static questions) and wallet connection.
- [ ] Create a basic ZK circuit to check for correct answers.
- [ ] Integrate proof upload to Walrus via the Soundness CLI and verify one question type.

### MVP (4-8 weeks)  
- [ ] Add a dynamic question bank, question randomization, and multi-project support.
- [ ] Deploy an ERC-721 smart contract on the Hoodi testnet to mint NFTs.
- [ ] Full integration with the Soundness Layer and user testing.
- [ ] (Optional) Prepare a production environment (database, IPFS) for use by multiple projects.

## Innovation
This app is unique because users can prove they understand the material without revealing the answers, while new projects get an engaging way to introduce their features or let the public know who they are. Graduation NFTs serve as verified credentials that can be used to gain access to premium features, such as:
1. Participating in airdrops (according to each project's terms and conditions)
2. Onchain Badge
   *This could be an option for other socialFi platforms. For example, project A runs an onchain mission on galxe, but answering the questions requires learning first, and proof of learning is based on the NFT minted.

By utilizing the Soundness Layer as a verification layer, this vApp showcases the network's ability to prove knowledge while maintaining privacy.

## Contact
1. Telegram: @PapaCZ
2. Discord: papacz
3. X: @Ajipur26

**Checklist before submitting:**
- [x] All fields completed
- [x] GitHub username matches PR author  
- [x] SL integration explained
- [x] Timeline is realistic
