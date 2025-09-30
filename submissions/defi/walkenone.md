# Soundness Layer vApp Proposal

## Developer

- **Name:** Walken  
- **GitHub:** @walkenone  
- **Discord:** walken#1234 (ID: 938485146322092134)  
- **Experience:**  
  Smart contract developer and DeFi builder with experience on Sui and EVM chains. Focused on privacy-first finance and protocol-level innovation. Previous contributor to Sui staking .

- **Submission Timestamp:** 2025-08-22

---

## Project

### Name & Category

- **Project:** SundLayer  
- **Category:** defi

---

### Description

**SundLayer** is a privacy-first DeFi yield and lending protocol built on the **Sui Network**, integrating **Soundness Layer (SL)** to protect user data and prevent Sybil farming.

It enables users to:
- Stake SUI for **liquid staking tokens (slSUI)**  
- Trade slSUI privately  
- Lend or borrow assets without revealing wallet balances or positions  
- Participate in fair, Sybil-resistant airdrop programs

**Key Problems Solved:**
- Lack of privacy in DeFi staking and trading
- MEV and frontrunning on public DEXs
- Sybil attacks on points/airdrop systems

---

## SL Integration

SundLayer uses Soundness Layer to enhance privacy and user uniqueness in the following ways:

- **ZK Staking Proofs:** Users can stake and prove stake balance without exposing amounts  
- **ZK Lending Access:** Borrow eligibility checked using private, SL-verified ZK proofs  
- **ZK Trade Execution:** Private token swaps via SL's encrypted execution module  
- **WALRUS Commitments:** Store user interaction proofs and verify without storing sensitive data  
- **ZK Identity Proofs:** Prevent Sybil attacks in reward systems via zero-knowledge uniqueness checks  
- **Proof Aggregation:** Batch user interactions for low-gas verification on Sui

---

## Technical

### Architecture
+-------------------------------------------------------------+
|                         User Interface                      |
|  (React App, Sui WalletKit, ZK-enabled forms, Wagmi for Sui)|
+-------------------------------------------------------------+
                           |
                           v
+-------------------------------------------------------------+
|                  Soundness Layer SDK (Client)               |
|   - Proof generation (ZK circuits)                          |
|   - Identity module (uniqueness proofs)                     |
|   - Privacy transaction composer                            |
+-------------------------------------------------------------+
                           |
                           v
+-------------------+                      +----------------------+
| Sui Smart Contracts| <==Txs & Proofs==>  |  WALRUS Commitments  |
|  - slSUI staking   |                     |  - Proof storage      |
|  - lending module  |                     |  - Commitment hashes  |
|  - private trade   |                     +----------------------+
|  - rewards manager |
+-------------------+


---

## Components

### 🖥 Frontend (React + TypeScript)
- Connects to Sui wallets (WalletKit / Suiet)
- Uses SL SDK to:
  - Generate ZK proofs for staking, lending, trading
  - Prove identity uniqueness privately
  - Encrypt transactions for private on-chain execution

### 🧠 Soundness Layer SDK
- **ZK Circuits** for:
  - Stake proof
  - Collateral proof
  - Private swap verification
  - Identity uniqueness
- **Proof Aggregation** to reduce gas/compute load
- **Privacy Middleware** to support encrypted DeepBook trades

### 📦 WALRUS Commitments
- Stores off-chain commitments of user interactions
- Verifiable on-chain without exposing data
- Used for future airdrop proof and activity audits

### 🧾 Sui Smart Contracts
- **Staking Contract**: Accepts SUI and mints slSUI
- **Lending Contract**: Uses ZK proofs for borrow eligibility
- **Trade Module**: Executes SL-routed swaps on DeepBook
- **Rewards Module**: Tracks points via SL-verified actions

---

## Stack Summary

| Layer          | Tech Stack                                                       |
|----------------|------------------------------------------------------------------|
| **Frontend**   | React, TypeScript, Tailwind, Sui WalletKit                       |
| **Backend**    | Node.js + Express (SL relayer + proof coordinator)               |
| **Blockchain** | Sui (staking, lending, rewards)                                  |
| **ZK Layer**   | Soundness Layer SDK, identity proof engine, ZK proof generators  |
| **Storage**    | WALRUS (off-chain commitments), IPFS (optional metadata)         |
| **DevOps**     | GitHub Actions (CI/CD), Docker for testnet deployment            |

---

## Proof Flow Example: Private Lending

1. User connects wallet → requests a loan using slSUI as collateral  
2. SL SDK generates ZK proof showing eligible stake or LP status  
3. Transaction composed with encrypted calldata and proof  
4. Submitted to Sui → contract verifies via SL  
5. If valid, loan is issued without revealing user’s stake/identity  
6. Commitment stored via WALRUS for auditability

---

## Features

- 🔒 **Private Liquid Staking** (stake SUI → slSUI with privacy)
- 🧾 **ZK Lending/Borrowing** (prove collateral privately)
- 🔁 **SL Private Swaps** (via DeepBook + encrypted calldata)
- 🏆 **Reward Program** (with Sybil-proof identity checks)
- 🧬 **Social & Identity Multipliers** (Discord, Twitter, SL ID)

---

## Social & Identity Boosters

Earn permanent multipliers by verifying identity and connecting socials via [Guild.xyz](https://guild.xyz):

| Action                      | Description                                | Multiplier Boost       |
|----------------------------|--------------------------------------------|-------------------------|
| Connect Discord            | Join + verify in Discord server            | +10%                   |
| Connect Twitter/X          | Follow @sundlayer_xyz                      | +10%                   |
| Connect Farcaster          | Follow @sundlayer                          | +10%                   |
| SL ZK Identity Proof       | Generate Sybil-resistant proof of uniqueness| +20%                   |

---

## How to Earn Points

Points accrue automatically by interacting with SundLayer. Multipliers apply for social and identity boosts.

| Action                         | Description                                                                   | Points                                |
|-------------------------------|-------------------------------------------------------------------------------|----------------------------------------|
| **Stake SUI on SundLayer**     | Stake SUI to earn liquid staking tokens (slSUI)                               | `+10 pts / $ per day × Multiplier`    |
| **Provide LP on DeepBook DEX** | LP slSUI/USDC or slSUI/SUI on DeepBook (Sui’s native DEX)                     | `+20 pts / $ per day × Multiplier`    |
| **Use ZK Lending/Borrowing**   | Lend or borrow SUI, USDC, or slSUI privately via SL ZK-powered lending market | `+30 pts / $ per day × Multiplier`    |
| **Refer Users**                | Refer users who stake, lend, or LP. Earn a % of their points.                 | `10% of referred user’s daily points` |
| **Hold slSUI in Wallet**       | Passive earnings for holding slSUI                                            | `+5 pts / $ per day`                  |
| **Use SL ZK Trade Module**     | Execute private swaps on DeepBook via SL privacy layer                        | `+25 pts per trade`                   |

---

## 🔥 Levels & Rewards

| Level | Points Needed | Reward                                      |
|-------|----------------|--------------------------------------------|
| 1     | 500            | Role: “Sund Seeker”                        |
| 2     | 5,000          | Role + Early Beta Access                   |
| 3     | 50,000         | NFT Airdrop + Role: “SL Pioneer”           |
| 4     | 250,000        | Limited Merch Drop + Role: “Sui Veteran”   |
| 5     | 1,000,000      | Protocol Governance Access                 |

---

## Timeline

### PoC (2–4 Weeks)
- Deploy slSUI staking contract to Sui testnet  
- Integrate SL ZK proofs for private staking and swaps  
- Build frontend (wallet connect, stake UI, points display)

### MVP (4–8 Weeks)
- Launch lending/borrowing module with ZK collateral proofs  
- Add SL-based referral system and identity rewards  
- Enable SL-powered DeepBook swaps  
- Launch public testnet + Sybil-resistant points campaign

---

## Innovation

SundLayer brings **Layer-2-grade privacy** to a **Layer-1 chain (Sui)** by deeply integrating Soundness Layer’s ZK toolkit.

- ✅ First private staking and lending experience on Sui  
- ✅ Sybil-proof points & rewards system via SL identity  
- ✅ MEV-resistant swaps using SL private execution  
- ✅ Real ZK-based user uniqueness, no KYC required  
- ✅ Gas-efficient batch ZK proof verification  

This unlocks a **new class of DeFi user**: privacy-conscious, yield-seeking, and fair-play-oriented. 

---

## links
- github repo: https://github.com/walkenone/testnet-vapps
- discord id : 938485146322092134
- x          : https://x.com/OneWalken30383

