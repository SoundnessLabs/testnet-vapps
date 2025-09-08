# PoC — Auto-Trade Bot (Concept Only)

**Author:** alvanero (Discord: 878534291695501372)  
**PR:** https://github.com/SoundnessLabs/testnet-vapps/pull/199
**Category:** defi  
**Status:** PoC submitted (conceptual, due to missing trade feature in Soundness)

## What I proved
- Able to simulate the workflow of Soundness:
  - Prepared a payload (proof_payload.json)
  - "Uploaded" → generated dummy `blob_id`
  - "Submitted" → generated dummy attestation

## IDs (dummy)
- `blob_id`: 0xmockblobid1234567890abcdef  
- `attestation`: 0xmockattestation9876543210fedcba  

## Commands
See `poc/commands.txt`

## Notes
Currently, Soundness testnet does not expose trade endpoints.  
This PoC demonstrates understanding of the pipeline (payload → blob → attestation) and will be extended once trade is available.


## Concept & Workflow — Auto-Trade Bot on Soundness Layer

### 🎯 Goal
The Auto-Trade Bot is designed to automate trading strategies (e.g., grid, arbitrage, DCA) on decentralized exchanges, while making each trade **verifiable and transparent** through the Soundness Layer.  
Even though trading endpoints are not yet available in Soundness testnet, this project provides a clear design plan for when they go live.

---

### 🛠 Components
1. **Bot Engine (Executor)**  
   Runs predefined strategies and executes trades on testnet DEXs, returning `tx_hash` logs.

2. **Proof Generator**  
   Builds a proof for every trade including:
   - Trading pair, side, and amount  
   - On-chain `tx_hash`  
   - Timestamp and strategy metadata  

   Example :
   ```json
   {
     "pair": "SUI/USDC",
     "side": "BUY",
     "amount": "25",
     "tx_hash": "0x123...",
     "timestamp": "2025-09-07T03:20:00Z",
     "strategy": "grid-v1"
   }

3. **Walrus Storage**  
   Uploads the proof and returns a **`blob_id`** as a permanent reference.

4. **Soundness CLI**  
   Submits the `blob_id` to the Soundness Layer, which generates an **attestation** to confirm validity.

5. **Dashboard / Explorer / Discord Bot**  
   Publishes the attestations so that any user can independently verify the bot’s trades.

---

### 🔄 Workflow
1. Bot executes a trade → get `tx_hash`.  
2. Proof Generator creates a JSON payload (`proof_payload.json`).  
3. Payload is uploaded to Walrus → returns `blob_id`.  
4. `blob_id` is submitted with Soundness CLI → returns **attestation**.  
5. Attestation is displayed in a dashboard or Discord bot for transparency.

---

### ✅ Value Proposition
- **Transparency:** Every trade has a verifiable proof.  
- **Auditability:** Users and investors can confirm the bot’s activity.  
- **Composability:** Other DeFi projects can consume attestations as inputs.  
- **Future-Ready:** Ready to be extended once Soundness enables direct trading features.

---
## 👤 How Users Use It

### 1. Configure Strategy
The user provides parameters such as:
- Trading pair (e.g., USDC/SUI)  
- Trade amount  
- Thresholds for execution  

### 2. Run the Bot
The bot executes automatically based on the configured strategy (e.g., every X minutes).

### 3. View Trade Records
Users can open the dashboard or Discord bot to see:
- Each trade result  
- Proof payloads  
- Verified attestations  

### 4. Trust but Verify
Instead of trusting logs printed by the bot, users rely on **attestations from Soundness Layer**, which cannot be faked by the bot developer.

---

✨ **In short:**  
- **For the trader** → the bot provides automation.  
- **For the community** → Soundness Layer provides verifiable transparency of each trade.


---

### 📊 Architecture Diagram
[Bot Engine] --(trade)--> [DEX Tx] --> tx_hash
|
v
[Proof Generator] --> proof_payload.json
|
v
[Walrus Storage] --> blob_id
|
v
[Soundness CLI] --> attestation
|
v
[Dashboard/Discord Bot] --> Public Verification
