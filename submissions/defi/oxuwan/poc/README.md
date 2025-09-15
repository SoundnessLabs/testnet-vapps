# MusicNFT PoC

## Overview
This PoC demonstrates a simple NFT marketplace concept for music tracks on top of the **Soundness Layer**.  
Musicians or creators can mint their audio tracks as NFTs, which fans and collectors can trade or hold.

## Features
- Mint audio NFTs (with title and URI pointing to Soundness Layer / IPFS / blob://...).
- Store track metadata (title + URI).
- Retrieve metadata with `getTrack()` function.
- Ownership controlled by contract owner.

## Smart Contract
- File: `MusicNFT.sol`
- Based on **ERC721** standard from OpenZeppelin.
- Written in **Solidity 0.8.x**.

## How to Use
1. Deploy the contract with Remix, Hardhat, or Foundry.
2. As the contract owner, call:
   ```solidity
   mintTrack(recipient, "Track Title", "blob://track-uri")
   getTrack(tokenId)
