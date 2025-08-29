vApp Submission: Pixel Jumper
Verification
github_username: "aamon3572"
discord_id: "1032045488242966718"
timestamp: "2025-08-29"

Developer
Name: Aamona
GitHub: @aamon3572
Discord: aamon909#0
Experience: Frontend web developer with experience in building JavaScript-based applications and browser mini-games. Familiar with blockchain integration, especially related to NFTs and verifiable data.

Project
Name & Category
Project: Pixel Jumper
Category: gaming

Description
Pixel Jumper is a simple browser-based endless runner game where players must jump to avoid obstacles.  
Problem solved: scores in casual games are often easy to manipulate (for example by modifying the client code). Pixel Jumper addresses this issue by using the Soundness Layer to verify score authenticity, ensuring that players can only claim achievements if their results are valid.

SL Integration
- The game records transcripts: player inputs, RNG seed, and physics state changes.  
- The transcript is used to generate a score proof.  
- The proof is submitted to the Soundness Layer for verification.  
- Once verified, the system triggers a contract on Sui to mint an achievement NFT.  

Technical
Architecture
- **Client:** HTML5 Canvas-based game that generates gameplay transcripts.  
- **Relayer (optional):** Helps submit transcripts + proofs to the chain.  
- **Soundness Layer:** Verifies the validity of score proofs.  
- **On-chain App (Sui):** Provides NFT rewards for valid scores.  

Stack
Frontend: Vanilla JS (HTML5, CSS, Canvas)  
Backend: Node.js (optional for relayer)  
Blockchain: Soundness Layer + Sui  
Storage: WALRUS/IPFS for replay transcript storage  

Features
- Lightweight casual gameplay (easy to play).  
- Scores verified with SL proofs.  
- NFT rewards based on valid scores.  

Timeline
PoC (2-4 weeks)
- Basic game (done).  
- Transcript generator.  
- Mock SL integration.  
- Simple UI for score submission.  

MVP (4-8 weeks)
- Complete proof circuit for physics and obstacle spawning.  
- Full SL verification.  
- NFT reward system on Sui.  
- Public leaderboard with verified scores.  

Innovation
Pixel Jumper is not just a casual game but a demonstration of how the Soundness Layer can make game scores *provably fair*. This opens the door for blockchain-based tournaments that are transparent and tamper-proof. The combination of a simple game + SL/NFT integration provides a unique experience that cannot be easily faked.  

Contact
Preferred: Discord DM (aamon909#0)  
Updates: GitHub project repository + community Discord  

Checklist before submitting:
- [x] All fields completed  
- [x] GitHub username matches PR author  
- [x] SL integration explained  
- [x] Timeline is realistic  
