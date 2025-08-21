# vApp Submission: Pukimay

## Verification
```yaml
github_username: "zilzol"
discord_id: "712311563331043399"
timestamp: "2025-01-15"
```

## Developer
- **Name**: donK
- **GitHub**: @zilzol
- **Discord**: donkdmc
- **Experience**: Brief background

## Project

### Name & Category
- **Project**: pukimay
- **Category**: identity

# vApp: zkEmail Attest — bukti domain email tanpa bocor alamat
**Category:** identity
**Author (GitHub):** <USERNAME_KAMU>
**Discord:** <DISCORD_ID_KAMU>

## TL;DR
Bukti “saya punya email di contoh.edu/@company.com” via DKIM + zero-knowledge, tanpa mengungkap alamat email.

## Problem
Verifikasi email biasa membocorkan alamat. Banyak use case cukup butuh domain (mis. .edu atau @company.com).

## Solution
Client ambil header DKIM → buat zk-proof bahwa domain cocok kebijakan (mis. *.edu) → kirim ke Soundness Layer (SL) untuk verifikasi & attestation.

## Architecture
- Client: parse DKIM, buat proof (SNARK/zkVM)
- SL (testnet): endpoint verifikasi → terbitkan attestation
- vApp lain: baca attestation + cek kebijakan & expiry

## Milestones
- Minggu 1–2: rangka circuit + verifier lokal
- Minggu 3–4: integrasi SL verifikasi & attestation
- Minggu 5: demo vApp (gate .edu & @company.com)
- Minggu 6: optimasi & dok

## Risks
Variasi DKIM (RSA/ed25519), DNS cache, anti-sybil (nullifier).
