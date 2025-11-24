# vApp Submission: SmartPay Protocol

## Verification
```yaml
github_username: "Salmanph9"
discord_id: "1193145957605462109"
timestamp: "2025-11-25"
```

## Developer

  - **Name**: Salman Paris Harahap
  - **GitHub**: @Salmanph9
  - **Discord**: s.p.h.
  - **Experience**: Experience in community management and building is highly valued.
## Project

### Name & Category

  - **Project**: SmartPay Protocol
  - **Category**: infrastructure

### Description

A decentralized automation tool for managing recurring crypto payments (subscriptions, DAO dues, and DCA). Allows users to set allowances for specific intervals without locking funds in a custodial contract.

### SL Integration

Leverages SL Executable Tokens to define payment schedules and logic. The SL token acts as the user's "subscription card," handling validity checks and authorization logic off-chain before settling on-chain.

## Technical

### Architecture

Smart Contracts handle the settlement and allowance registry. SL tokens carry the logic for billing cycles and user notifications, ensuring payments only trigger when conditions are met.

### Stack


Frontend: Next.js + Tailwind CSS

Backend: NestJS (for indexing)

Blockchain: SL + Base + Arbitrum

Smart Contracts: Solidity (Foundry)


### Features

Gas-efficient recurring payments

One-click subscription management

Cross-chain balance checking

## Timeline

### PoC (2-4 weeks)

  - [ ] Core allowance contracts
  - [ ] Basic dashboard UI
  - [ ] SL Token integration

### MVP (4-8 weeks)

  - [ ] Multi-chain support
  - [ ] Notification service (Email/Telegram)
  - [ ] Merchant SDK

## Innovation

Replaces custodial subscription models with self-sovereign logic using Smart Layer, reducing risk for both merchants and users.

## Contact

d preferred. Updates posted in community channels.

**Checklist before submitting:**

  - [x] All fields completed
  - [x] GitHub username matches PR author
  - [x] SL integration explained
  - [x] Timeline is realistic

<!-- end list -->
