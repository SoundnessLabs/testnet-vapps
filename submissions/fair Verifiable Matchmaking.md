# vApp Submission: [fair Verifiable Matchmaking]

## Verification
```yaml
github_username: "Ankitsahami"
discord_id: "1202231551270592553"
timestamp: "Thursday, August 21, 2025, at 6:37:29 PM IST"
```
## Developer
- **Name**: ankit sahani
- **GitHub**: @ankit09439
- **Discord**: username#1234
- **Experience**: i have build many dapps in eth , hemi , in monad and i am also helping pody network in there product. 
i many focused in game like dapps because thats what i have been making since the start.
## Project
### Name & Category
- **Project**: FairMatch
- **Category**: identity/gaming
### Description
Traditional online matchmaking systems are centralized and opaque, leading to unfair games where smurfs, hackers, or unbalanced teams ruin the experience. Players have no way to verify if matchmaking was fair or whether their opponents actually belong in the same skill bracket.
FairMatch solves this by using zero-knowledge (zk) proofs on the Soundness Layer (SL) to enforce trustless matchmaking. Each player provides a zk-proof of their rank and stats without revealing sensitive account details. The matchmaking engine then assembles teams only from verified players of similar skill.
The result is provably fair matchmaking:
No smurfs (fake low-rank accounts).
No inflated or manipulated MMR.
Players can independently verify that the game lobby was assembled fairly.
### SL Integration  
FairMatch leverages the Soundness Layer (SL) for:
zk-Credentials → Players generate zero-knowledge proofs of rank, match history, or skill metrics, verified against trusted issuers (game servers or eSports organizers).
zk-Aggregation → Multiple proofs are combined to show a valid lobby (e.g., “10 verified players, all in Gold rank bracket”) without revealing individual stats.
zk-Signatures → To ensure lobbies cannot be tampered with after formation — the proof of fair matchmaking is signed and verifiable.
WALRUS Storage → Stores verifiable lobby proofs, so disputes can be checked after a game.
## Technical
### Architecture
FairMatch follows a decentralized client–server model with zk-verification baked into the matchmaking process:
Player Client
A lightweight game client plugin or web app where players connect their wallet and request matchmaking.
Generates zk-proofs of rank/skill from game credentials without revealing private account data.
Matchmaking Service
Receives proofs from multiple players.
Uses SL zk-verification to confirm proofs are valid.
Uses zk-aggregation to build a verifiable lobby (e.g., all players are within ±200 MMR).
Publishes a signed proof of fairness to storage.
Game Server / Organizer
Fetches the lobby proof from storage.
Runs the actual match with only verified players.
Optionally attaches post-match results as zk-proofs to update player credentials.
Verification Layer (Soundness)
Ensures all proofs (rank, lobby, outcomes) are verifiable on-chain.
Provides zk-primitives for uniqueness, aggregation, and credential issuance.
Storage
Lobby proofs and match records stored in WALRUS / IPFS, accessible for audits or dispute resolution.
### Stack
- Frontend:
React + Tailwind (web client for matchmaking UI)
Wallet integration (e.g., Wagmi / RainbowKit for Ethereum)
Backend:
Node.js (Express) for matchmaking coordination
Rust modules for zk-proof generation/verification (performance critical parts)
Blockchain:
Soundness Layer (SL) as the core zk-verification layer
Ethereum (or a testnet like Sepolia) for optional game tokens/rewards
Storage:
WALRUS → for verifiable lobby & match proofs
IPFS → for metadata (game configs, tournament details)
(Optional) Postgres → for caching and user session data
### Features
zk-Verified Matchmaking – Players submit zero-knowledge proofs of their rank/skill level; the system only forms lobbies from verified, fair matches.
Lobby Proofs – Each game lobby comes with a signed zk-proof that the matchmaking was unbiased and all players met the skill criteria.
Dispute Resolution – Players can verify lobby proofs on-chain or via the FairMatch explorer, ensuring no manipulation by servers or organizers.
## Timeline
PoC (2–4 weeks)
Implement basic matchmaking flow with wallet connect.
Integrate Soundness Layer for zk-proof verification of player rank.
Generate and store simple lobby proofs in WALRUS/IPFS.
Build a minimal React frontend for queueing and viewing proofs.
MVP (4–8 weeks)
Full matchmaking with zk-aggregation (balanced teams, MMR ranges).
Signed lobby proofs with on-chain verifiability.
Dispute resolution module (players can check proofs post-match).
UI improvements: match history, lobby explorer, proof viewer.
## Innovation
FairMatch introduces trustless matchmaking — something no existing gaming platform provides today.
Traditional matchmaking systems are opaque and controlled by centralized servers, leaving players vulnerable to smurfs, cheaters, or
manipulated MMR.
## Contact
uhh i can update and give my work in x and discord.
**Checklist before submitting:**
- [✅] All fields completed
- [✅ ] GitHub username matches PR author  
- [✅ ] SL integration explained
- [✅ ] Timeline is realistic
