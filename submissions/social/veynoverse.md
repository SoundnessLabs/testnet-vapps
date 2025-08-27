# Proof of Reputation for Open Source Devs

**Category:** social  
**Author (GitHub):** veynoverse  
**Discord ID:** raverynverse (1251372132336930836)  

## 1) Ringkasan singkat
vApp untuk memberi **reputation score** kepada developer berdasarkan PR yang di-merge di GitHub, diverifikasi menggunakan Soundness Layer. Dengan ini, developer dapat membuktikan kontribusinya tanpa harus mengungkap identitas pribadi.

## 2) Nilai & use case
- Developer dapat bukti kontribusi open source tanpa doxxing.
- Komunitas dapat menggunakan proof ini untuk mengelola role, akses bounty, atau reputasi.
- Target pengguna: open-source developers dan proyek DAO.

## 3) Integrasi dengan Soundness Layer
- Proof kepemilikan akun GitHub + event PR merge.
- Generate proof melalui Soundness CLI, diverifikasi oleh smart contract sebelum badge diberikan.

## 4) Arsitektur Teknis
- Frontend: Next.js
- Smart Contract: Solidity (Testnet)
- Soundness: Prover & Verifier API

## 5) Roadmap
- Minggu 1: Proof GitHub account & event PR
- Minggu 2: UI dasar untuk submit & verify
- Minggu 3: Deploy smart contract verifier
- Minggu 4: Testing, dokumentasi, dan demo

## 6) Tim
Solo builder

## 7) Tautan
- Repo: akan diperbarui setelah PoC
