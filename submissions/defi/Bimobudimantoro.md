# vApp Submission: DeFlow

## Verification

- `github_username`: "Bimobudimantoro"  
- `discord_id`: "918193119864971284"  
- `timestamp`: "2025-08-28"

## Developer

- **Name**: Bimo Budimantoro  
- **GitHub**: @Bimobudimantoro  
- **Discord**: bimobb  
- **Experience**:  
  Backend developer with 4 years in blockchain infrastructure and DeFi, delivering smart contract tooling and on-chain analytics with expertise in Solidity, Node.js, zkVMs, and data pipelines.

## Project

### Name & Category
- **Project**: DeFlow  
- **Category**: defi / identity

### Description
DeFlow solves the lack of unified credit reputation in DeFi. User activity remains siloed across protocols, limiting trust and incentive design.  
DeFlow aggregates on-chain behavior to compute a privacy-preserving credit score using zk proofs, allowing users to present verifiable, zero-knowledge attestations of reliability to multiple DeFi platforms without revealing their full transaction history.

### SL Integration

- **Proof Storage & Attestation**: DeFlow will submit zk proofs to Walrus, the decentralized storage network, and then use Soundness Layer on Sui as the verification coordinator, benefiting from low-cost, fast, censorship-resistant proof attestation (soundness.xyz).  
- **Seamless UX**: Inspired by the Soundness testnet launch (“Play ZK”), users will generate proofs (via custom logic rather than game solves), then submit to SL through a CLI or UI flow similar to the Discord-based experience (soundness.xyz).  
- **Verifier Integration**: Leveraging sp1-sui's Groth16 verifier in Sui Move smart contracts allows DeFlow to verify zk proofs directly on-chain within Sui’s Move ecosystem.

## Technical

### Architecture

1. **User Interaction**: Users connect via frontend, trigger DeFlow to compute a zk credit score proof (e.g., using SNARK-friendly circuits via SP1 zkVM).  
2. **Proof Lifecycle**: Proof gets uploaded to Walrus → SL validates via Sui’s Move smart contract using sp1-sui Groth16 verifier.  
3. **Attestation & Use**: Soundness Layer produces attestations. These can be consumed by DeFi protocols to offer dynamic rates or credit lines.  
4. **Cross-chain Potential**: SL’s cross chain capability means attestations could also be verified on chains like Ethereum via light-client bridges.

### Stack

- **Frontend**: React + web3 wallet integration  
- **Backend**: Node.js for orchestration + proof generation  
- **Blockchain**:
  - Soundness Layer (via Sui Move contracts)  
  - SP1 zkVM + sp1-sui verifier for zk proof flow  
- **Storage**: IPFS/Walrus for proof blobs  
- **Proof System**: Groth16 or SNARK via SP1 zkVM ecosystem

### Features

1. zk-based Credit Score: On-chain proofs of aggregated DeFi activity without exposing user history.  
2. sp1-sui Verifier Integration: On-chain Groth16 proof verification using Sui contracts.  
3. Soundness Layer Attestation: Low-cost, fast, decentralized proof validation with cross-chain potential.

## Timeline

### PoC (2–4 weeks)

- [x] Build a circuit (e.g., scoring logic), generate zk proof  
- [x] Upload proof to Walrus, submit to SL via CLI  
- [x] Use sp1-sui Move contract to verify proof on Sui

### MVP (4–8 weeks)

- [x] UI for proof generation and SL submission (inspired by the Discord flow)  
- [x] Handle attestations from SL, integrate with one DeFi protocol (e.g., lending protocol)  
- [x] Add audit logging and basic user testing

## Innovation

- Unified DeFi Reputation: Bridges siloed activity across platforms with privacy-preserving zk scores.  
- Composable, verifiable identity + credit layer: Leverages cutting edge SP1 verification in Sui.  
- Cost effective scalability: Employs Soundness Layer’s decentralized storage and verifiers to keep costs minimal and latency low (soundness.xyz, GitHub, soundness.xyz).  
- Engaging UX: Inspired by “Play ZK” — fun, simplified flow for users to submit and share proof attestations.

## Contact

- Discord: bimobb  
- Twitter/X: @bimo96

## References

- sp1-sui: Enables Groth16 proof verification in Sui Move using SP1 zkVM outputs (soundness.xyz, GitHub).  
- Soundness Layer & Walrus: Provides decentralized, fast, low-cost proof storage and attestation (soundness.xyz).  
- ZK Proof Primer: The “Where’s Waldo?” blog effectively illustrates ZK basics—foundation for why Soundness Layer matters (soundness.xyz).
