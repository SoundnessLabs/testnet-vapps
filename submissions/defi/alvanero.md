# vApp Submission: [SoundTrade]

## Verification
```yaml
github_username: "alvanero"
discord_id: "878534291695501372"
timestamp: "2025-08-27"
```

## Developer
- **Name**: Alvanero
- **GitHub**: @alvanero
- **Discord**: alvanero
- **Experience**: Brief background

# Auto-Trade Bot on Soundness Layer — DeFi

## Summary
This project introduces an automated trading bot that leverages Soundness Layer’s proof system to verify and attest trades in a secure and transparent way. The bot interacts with decentralized exchanges (DEXs) across testnets, submits zero-knowledge proofs of executed trades, and publishes attestations through Soundness CLI.

## Problem & Solution
- **Problem:** Trading bots in DeFi often operate as black boxes. Users cannot easily verify whether trades were executed as promised or if strategies are manipulated.
- **Solution:** By integrating Soundness Layer, every trade execution generates a proof. This proof is uploaded and verified, ensuring that each action of the bot is transparent, auditable, and tamper-proof.

## Technical Architecture
- **Trading Engine:** Automated strategy runner (simple arbitrage and grid trading on testnet DEXs).
- **Proof Generation:** Each executed trade generates a proof of:
  - trade execution parameters (asset pair, price, amount)
  - transaction hash on-chain
- **Storage:** Proof is uploaded to Walrus, returning a `blob_id`.
- **Attestation:** The bot submits `blob_id` via Soundness CLI to Soundness Layer, which issues a verified attestation.
- **Verification:** Users or external dApps can query the attestation to validate the bot’s trades.

### Flow
1. Bot executes trade → record tx hash.  
2. Bot generates proof of execution.  
3. Proof uploaded to Walrus → returns `blob_id`.  
4. CLI command submits `blob_id` → Soundness Layer issues attestation.  
5. Attestation published to dashboard/Discord.

## Integration with Soundness Layer
- Commands used:  
  - `soundness proof submit --blob-id <id>`  
  - `soundness attest --proof <proof>`  
- Attestations link directly to executed trades, creating a **verifiable trading history** for the bot.

## Milestones & Timeline
- **M1 (Week 1–2):** Implement basic auto-trade bot with mock trades on testnet.  
- **M2 (Week 3):** Integrate proof generation + Walrus storage.  
- **M3 (Week 4):** Submit proofs via CLI, receive attestations.  
- **M4 (Week 5):** Build simple dashboard/Discord bot to show verified trades.  

## Team
- **GitHub:** [alvanero](https://github.com/alvanero)  
- **Discord ID:** 878534291695501372  
- Solo developer focusing on DeFi automation, bots, and Web3 integrations.

## Repos/References
- Codebase: (to be added after PR approval)
- Soundness CLI documentation
- Soundness Layer blog & tutorials
