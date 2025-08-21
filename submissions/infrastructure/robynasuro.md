# vApp Submission: Soundness TUI

## Verification
```yaml
github_username: "robynasuro"
discord_id: "435276766635098112"
timestamp: "2025-08-22"
```

## Developer
- **Name**: Roby 
- **GitHub**: [@robynasuro](https://github.com/robynasuro)
- **Discord**: 0xcreamy.eth
- **Experience**: Rust + WebAssembly developer focused on Web3 tooling and infrastructure. Previous work includes building CLI/TUI applications and integrating blockchain testnets.

## Project

### Name & Category
- **Project**: Soundness TUI
- **Category**: infrastructure

### Description
**Soundness TUI** is a lightweight WebAssembly-based tool designed to help developers and testers interact with the Soundness Layer (SL).

With this TUI, users can:
- Generate or import keypairs based on mnemonics,
- Encrypt and securely store keys in the browser,
- Compose payloads and submit proofs to the Soundness testnet with a single click.

The problem it solves: many developers struggle to test proof flows without setting up a full backend stack. This TUI provides a direct browser interface — fast, secure, and simple.

### SL Integration
- Submits proofs to the `https://testnet.soundness.xyz/api/proof` endpoint via a proxy.
- Signs canonical strings using Ed25519, verified by the SL.
- Supports both `game` and `elf_file` proof modes according to the Soundness protocol.
- Future roadmap: proof verification directly inside the UI.

## Technical

### Architecture
Browser (Rust + Yew → WASM)
     │
     ├──> Vercel Proxy (Node.js serverless API)
     │         │
     │         └──> Soundness Testnet API (/proof)
     │
     └──> Local keystore (encrypted, password-based)

### Stack
- **Frontend**: Rust + Yew (compiled to WebAssembly)
- **Backend/Proxy**: Node.js (Vercel Serverless)
- **Blockchain**: Soundness Layer testnet
- **Storage**: Encrypted keystore in browser (AES-GCM-SIV + PBKDF2), JSON

### Features
1. Generate keypair + mnemonic (BIP39)
2. Import mnemonic (testnet only)
3. Encrypted key storage (password-protected)
4. Compose & sign canonical strings
5. Submit proofs via proxy to SL
6. Display server responses directly in the UI

## Timeline

### PoC (2-4 weeks)
- [x] Keypair generation & import
- [x] Basic proof submission flow
- [x] Vercel proxy API → SL testnet
- [ ] Add security hardening (zeroize, AAD)

### MVP (4-8 weeks)
- [ ] Screenshots/GIF demo + UX polishing
- [ ] CI + test coverage (wasm build + proxy tests)
- [ ] Migration to Argon2id + ephemeral storage
- [ ] Collect user feedback and iterate

## Innovation
- **Zero-setup**: open the URL and immediately generate keys & submit proofs.
- **Browser-native**: runs via WebAssembly, no installation needed.
- **Secure by design**: encrypted secret keys, password-derived encryption keys, no plaintext storage.
- **Developer-friendly**: structured logging and detailed error handling, ideal for debugging Soundness PoCs.

## Contact
- **Discord**: 0xcreamy.eth
- **GitHub Issues**: [robynasuro/soundness-tui](https://github.com/robynasuro/soundness-tui/issues)
- **Preferred updates**: Discord  + GitHub repo

---

**Checklist before submitting:**
- [x] All fields completed
- [x] GitHub username matches PR author
- [x] SL integration explained
- [x] Timeline is realistic
