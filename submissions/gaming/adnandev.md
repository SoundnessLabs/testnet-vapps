vApp Submission: Chain Memory Game

Verification
github_username: "adnas17"
discord_id: "1006586581722877963"
timestamp: "2025-08-29"

Developer
Name: AdnanDev
GitHub: @adnas17
Discord: adnas1717#0
Experience: 3 years as a frontend developer, with experience building browser-based games in React & simple Web3/NFT integrations.

Project
Name & Category
Project: Chain Memory Game
Category: gaming

Description
Chain Memory Game is a simple web-based memory card game (flip & match pairs).  
Problem solved: in traditional online games, scores are stored on centralized servers and can be manipulated. With Soundness Layer (SL), player progress and scores are verified and recorded transparently on-chain.

SL Integration
- Each matched pair is recorded through SL with integrity proof.  
- Final scores are verified via SL and cannot be tampered with.  
- A global leaderboard leverages SL logs for transparency.  

Technical
Architecture
- Frontend React displays the memory card board.  
- Backend Node.js manages game sessions & communicates with SL.  
- SL stores scores & progress as verifiable logs.  
- Optional database caching for fast leaderboard queries.  

Stack
Frontend: React + TailwindCSS  
Backend: Node.js (Express)  
Blockchain: Soundness Layer  
Storage: SL logs + SQLite/Postgres (optional)  

Features
- Simple memory card gameplay (flip & match)  
- Score verification via SL  
- Transparent global leaderboard  

Timeline
PoC (2-4 weeks)
- Basic memory game UI  
- SL integration for score validation  
- Simple leaderboard  

MVP (4-8 weeks)
- Full features: card categories, difficulty levels  
- Polished leaderboard  
- Production-ready with user testing  

Innovation
While memory games are common, the integration with SL makes this unique: scores are 100% transparent, tamper-proof, and publicly auditable.  

Contact
Preferred: Discord : adnas1717#0  
Updates: GitHub repo + Discord community

Checklist before submitting:
- [x] All fields completed  
- [x] GitHub username matches PR author  
- [x] SL integration explained  
- [x] Timeline is realistic  
