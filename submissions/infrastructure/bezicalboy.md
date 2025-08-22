# vApp Submission: Chronos Seal

## Verification
```yaml
github_username: "bezicalboy"
discord_id: "1280746472266141697"
timestamp: "2025-08-22"
```

## Developer

  - **Name**: Muhamad Rico Wijaya
  - **GitHub**: @bezicalboy
  - **Discord**: xentastix_05286
  - **Experience**: I am a passionate web developer with experience in JavaScript frameworks and an emerging interest in blockchain technology. I am looking to build a foundational and useful tool on the Soundness Layer.

## Project

### Name & Category

  - **Project**: Chronos Seal
  - **Category**: infrastructure

### Description

Chronos Seal provides a simple solution to a common problem: how to prove that a file or piece of data existed at a specific time without relying on a trusted third party. This vApp allows users to create a permanent, immutable, and timestamped proof of existence for any digital file by anchoring its cryptographic hash onto the Soundness Layer.

### SL Integration

This project is a direct and clear use of Soundness Layer's core function as a **decentralized verification layer**.

1.  **Verification & Timestamping**: A user generates a hash of their file on their local machine. They submit this hash in a transaction to a smart contract on Soundness Layer. SL's role is to **verify** and sequence this transaction, which acts as a secure, decentralized timestamping service.
2.  **Data Availability**: The transaction record containing the file's hash and the block's timestamp will be permanently stored and made available through the **Walrus** data availability layer, ensuring the proof can be audited by anyone, anytime.
3.  **Efficiency**: We will leverage the **low latency** and **high throughput** of the network to make timestamping fast and affordable for everyone.

## Technical

### Architecture

The architecture is intentionally minimalist to ensure it's easy to build and maintain.

1.  **Frontend Only**: A simple web interface where a user can select a local file.
2.  **Client-Side Hashing**: JavaScript in the browser will read the file and compute its SHA-256 hash. The file itself never leaves the user's computer.
3.  **Smart Contract Interaction**: The user connects their wallet and sends a transaction containing the hash to our simple smart contract on Soundness Layer. The contract just needs to store the hash against the sender's address and the timestamp.

### Stack

  - **Frontend**: Vanilla JavaScript, HTML, CSS (No complex frameworks needed)
  - **Blockchain**: Soundness Layer (SL)
  - **Backend**: None required

### Features

1.  **Seal File**: Generate a hash from a user-selected file and submit it to Soundness Layer to create a timestamped proof.
2.  **Verify File**: Allow a user to re-upload a file, generate its hash, and check if that hash exists on-chain to verify its timestamp.

## Timeline

### PoC (2-4 weeks)

  - [ ] Basic UI to select a file and generate its hash.
  - [ ] Connect to a wallet and call a smart contract on the SL testnet.
  - [ ] A simple page to view the transaction on an explorer.

### MVP (4-8 weeks)

  - [ ] A cleaner UI for both sealing and verifying files.
  - [ ] A feature to look up past seals made by the user's wallet address.
  - [ ] User testing and feedback.

## Innovation

The innovation of Chronos Seal is its simplicity and accessibility. It provides a fundamental, essential tool for the decentralized web. By using Soundness Layer, it offers a more efficient and purpose-built solution for verification compared to using a general-purpose blockchain. It's a foundational "lego block" that other developers or applications can rely on.

## Contact

The best way to contact me is on Discord at `xentastix_05286`.

**Checklist before submitting:**

  - [x] All fields completed
  - [x] GitHub username matches PR author
  - [x] SL integration explained
  - [x] Timeline is realistic

<!-- end list -->
