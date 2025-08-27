# 🎮 Space Shooter Vapp

## 🔎 Overview
**Space Shooter Vapp** is a web-based 2D game built on top of the **Soundness Layer**, where players control a spaceship to shoot enemies and avoid obstacles.  
This project serves as a simple example of an **on-chain game** with score storage.

##⚙️  Key Features
- 🚀 Keyboard controls (WASD/Arrow) + Space to shoot. 
- 👾 Dynamic enemy spawns with increasing difficulty. 
- 💎 Local score tracking + stub function to store scores on the Soundness Layer. 
- 🏆 Leaderboard (local mock + placeholder for on-chain integration). 
- ⏸️  Pause (P) & restart (R). 

## 🚀 Goals
1. Demonstrate score and leaderboard storage through the **Soundness Layer**. 
2. Provide an example of lightweight game + blockchain integration. 
3. Serve as a foundation for further development within the Soundness ecosystem. 

## 👨‍💻 Tech Stack
- **Frontend:** HTML5 Canvas + JavaScript 
- **Blockchain:** Soundness CLI (stub in `main.js` → adapt to your environment) 

## ▶️  How to Run Locally
1. Open `index.html` directly in the browser **or** run a simple server: 
   ```bash
   # from folder vapps/space-shooter
   python3 -m http.server 8080
   # then open http://localhost:8080
