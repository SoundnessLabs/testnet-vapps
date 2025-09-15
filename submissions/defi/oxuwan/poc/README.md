# MusicNFT PoC

Minimal Proof of Concept smart contract for the Music NFT Marketplace vApp.

## Features
- Mint ERC-721 tokens for music tracks.
- Store title + URI (can point to Soundness Layer blob or IPFS).
- Functions: `mintTrack(recipient, title, uri)` and `getTrack(tokenId)`.

## How to Test
1. Compile + deploy with Hardhat/Remix.
2. Call `mintTrack(yourAddress, "Song Title", "blob://<your-blob-id>")`.
3. Check ownership and use `getTrack(tokenId)`.

This PoC shows how music can be tokenized and linked with Soundness Layer storage.
