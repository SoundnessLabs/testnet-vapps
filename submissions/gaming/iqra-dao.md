# Soundness Gaming: zk-Waldo

## Overview
**zk-Waldo** is a zero-knowledge proof based game demo where a player can prove that they found "Waldo" in an image — without revealing the actual location.  
This demonstrates how zero-knowledge proofs can enable privacy-preserving gameplay on the Soundness Layer.

## Category
- gaming

## Motivation
- Showcase how zero-knowledge proofs can be applied in gaming.
- Players can prove game outcomes without revealing sensitive information.
- zk-Waldo highlights Soundness Layer’s developer experience and scalability.

## Technical Architecture
1. **Frontend (Game UI):** Player receives an image with hidden Waldo.
2. **Proof Generation:** Once Waldo is found, a zk circuit generates a proof.
3. **Verification:** The proof is verified by a Soundness Layer contract.
4. **Result:** The player can prove they found Waldo without revealing its location.

## Integration with Soundness Layer
- Uses Soundness Layer zkApp framework.
- On-chain verifier contract validates proofs.
- Deployed on Soundness testnet as a demo.

## Development Timeline
- **Week 1–2:** Game UI + Circuit Design  
- **Week 3:** Proof generation integration  
- **Week 4:** Testnet deployment on Soundness Layer  
- **Week 5:** Testing and documentation  

## Team
- GitHub: [iqra-dao](https://github.com/iqra-dao)  
- Discord ID: *(maakhadiza)*  

## Future Scope
- Multiplayer puzzle-based gaming with zk proofs  
- NFT rewards integration  
- Privacy-preserving leaderboards  

---
