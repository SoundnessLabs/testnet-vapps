# TideLock — Threshold zk-Release Drop

**Category:** infrastructure  
**GitHub:** edy-wijaya  
**Discord:** maxibillion_

## Summary
TideLock encrypts content and stores it on Walrus, and only releases the decryption key once a **threshold K** of valid eligibility proofs has been reached. Proofs are verified via the Soundness Layer using zero-knowledge (membership + nullifier), while Sui stores the pointer, status, and event feed. The result is a collective, privacy-preserving, verifiable content release.

## Why Soundness + Walrus + Sui
- **Walrus**: low-cost blob storage and data availability.
- **Sui**: coordination of state, threshold K, and a queryable event feed.
- **Soundness Layer**: fast, economical, reproducible verification of ZK proofs.

## Architecture
1) Creator encrypts a file → uploads ciphertext to Walrus → obtains `blob_id` and `content_hash`.
2) Creator registers a “drop” on Sui (stores threshold **K**, Walrus pointer, and `content_hash`).
3) Participants submit **pledges**:
   - PoC (Week 1–2): basic signature → `PledgeReceived`.
   - Final (Week 3+): **ZK proof** (Merkle membership / token-based eligibility) + **nullifier** (prevents double-pledge) + **content binding** (to `content_hash`).
4) When the valid pledge count reaches **K** → `ThresholdMet`. The UI releases or reconstructs the key (k-of-n) and users decrypt content from Walrus.

## Milestones & Timeline
- **Week 1–2:** PoC — CLI encrypt+upload to Walrus; Sui contract with `register_drop`, `pledge`, `ThresholdMet`; basic event feed.
- **Week 3:** ZK circuits (membership + nullifier + binding) and verifier integration with Soundness Layer.
- **Week 4:** UI/UX for download & decrypt; private link (share-code) or Shamir k-of-n; end-to-end testing.
- **Week 5:** Public demo, documentation, and iteration plan.

## Technical Notes
- Move module `tidelock`: shared `Drop` object (`threshold`, `count`, `content_hash`, `blob_id`, `released`), events `DropRegistered`, `PledgeReceived`, `ThresholdMet`.
- ZK: membership tree (allowlist/role/NFT holder), nullifier (epoch-based), binding to `content_hash`.
- Frontend: Next.js; CLI tooling for AES-GCM encryption and SHA-256 hashing; Walrus upload API.
- Key release: (A) server-side release at `ThresholdMet`, or (B) Shamir k-of-n with client-side reconstruction.

## Team
Solo builder: edy-wijaya.

## Testnet Needs
- Soundness verifier endpoint + sample payload format.
- Circuit design review (nullifier & binding best practices).

## Links
- X/Discord: will be added after the PR is opened.
