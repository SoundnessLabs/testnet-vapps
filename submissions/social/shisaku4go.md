# vApp Submission: Vibe Meter

## Verification

```yaml
github_username: shisaku4go
discord_id: 1104356122468548641
timestamp: "2025-08-24"
```

## Developer

- **Name**: robobuncho
- **GitHub**: @shisaku4go
- **Discord**: robo
- **Experience**: I'm a software developer with over 10 years of experience in web2 development. I'm also a self-taught web3 developer, and my main focus is on the Sui and Polygon (EVM-compatible) blockchains.

## Project

### Name & Category

- **Project**: Vibe Meter
- **Category**: social

### Description

Vibe Meter is a social discovery tool that allows users to find people with similar interests without directly revealing personal data. Users create a list of words or categories representing their interests. The application then uses zero-knowledge proofs to securely compare interests, enabling a low-pressure and low-risk way to find potential friends or collaborators.

### SL Integration

Vibe Meter will use the Soundness Layer (SL) to verify the zero-knowledge proofs. The user's list of words will be used to generate a zk-proof using the Ligetron SDK. This proof will then be submitted to the SL to verify its validity and ensure the data's integrity. When two users want to compare their interests, their respective zk-proof IDs will be validated by the SL, and a similarity score will be computed and displayed to both users. This process ensures that the comparison is secure and verifiable without either user's raw data being exposed.

## Technical

### Architecture

The user will interact with a web-based frontend. When a user submits their list of words, the frontend will use the Ligetron SDK to create a zero-knowledge proof locally in the browser. This proof will be stored in a decentralized storage solution like Walrus. A unique proof ID will be generated and associated with the user.

To compare interests, two users will input their respective proof IDs. The application will then use a backend service to access and verify both proofs via the Soundness Layer. A comparison algorithm will compute a similarity score based on the verified proofs, which will be returned to the frontend. The original data (the lists of words) will never leave the user's browser, maintaining privacy.

### Stack

- **Frontend**: React (Walrus Sites)
- **Backend**: Node.js + wasm (AWS Lambda)
- **Blockchain**: SL + Sui
- **Storage**: Walrus

### Features

1. Word Input: A web interface where users can paste a list of 10-100 words.
2. ZK-Proof Generation: Use the Ligetron SDK to generate a zero-knowledge proof from the list of words.
3. Proof Storage: Securely save the zk-proof to the Walrus site.
4. Interest Comparison: Allow users to input another user's proof ID to compare interests and display a general compatibility score.

## Timeline

### PoC (2-4 weeks)

- [ ] Implement the UI for word list input.
- [ ] Integrate Ligetron SDK to generate a basic zk-proof.
- [ ] Connect to Walrus for saving and retrieving the proof.
- [ ] Integrate with the Soundness Layer to verify a single proof.

### MVP (4-8 weeks)

- [ ] Implement the full end-to-end flow, including comparing two proofs and displaying a similarity score.
- [ ] Refine the UI and user experience.
- [ ] Conduct preliminary user testing and gather feedback.
- [ ] Optimize the proof generation and verification process for efficiency.

## Innovation

Vibe Meter's primary innovation lies in its unique approach to social connection. Unlike traditional social media platforms that rely on explicit data sharing, Vibe Meter leverages zero-knowledge proofs to enable a privacy-first, trustless form of social discovery. This approach makes it a fun and secure way for people to find others with similar interests, reducing the pressure and privacy concerns associated with traditional methods. The use of words as a proxy for interests provides a rich, global dataset for comparison, making the application widely accessible and scalable.

## Contact

I will share updates on this project via GitHub and Discord. For direct inquiries, you can reach me on Discord.

**Checklist before submitting:**

- [x] All fields completed
- [x] GitHub username matches PR author
- [x] SL integration explained
- [x] Timeline is realistic
