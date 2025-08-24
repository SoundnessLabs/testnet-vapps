# vApp Proposal: Wordle-ZK

**Category:** gaming  
**GitHub Username:** 0xdkcvd  
**Discord ID:** 328623282892832768  
**Project Repository:** https://github.com/0xdkcvd/Wordle-ZK 
**Demo URL (optional):** -

---

## 1) Summary

**Wordle-ZK** is a minimal verifiable game demonstrating an end-to-end Soundness stack flow:

- Player submits 5-letter guesses (max **6/day**, auto-reset at **SGT midnight**).
- Server returns Wordle feedback (✅/🟡/⬜️) and a **Groth16 (BN254)** artifact (vk, inputs, proof) exported from **SP1**.
- (Optional) Artifact is published to **Walrus** (data availability).
- A transaction **verifies** the proof on **Sui testnet** via a Move verifier function. Sample: https://suiscan.xyz/testnet/tx/2oEQdtoAVmQPNwYqQMkoUA4waXvzAE92yYFWet6KXQw4
- UI links the transaction on **Suiscan**.

Self-contained, easy to run locally, and a clear starter for integrating **SP1 + Soundness + Sui**.

---

## 2) Problem / Motivation

Builders need a concise, production-adjacent example that goes beyond proof generation to show **proof → DA → on-chain verification** with a friendly UI and daily gameplay constraints. Wordle-ZK fills this gap with a familiar game loop and a Vercel-friendly deployment story.

---

## 3) Core Idea / User Flow

1. Player types a 5-letter guess and presses **Enter**.  
2. App prepares a **Groth16** artifact (`artifact.json`: `vk_b64`, `inputs_b64`, `proof_b64`).  
3. (Optional) Artifact is published to **Walrus**, returning a `daCid`.  
4. Backend calls the **Sui Move verifier** with BCS-encoded `vector<u8>` args (vk/inputs/proof).  
5. UI displays **tx digest** with a **Suiscan** link; attempts persist for the current SGT day.

---

## 4) Technical Architecture

**Frontend:** Next.js (App Router)  
- Components: `<Board/>`, `<Keyboard/>`.  
- Daily state persisted in `localStorage` (6 attempts/day), **SGT midnight** auto-reset.  
- Explorer link: `https://suiscan.xyz/testnet/tx/<txDigest>`.

**Server Routes (Node runtime):**
- `POST /api/prove`  
  - Loads `artifact.json` (SP1-exported Groth16 BN254).  
  - Returns UI feedback for the demo; designed to switch to **SP1 journal** later.
- `POST /api/publish`  
  - If `WALRUS_API_URL` is set, publishes artifact and returns `daCid`; otherwise returns a mock so the app remains runnable.
- `POST /api/verify`  
  - Uses `@mysten/sui.js` + `@mysten/bcs` to call:
    - `SOUNDNESS_VERIFIER_TARGET = 0x<package_id>::groth16_verifier::verify_groth16_bn254_proof`
  - Packs vk/inputs/proof as **BCS `vector<u8>`**, avoiding `InvalidBCSBytes` across SDK versions.  
  - Returns `txDigest` on success.

**Environment (.env.example):**
```
SUI_RPC_URL=https://fullnode.testnet.sui.io
SUI_PRIVATE_KEY=suiprivkey... or 0x<32-byte-hex>                                    # not committed
SOUNDNESS_VERIFIER_TARGET=0x<pkg>::groth16_verifier::verify_groth16_bn254_proof
WALRUS_API_URL=                                                                     # optional
WALRUS_API_KEY=                                                                     # optional
SECRET_WORD=APPLE                                                                   # demo-only feedback
ARTIFACT_JSON_PATH=./artifact.json                                                  # demo artifact
```

**Data Formats**
- `artifact.json`
  ```json
  { "vk_b64": "...", "inputs_b64": "...", "proof_b64": "..." }
  ```
- `/api/verify` request
  ```json
  { "artifact": { "vkB64": "...", "inputsB64": "...", "proofB64": "..." }, "daCid": "..." }
  ```

---

## 5) Soundness Layer Integration

- **Proof system:** SP1 Groth16 (BN254).  
- **DA:** Walrus publisher (optional).  
- **On-chain verification:** Sui Move verifier (from `sp1-sui` or equivalent) via `@mysten/sui.js`.  
- Clean hand-off path to later adopt a **Soundness Layer relayer** pattern and journal-driven feedback.

---

## 6) Milestones & Timeline

- **M0:** Proposal approval; scaffold `examples/wordle-zk/`.  
- **M1:** Merge PR with demo artifact; Sui verify succeeds on testnet.  
- **M2:** Enable real Walrus publishing + link to blob viewer (if available).  
- **M3:** Swap demo artifact with a **real SP1 Wordle circuit**; derive UI feedback from the journal.  
- **M4:** Optional scoreboard + tamper-evident daily commit; UX polish.

---

## 7) Success Criteria

- Runs locally with only `.env.local` configured.  
- On-chain verification succeeds (tx visible on Suiscan).  
- Daily limits enforced; state persists across refresh; SGT reset works.  
- Clear README and `.env.example`; **no secrets committed**.  
- Example is sandboxed (does not affect repository root build/tooling).

---

## 8) Risks & Mitigations

- **SDK drift / BCS issues:** Use `vector<u8>` packing; pin `@mysten/sui.js` version.  
- **Walrus availability:** Mock fallback keeps the demo runnable offline.  
- **Secret leakage:** All secrets via env; `.env.local` ignored by Git.  
- **Circuit mismatch:** Keep demo artifact tiny; easily swappable with a real SP1 artifact later.

---

## 9) Individual

- **Lead:** 0xdkcvd (Full-stack; Sui + SP1 integration).  
- Open to collaborate with Soundness reviewers on verifier targets and best practices.

---

## 10) License

MIT
