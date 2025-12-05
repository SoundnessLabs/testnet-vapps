# vApp Submission: [Your Project Name]

## Verification
```yaml
github_username: "pael666"
discord_id: "pa3l"
timestamp: "2025-12-05"
```

## Developer
- **Name**: Raphael Pardamean
- **GitHub**: @pael666
- **Discord**: pa3l
- **Experience**: I’ve been exploring Web3 development by experimenting with small prototypes, especially around verification and content authenticity. Recently I’ve been interested in how creators can prove ownership of their work without revealing the content itself, which inspired the idea for ProofLens.

## Project

### Name & Category
- **Project**: ProofLens
- **Category**: infrastructure

### Description
With AI-generated content becoming more common, it’s getting harder to tell whether someone truly created the text or images they post. ProofLens gives users a simple way to prove authorship by hashing content locally and generating an attestation through the Soundness Layer.
This lets anyone verify the claim without needing the original file.

### SL Integration  
ProofLens uses the Soundness Layer to generate verifiable, tamper-proof authorship attestations. SL allows users to create proofs using only a hash, keeping the content private. Other apps can independently verify these attestations through SL, making authorship claims portable and trustworthy.

## Technical

### Architecture
The frontend handles uploading and hashing content locally.
The backend receives the hash and communicates with the Soundness Layer to generate an attestation.
A verification endpoint allows third-party applications to confirm whether a content hash has a valid proof.
Optional decentralized storage (WALRUS/IPFS) can be used if users want to save their content publicly.

### Stack
- **Frontend**: React / Next.js
- **Backend**: Node.js
- **Blockchain**: SL
- **Storage**: WALRUS / IPFS

### Features
1. Local content hashing
2. Authorship attestation  
3. Simple dashboard for proof history

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
ProofLens focuses on a straightforward but important issue: proving who created something in a world filled with AI-generated content. Instead of detecting AI, it gives creators a cryptographic way to say, “I made this,” backed by an independent attestation from the Soundness Layer.

## Contact
Preferred contact: Discord (pa3l). Updates will be shared through GitHub (@pael666).


**Checklist before submitting:**
- [ ] All fields completed
- [ ] GitHub username matches PR author  
- [ ] SL integration explained
- [ ] Timeline is realistic
