# Queens Duel – Anti-bot PvP strategy game

## Author
GitHub: animsopandi  
Discord ID: 1008523761127010365  

## Category
gaming  

## Short description
Queens Duel is a 1v1 strategy game based on the classic "8 Queens puzzle."  
Players take turns placing queens on a shared chessboard. The goal is to place more valid queens than the opponent.  
Rules ensure fair play and prevent botting or multi-accounting.  

## Technical architecture
- **Board Setup**: Both players secretly place their first queen simultaneously.  
- **Turns**: After the initial placement, players alternately place one queen at a time.  
- **Rules**: A queen cannot be placed in a row, column, or diagonal already attacked by another queen.  
- **Victory Condition**: When no valid squares remain, the player who placed the most queens wins.  
- **Rewards**: The winner receives a CLI access ticket (testnet reward), ensuring only real users who engage can progress.  

## Soundness layer integration
- **Proof of Play**: Every move generates a lightweight zk-proof that verifies the placement was valid under the rules.  
- **Anti-bot validation**: Since moves require strategic reasoning, bots and cloned accounts are filtered out by proof checks.  

## Development timeline
- Week 1: Design chessboard UI + basic game logic.  
- Week 2: Implement 1v1 matchmaking + simultaneous placement feature.  
- Week 3: Add zk-proof integration for move validation.  
- Week 4: Testing + CLI reward distribution.  
