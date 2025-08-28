# vApp Submission: Proof-of-Activity NFT

## Verification
```yaml
github_username: "Odienurman"
discord_id: "870680093138243605"
timestamp: "2025-08-28"
```

## Developer
- **Name**: Odie Nurman
- **GitHub**: @Odienurman
- **Discord**: 870680093138243605
- **Experience**: Full-stack developer with 3+ years of experience in blockchain development. Skilled in building smart contracts, Web3.py integrations, and proof-of-concept vApps. Focused on exploring identity, reputation, and on-chain gamification

## Project

### Name & Category
- **Project**: Proof-of-Activity NFT
- **Category**: other (identity + social)

### Description
Proof-of-Activity NFT is a vApp that issues non-transferable NFTs (soulbound) as proof of user contributions on testnet.
Problem: It is difficult to measure user participation & contributions in testnet environments. Rewards are usually meaningless faucet tokens.
Solution: Every time a user performs a specific action (e.g., deploy a contract, join a campaign, vote, or make a transaction), the system verifies the activity via Soundness Layer attestation and then mints a unique NFT as on-chain reputation proof.

### SL Integration  
-Uses Soundness Layer attestations to validate user activity before minting the NFT.
-NFTs can only be obtained if the action is verified as valid.
-User reputation becomes transparent and auditable through SL.

## Technical
Architecture
-User performs an on-chain action.
-Backend/relayer sends a request to SL for verification.
-If valid → smart contract mints a soulbound NFT to the user.
-NFT metadata (activity type, timestamp, campaign ID) is stored on IPFS/WALRUS.

### Stack
-Frontend: React + Tailwind
-Backend: Node.js / Python FastAPI
-Blockchain: Soundness Layer + target testnets (BNB / Sei / others)
-Storage: WALRUS + IPFS

### Features
1.Soulbound NFTs as proof of activity
2.SL-based verification for user actions
3.Dashboard to explore user NFTs & reputation

## Timeline

### PoC (2-4 weeks)
- [✅] Basic soulbound NFT contract
- [✅] SL attestation integration
- [✅ ] Simple UI for minting

### MVP (4-8 weeks)  
- [✅] Multi-activity NFT support
- [✅] Explorer dashboard
- [✅] Reputation scoring from NFTs
- [✅] User testing

## Innovation
Unlike faucet tokens or generic badges, this vApp provides verifiable, non-transferable NFTs as lasting identity markers.
Backed by Soundness Layer, they are resistant to manipulation and useful for future dApps, governance, and community recognition.

## Contact
Discord : odienurman#1234
GitHub  : @Odienurman
Email   : odienurman.dev@example.com

**Checklist before submitting:**
- [✅] All fields completed
- [✅] GitHub username matches PR author  
- [✅] SL integration explained
- [✅] Timeline is realistic