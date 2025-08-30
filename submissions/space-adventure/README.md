# 🚀 Space Adventure Vapp

## 🎵 Overview
**Space Adventure Vapp** is a web-based space exploration game built on top of the **Soundness Layer**, where players travel across galaxy sectors to discover **on-chain audio artifacts**.  
Each artifact is unique, can be collected, remixed, and used to empower the player’s ship.  
This project serves as an example of integrating **blockchain audio assets** with gameplay.

---

## ✨ Key Features
- 🛰️ Sector exploration with random encounters (artifacts, puzzles, pirates).  
- 🎶 Collect audio artifacts (loop, SFX, stems) stored on the Soundness Layer.  
- ⚡ Loadout system (choose 3 artifacts = buffs such as speed/shield).  
- 🏆 Local leaderboard mock + placeholder for Soundness Layer integration.  
- ⏯️ Pause (P) & restart (R).  

---

## 🎯 Goals
1. Demonstrate how **audio artifacts** can be minted and tracked on the **Soundness Layer**.  
2. Provide an example of combining a **social-audio mechanic** with blockchain integration.  
3. Serve as a foundation for future multiplayer or remix-room expansions.  

---

## 🛠 Tech Stack
- **Frontend:** HTML5 Canvas + JavaScript / WebAudio API.  
- **Blockchain:** Soundness CLI (stub in `main.js` → adapt to your environment).  

---

## �� How to Run Locally
1. Open `index.html` directly in the browser **or** run a simple server:
   ```bash
   # from folder vapps/space-adventure
   python3 -m http.server 8080
   # then open http://localhost:8080
