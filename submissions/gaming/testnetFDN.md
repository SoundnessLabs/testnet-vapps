# vApp Submission: [Your Project Name]

## Verification
```yaml
github_username: "testnetFDN"
discord_id: "1082637970328272927"
timestamp: "2025-09-27"
```

## Developer
- **Name**: nasikin  
- **GitHub**: @testnetFDN   
- **Discord**: 0xidn        
- **Experience**: Web3/NFT enthusiast, focusing on campaign NFTs and modular contract design.

## Project

### Name & Category
- **Project**: NFTs           
- **Category**: gaming

### Description
A profile picture (PFP) NFT collection for GTD testnet participants.

Total Supply: 5,000 NFTs
Minting Phases:
 - Phase 1 — GTD (Guaranteed Allocation): 1,000 NFTs reserved for top leaderboard testers.
 - Phase 2 — Public FCFS: 4,000 NFTs, for tester.
Minting Cost: Free (gas only).
Utility: Recognition NFTs for early GTD testers, designed as PFPs with traits, extendable to future use cases.

### SL Integration  
- Integrate Soundness Layer zk-proofs to validate leaderboard eligibility for the GTD phase.
- FCFS mint does not require proof verification.

## Technical

### Architecture
1. Deploy an ERC-721A contract with fixed max supply = 5,000.
2. Define two minting phases:
 - Phase 1 (GTD): whitelist addresses for top testers, max supply = 1,000.
 - Phase 2 (FCFS): open minting, max supply = 4,000.
3. Minting ends automatically when total supply reaches 5,000.
Events:
 - Minted(address, tokenId) for mint tracking.
 - PhaseStarted(uint256 phaseId, uint256 supply) for clarity.

### Stack
- **Frontend**: React/Next.js + wagmi (wallet connect + mint UI).
- **Backend**: Optional for proof verification during GTD phase.  
- **Blockchain**: Solidity (OpenZeppelin ERC721A).
- **Storage**: IPFS/Arweave for artwork & metadata (PFP format).

### Features
Fixed Supply = 5,000 NFTs.
Phase 1 GTD = 1,000 guaranteed NFTs for top testers.
Phase 2 FCFS = 4,000 free-mint NFTs for the testers.
PFP Utility — designed as a profile picture NFT with traits/rarity.
Proof Validation — optional zk-proof check for GTD eligibility.
