# Project Name
Sudoku zkApp

## Category
Gaming

## Project Description
A zero-knowledge powered Sudoku puzzle game where players solve a Sudoku grid and generate a proof of correctness without revealing their solution.

The project demonstrates how Soundness can enable privacy-preserving gameplay and trustless leaderboards, proving the validity of a puzzle solution without exposing the actual answers.

## Team Members
- abaynath (solo)

## Links
- GitHub: https://github.com/Abaynath/sudoku-zkapp
- Twitter: https://twitter.com/Aperw3

## Contact
- Discord: Aperw3
- Discord ID: 1222236804556521495
- GitHub: https://github.com/Abaynath

## What problem does this solve?
Traditional puzzle games require revealing the solution to prove correctness. With zero-knowledge proofs, players can demonstrate they solved the puzzle correctly without exposing the answers.

## How does it use Soundness?
- Players solve Sudoku in a web-based UI.
- When finished, they click **Generate Proof**.
- The client generates a zero-knowledge proof that the solution satisfies Sudoku rules.
- The proof is sent to and verified by Soundness Layer.
- Verification returns a “valid/invalid” result without exposing the player’s solution.

## Future Work
- Add multiple Sudoku difficulty levels.
- Introduce a leaderboard based on proof submissions.
- Explore multiplayer “ZK Sudoku tournaments” where players prove they solved puzzles without revealing solutions.
