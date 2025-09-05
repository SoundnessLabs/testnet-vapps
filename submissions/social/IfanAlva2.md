vApp Submission: Community Trust Board

## Verification
- github_username: "IfanAlva2"
- discord_id: "780608657356357652"
- timestamp: "2025-08-29"

## Developer
- Name: IfanAlva
- GitHub: @IfanAlva2
- Discord: alzion_7
- Experience: Beginner in web development, experimenting with simple projects and learning by building.

## Project
### Name & Category
- Project: Community Trust Board  
- Category: social

### Description
This vApp provides a **community-driven trust board** for X posts.  
Users can submit a post URL once, and the entry becomes a shared board where the community can:  
- Vote whether the post is *Legit / Scam / Unsure*  
- Add comments or suggestions with Soundness Layer proofs  

This helps participants quickly verify project claims or community announcements without manually checking one by one.

---

## SL Integration
- Each entry (post) will be assigned a **Soundness proofId**  
- Votes and comments will also be submitted with proofIds, ensuring verifiable and tamper-proof participation  
- Soundness Layer ensures entries and community interactions are cryptographically verified

---

## Technical
### Architecture
- Frontend client where users submit a post URL  
- System checks if entry already exists (1 post = 1 entry)  
- Users vote or comment, with each interaction tied to a Soundness proof  
- All entries rendered into a transparent community trust board

### Stack
- Frontend: HTML/Vanilla JS (PoC), React (MVP upgrade)  
- Backend: None (PoC), optional Node.js/Firebase for multi-user sync in MVP  
- Blockchain: Soundness Layer (for proofs)  
- Storage: localStorage (PoC), IPFS/WALRUS or DB (MVP)

### Features
- One entry per post (no duplicates)  
- Voting system (Legit / Scam / Unsure)  
- Comment & suggestion board per entry  
- Proof integration for entries, votes, and comments

---

## Timeline
### PoC (2-4 weeks)
- [x] Basic functionality
- [x] SL integration
- [x] Simple UI

### MVP (4-8 weeks)  
- [ ] Full features
- [ ] Production ready
- [ ] User testing
      
---

## Innovation
Unlike normal polls, this tool focuses on **community consensus with verifiable proofs**.  
It prevents duplicate entries and consolidates all votes/comments into a single trusted board per post, making scam detection and project validation easier for communities.  

---

## Contact
- Preferred contact: Discord DM & Gmail arknow6@gmail.com

---

## Checklist before submitting:
- [x] All fields completed  
- [x] GitHub username matches PR author  
- [x] SL integration explained  
- [x] Timeline is realistic
