# vApp Submission: [EchoPass]

## Verification
```yaml
github_username: "Duckmario"
discord_id: "duckm0131"
timestamp: "2025-08-29"
```

## Developer
- **Name**: Duck
- **GitHub**: @Duckmario
- **Discord**: duckm0131
- **Experience**: Active participant in Web3 communities, with background in user research and community growth. Exploring how verifiable credentials can improve trust and reduce spam in online spaces.

## Project

### Name & Category
- **Project**: EchoPass
- **Category**: identity

### Description
Today’s online communities suffer from spam, fake accounts, and low-quality engagement. EchoPass is a portable reputation passport: a verifiable profile that carries a user’s community contributions across platforms.

Instead of proving identity with raw addresses, EchoPass lets users showcase proof of participation (events joined, tasks completed, predictions made) that’s validated via Soundness Layer. Communities can use this to filter members, reward engagement, and build stronger trust.

### SL Integration  
SL used to issue proof-of-engagement credentials (attending an AMA, submitting feedback, completing quests).
Each credential is cryptographically verifiable and tied to a reputation score.
Communities query SL to decide: “Should this user get access, perks, or moderation leniency?”

## Technical

### Architecture
User completes community action (e.g. join call, post verified feedback).
EchoPass issues SL credential for that action.
User stores credentials in wallet; dApps/DAOs can query to verify reputation.

### Stack
Frontend: React dashboard for viewing passport
Backend: Node.js credential issuer
Blockchain: SL as primary credential layer
Storage: IPFS for metadata, WALRUS for logs

### Features
Verifiable community participation credentials
Reputation passport viewable by user or sharable with dApps
Simple verification API for communities to gate access

## Timeline

### PoC (2-4 weeks)
Basic credential issuance for one community action
Simple user dashboard to view EchoPass
Test query API for one sample DAO

### MVP (4-8 weeks)  
Multi-action credential support (feedback, events, tasks)
Reputation scoring system
Pilot with 2–3 partner communities

## Innovation
EchoPass transforms fragmented community engagement into portable social capital. Instead of Discord roles or Telegram bots that reset every time, users carry their participation record across platforms. Communities instantly see “who contributes” vs “who spams.”

## Contact
Github:@Duckmario

**Checklist before submitting:**
- [ ] All fields completed
- [ ] GitHub username matches PR author  
- [ ] SL integration explained
- [ ] Timeline is realistic
