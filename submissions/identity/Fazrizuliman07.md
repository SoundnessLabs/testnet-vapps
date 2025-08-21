# zkAgeProof: Zero-Knowledge Age Verification

**Category:** identity

## Summary
zkAgeProof is a zkApp that allows users to prove they are above a certain age (e.g., 18+) without revealing their full date of birth.  
The proof is generated off-chain and verified on-chain through the Soundness Layer, ensuring privacy and trust.

## Problem
Most websites and apps require users to disclose sensitive personal information such as date of birth for age verification.  
This creates privacy risks, potential for data leaks, and unnecessary data sharing.

## Solution
zkAgeProof leverages zero-knowledge proofs to let a user prove their age is above a threshold without revealing exact details.  
- The user submits their date of birth locally (off-chain).  
- A zkProof is generated confirming whether the age > threshold.  
- The Soundness Layer validates the proof onchain.  
- The application receives only a "YES/NO" verification, not the raw data.

## Technical Architecture
- **Client**: User inputs their date of birth locally.  
- **ZK Circuit**: Generates proof (e.g., age >= 18).  
- **Soundness Layer**: Onchain attestation validates the proof.  
- **Verifier App**: Third-party apps (social platforms, KYC-lite services) consume the attestation for access control.

## Benefits
- Protects user privacy by hiding sensitive personal data.  
- Reduces risk of centralized data leaks.  
- Easy to integrate into apps requiring age checks (social media, online games, KYC-lite platforms).

## Development Timeline
- **Week 1**: Design ZK circuit & prepare age threshold logic.  
- **Week 2**: Implement prototype using Soundness Layer SDK.  
- **Week 3**: Integrate with demo app (web form for age check).  
- **Week 4**: Testing & documentation.  

## Future Work
- Extend to different thresholds (21+, 25+).  
- Combine with decentralized identity frameworks.  
- Mobile-first version for easy onboarding.  

---

*Submitted by Fazrizuliman07 for the Soundness Dev Program*
