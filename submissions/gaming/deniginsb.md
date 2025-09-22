# vApp Submission: zk-Trivia Game

## Verification
```yaml
github_username: "deniginsb"
discord_id: "622368860787572746"
timestamp: "2025-09-22"
```

## Developer
 - Name: Deni Ardiansyah
 - GitHub: @deniginsb
 - Discord: deniajaa10
 - Experience: 2+ years experience in Web3 and coding. Built blockchain games, wallet analytics, and AI-powered apps with focus on zk and on-chain interactivity.

## Project

### Name & Category
 - Project: zk-Trivia Game
 - Category: gaming

### Description
A quiz/trivia game where players answer random questions under time pressure. Each answer is verified with zk-proofs via Soundness Layer, ensuring scores cannot be faked. Leaderboards show only attested results, making the game fair and trustless.

### SL Integration

Use Soundness CLI to generate proof for each correct answer. Every correct response creates an attestation stored in WALRUS. Leaderboard queries WALRUS + Soundness to verify validity before showing results. Prevents score manipulation and enables transparent competitions.

## Technical

### Architecture
Frontend Web App: shows trivia questions, timer, and submits answers. Backend API: verifies answers, calls Soundness CLI for proof. Soundness Layer: generates attestation of valid answers. Storage: WALRUS stores proof + attested score. Leaderboard UI: fetches verified scores to display rankings.

### Stack
 - Frontend: React + Tailwind (minimalist dark theme with neon accents)
 - Backend: Node.js + Express
  Blockchain: Soundness Layer (zk proofs + attestations)
 - Storage: WALRUS decentralized storage

### Features

 - Trivia questions with timer (10–20s per question).
 - Score submission with zkProof verification.
 - Attestation badge for valid scores.
 - Global leaderboard (only verified scores count).
 - Optional multiplayer rounds or tournaments.

## Timeline

### PoC (2-4 weeks)
 - [ ] Setup trivia question set + simple frontend
 - [ ] Integrate Soundness CLI for proof per correct answer
 - [ ] WALRUS integration for storing attestations
 + [ ] Basic leaderboard display

### MVP (4-8 weeks)
 - [ ] Full leaderboard with filters & badges
 - [ ] Randomized question sets + time-limited rounds
 - [ ] Polished dark-themed UI with animations
 - [ ] Multi-round tournaments + user testing

## Innovation
Most trivia apps rely on centralized servers that can be manipulated. zk-Trivia Game is unique because every correct answer is proven with zkProofs and attested on Soundness Layer, ensuring fairness and transparency. This demonstrates zk in a simple, fun, and community-friendly context.

## Contact
Preferred updates via GitHub PRs and Soundness Discord. Contact via Discord: deniajaa10 or GitHub: @deniginsb.

Checklist before submitting:
 * [x] All fields completed
 * [x] GitHub username matches PR author
 * [x] SL integration explained
 * [x] Timeline is realistic
