# vApp Submission: AnonSurvey — Eligibility-Gated Anonymous Survey

## Verification
```yaml
github_username: "dicky-setiadi"
discord_id: "462058243057909760"
timestamp: "2025-08-28"
```

- Name: Dicky Setiadi
- GitHub: @dicky-setiadi
- Discord: @_dckys
- Experience: Junior Web Developer (JavaScript/Node.js)

## Project

### Name & Category
- Project: AnonSurvey (Eligibility-Gated Anonymous Survey)
- Category: other

### Description
Short surveys where only eligible users (verified via Soundness) can submit exactly once, while responses remain anonymous. Useful for community votes or feedback without Sybil or doxxing.

### SL Integration
The server checks each user’s Soundness proof on testnet against a simple policy (e.g., require human_verified). When the proof is valid and the user hasn’t responded before, the answer is accepted and a receipt is returned. PoC is read/verify only (no on-chain writes).

## Technical

### Architecture
1. Users open a survey page and connect a wallet.
2. The app asks the server for a one-time challenge, the user signs it, and the app sends the signed challenge together with a Soundness proof.
3. The server verifies the wallet signature and then asks Soundness whether the proof satisfies the configured policy.
4. If the user is eligible and hasn’t answered yet, the server records the choice and issues a short-lived access token or receipt so the client can confirm submission.
5. A public results page displays only aggregated counts; no personal data is shown.

### Stack
- **Frontend**: Discord bot (Node.js/JavaScript) + a verify page used only for wallet connect and message signing (React).
- **Backend**: Node.js + Express
- **Blockchain**: Soundness Layer
- **Storage**: Database (PostgreSQL) + WALRUS for public snapshots

### Features
1. Eligibility-gated voting using Soundness proofs (policy-based, e.g., human_verified).
2. Anonymous answers: store only a salted address_hash, not the raw address.
3. One-vote-per-address enforcement with a unique constraint.
4. Discord-first UX with a bot orchestrating verify & vote.
5. Public aggregated results; optional WALRUS/IPFS snapshots for transparency.
6. Minimal admin flow to create/close a survey.

## Timeline

### PoC (2-4 weeks)
- [ ] Discord bot skeleton and minimal verify page wired up.
- [ ] Wallet connect, one-time challenge, signing flow.
- [ ] Server-side eligibility check (Soundness testnet) with a simple policy.
- [ ] One-vote-per-address (salted hash) and basic aggregated results page.

### MVP (4-8 weeks)
- [ ] Better error states and UX polish in bot & verify page.
- [ ] Optional Redis rate limiting.
- [ ] Simple admin utilities (open/close survey, view logs).

## Innovation
1. Privacy-preserving + Sybil-resistant: enforces one vote per address using a salted hash, avoiding storage of raw wallet addresses or personal information.
2. Configurable policy: eligibility rules (e.g., human_verified) are swap-in via ENV/JSON—no code changes needed.
3. Fast, zero-gas PoC: read/verify only on Soundness testnet, enabling quick iteration and no on-chain costs.

## Contact
- **Discord**: @_dckys
- **GitHub**: @dicky-setiadi
- **Email**: dickysetiadi50@gmail.com
