# vApp Submission: [ProofFlip]

## Verification
```yaml
github_username: "Azeezatou"
discord_id: "979677014346854422"
timestamp: "2025-09-01"
```

## Developer
- **Name**: Abdulqadir Azeezat
- **GitHub**: @Azeezatou
- **Discord**: azeezatou
- **Experience**:  A frontend and blockchain developer with over 3 years experience, a Web3 contributor and also a DevRel.

## Project

### Name & Category
- **Project**: ProofFlip
- **Category**: gaming

### Description
What problem does your vApp solve? What does it do?

"ProofFlip" is a simple memory game where a set of face-down cards hide pairs (e.g., 🍎🍎, 🎲🎲, 🦄🦄).
Players flip two cards per turn.
If they match, the player scores.
If not, the cards are hidden again.
Game continues until all pairs are found.
The upside: all flips and card positions are validated with zero-knowledge proofs, so players (and observers) know the game is fair, without exposing the hidden state prematurely.

### SL Integration  
How will you use Soundness Layer? What specific SL features?

In ProofFlip, the Soundness Layer (SL) ensures trust and transparency. 
It starts with a provably fair deck shuffle using a zero-knowledge (ZK) proof, verified by the SL and attested on Walrus. During gameplay, each card flip is validated with a ZK proof to confirm it's a legal move without revealing hidden information. 
The SL then verifies these proofs, updating the game state. This creates a tamper-proof record of the entire game on Walrus, which is used to generate a verifiable, trustless leaderboard, making it impossible to insert fake scores.

## Technical

### Architecture
High-level system design and approach

### Stack
- **Frontend**: React + Tailwind (clean UI, animations for flips).
- **Backend**: Rust (game logic + SP1 zkVM integration).  
- **Blockchain**: Soundness Layer for proof verification.
- **Storage**: Walrus for game state persistence, optional IPFS for assets (card skins, themes).

### Features
1. Core feature 1: Fair Shuffle: ZK-verified shuffle ensures no cheating from server or players.
2. Core feature 2: ZK Flip Validation: Every move is proven valid without leaking hidden info.
3. Core feature 3: Leaderboard with Proofs: Community leaderboard built from verified proofs of wins & scores.

## Timeline

### PoC (2-4 weeks)
- [ ] Basic functionality
- [ ] SL integration
- [ ] Simple UI

### MVP (4-8 weeks)  
- [ ] Full features
- [ ] Production ready
- [ ] User testing

## Innovation
What makes this unique? Why will people use it?
What makes the game unique is its blend of simplicity, trustlessness, and community engagement. It’s fun and simple, a casual game anyone can pick up and enjoy without barriers. It’s also trustless, meaning cheating is impossible since every move is verified with zero-knowledge proofs. The social layer adds competitiveness through leaderboards and challenges with friends, keeping players engaged. Most importantly, it contributes to Soundness by showcasing real zkVM use cases in a playful, accessible way, bridging serious cryptography with casual gaming.

## Contact
Preferred contact method and where you'll share updates.

Telegram: @azee_zatou (preferred) (for team conversations and regular updates.)

Email: abdulkadirazeezat01@gmail.com (for official emails and informations)

X(formerly Twitter): @Azee_zatou (optional)

**Checklist before submitting:**
- [x] All fields completed
- [x] GitHub username matches PR author  
- [x] SL integration explained
- [x] Timeline is realistic

 


