# zk-Chess

## Summary
zk-Chess is a verifiable game dApp built on top of the Soundness layer.  
The goal is to showcase how zero-knowledge proofs can be applied in interactive games, where each move is validated off-chain but can be verified on-chain.

## Motivation
- Provide a fun and practical demo for new developers in the Soundness ecosystem.  
- Demonstrate how Soundness CLI can be used for proof generation.  
- Expand the library of vApps beyond financial or utility use-cases.

## Technical Approach
- **Language / Framework**: Rust + WASM for computation.  
- **Integration**: Soundness CLI generates proofs for each valid move.  
- **Flow**:
  1. A player makes a move.
  2. The program checks if the move is valid.
  3. A proof is generated using Soundness CLI.
  4. The proof + move are submitted for verification.
  
- **Output**: Verified move history that cannot be tampered with.  

## Roadmap
- [ ] Step 1: Build minimal chess move validator in Rust.  
- [ ] Step 2: Integrate Soundness proof generation.  
- [ ] Step 3: Create simple UI for demonstration.  
- [ ] Step 4: Deploy demo and share with community.  

## GitHub Handle
@ze3m
