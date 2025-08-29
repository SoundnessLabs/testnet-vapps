# vApp Submission: [KeyForge]

## Verification
```yaml
github_username: "Tomastaro"
discord_id: "horsex_"
timestamp: "2025-08-29"
```

## Developer
- **Name**: Toma
- **GitHub**: @Tomastaro
- **Discord**: horsex_
- **Experience**: Experimented with node deployment and infra tooling for L2 projects. Familiar with user-side pain points (keys, wallets, access). Looking to turn these into dev tools.

## Project

### Name & Category
- **Project**: KeyForge
- **Category**: infrastructure

### Description
KeyForge is a unified access layer for dApps.
Right now, every app forces users to manage keys, signatures, and logins separately. This creates friction and drives away non-crypto users.

KeyForge acts as a credential router: developers plug in once, and their app can verify wallet, social login, and reputation credentials via Soundness Layer. Users no longer juggle multiple accounts — they just use one verifiable profile.

### SL Integration  
Use SL to issue and validate unified credentials (wallet + reputation).
SL handles proofs of ownership, so dApps don’t need to reinvent login flows.
Apps can query KeyForge API for lightweight “is this user trusted/verified?” checks.

## Technical

### Architecture
User connects wallet or social → KeyForge issues SL credential.
dApp integrates KeyForge SDK → queries user’s credential status.
Verification happens through SL, with KeyForge acting as the middleware.

### Stack
Frontend: Minimal SDK integration + docs site
Backend: Rust service to handle requests
Blockchain: SL + Ethereum (for settlement)
Storage: WALRUS for logs, IPFS for credential metadata

### Features
One-click credential issuance for users
Simple SDK for devs (login + verify in one line)
Portable profile across any integrated dApp

## Timeline

### PoC (2-4 weeks)
Basic wallet login → SL credential issued
Simple SDK demo with test dApp
Docs for integration

### MVP (4-8 weeks)  
Multi-login support (wallet + email/social)
Reputation flag (trusted vs new user)
First 2–3 dApps onboarded

## Innovation
Removes friction from onboarding new users into crypto.
Saves devs from rebuilding login + credential logic.
Creates a shared “trust layer” for the ecosystem.

## Contact
Github:@Tomastaro

**Checklist before submitting:**
- [ ] All fields completed
- [ ] GitHub username matches PR author  
- [ ] SL integration explained
- [ ] Timeline is realistic
