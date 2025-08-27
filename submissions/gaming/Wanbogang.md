# NFT Auto-Minter Bot (vApp Proposal)

**Category:** gaming  
**GitHub:** https://github.com/Wanbogang  
**Discord:** gusjenggot (751085079505403974)

## TL;DR
A small, fast bot that mints free NFTs the moment a mint contract goes live. Targets Ethereum, Base, Arbitrum, Polygon. Uses Soundness Layer checks to avoid bad tx and improve reliability.

## Problem
Free mints are gone in seconds. Manual clicks lose to automation. We need an automated flow with guarded transaction sending.

## What I’m building
- **Listener**: watch deploy/mint windows on the 4 chains.
- **TX executor**: sane gas strategy (priority fee bump, bounded replacements).
- **Guards (Soundness)**: pre-send check so tx only goes out if proofs/conditions are sound.
- **Dry-run + logs**: safe testing and measurable outcomes.

## Technical architecture
- **Runtime**: Node.js 20; Hardhat for contract interaction.
- **Chains**: eth / base / arb / polygon (RPC via `.env`).
- **Gas logic**: priority fee baseline, incremental bump, hard cap on replacements.
- **Soundness integration**: call Soundness CLI/SDK before send (gate tx), and record proof id for audit.
- **Config**: `.env` driven; supports `DRY_RUN=true`.

### Minimal config (example)
```env
ETH_RPC_URL=...
BASE_RPC_URL=...
ARB_RPC_URL=...
POLYGON_RPC_URL=...
MAX_PRIORITY_FEE_GWEI=2.0
MAX_FEE_MULTIPLIER=2.8
REPLACEMENT_AFTER_SECONDS=10
REPLACEMENT_PRIORITY_BUMP_GWEI=1.0
MAX_REPLACEMENTS=5
DRY_RUN=true
