bash submissions/identity/Mariorizkyy.md
# Proof-of-Presence vApp
**Category:** identity  
**Author (GitHub):** Mariorizkyy  
**Discord ID:** 1144580237670228109  

## Problem
Banyak aplikasi komunitas dan DAO membutuhkan cara sederhana untuk membuktikan bahwa seseorang hadir atau berpartisipasi, tanpa harus mengungkap data pribadi. Saat ini, kebanyakan solusi bergantung pada verifikasi manual atau data terpusat.

## Proposal
Proof-of-Presence vApp memungkinkan pengguna menghasilkan bukti kriptografi sederhana bahwa mereka “hadir” pada suatu event atau aktivitas (misalnya check-in, kontribusi, atau claim hadiah). Bukti ini dapat diverifikasi secara trustless di Soundness Layer, sehingga komunitas atau dApp lain bisa langsung memvalidasi tanpa harus percaya pada server terpusat.

## Architecture & SL Integration
- **Client:** Web (React/Next.js) sederhana untuk generate & submit bukti.  
- **Backend:** Opsional, hanya sebagai relay untuk menyimpan metadata event.  
- **Soundness Layer:**  
  - Menggunakan **Soundness CLI** untuk menghasilkan proof saat user check-in.  
  - Bukti diverifikasi oleh smart contract / service melalui Soundness verification.  
- **Data flow:**  
  1. User klik “Check-In” di UI.  
  2. Soundness CLI generate proof.  
  3. Proof dikirim & disimpan.  
  4. Verifier (organizer/DAO) cek validitas proof langsung di Soundness.  

## Minimal PoC Scope
- Form check-in (username atau wallet address).  
- Generate proof menggunakan Soundness CLI.  
- Verifikasi proof, tampilkan status valid/invalid.  

## Milestones & Timeline
- **M1 (Week 1):** Setup repo + integrasi dasar Soundness CLI.  
- **M2 (Week 2):** Web UI sederhana (input nama, tombol “Generate Proof”).  
- **M3 (Week 3):** Proof verification flow + dokumentasi demo.  

## Risks & Mitigation
- **Risiko:** User kesulitan instal CLI.  
  - **Solusi:** Buat skrip wrapper & tutorial singkat.  
- **Risiko:** Verifikasi lambat saat proof besar.  
  - **Solusi:** Fokus dulu ke proof sederhana untuk PoC.  

## Links
- GitHub pribadi: https://github.com/Mariorizkyy  
- Demo (jika ada): (opsional untuk tahap awal)
