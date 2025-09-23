# Project Name
Sudoku zkApp

## Category
Gaming

## Project Description
A zero-knowledge powered Sudoku puzzle game where players solve a Sudoku grid and generate a proof that their solution is valid without revealing the actual solution. The app demonstrates how Soundness Layer can be used to verify constraint satisfaction problems in a gamified and interactive way.

## Team Members
- abaynath (solo)

## Links
- GitHub: https://github.com/Abaynath/sudoku-zkapp (placeholder, will update if repo is created)
- Twitter: https://twitter.com/Aperw3 

## What problem does this solve?
Traditional puzzle games require revealing the solution to prove correctness. With Soundness, we can generate a proof that all Sudoku rules are satisfied — each row, column, and 3x3 block contains digits 1–9 without repetition — without revealing the grid itself. This highlights the power of zero-knowledge proofs in a fun, accessible format.

## How does it use Soundness?
- Players solve Sudoku in a web-based UI.
- When finished, they click **Generate Proof**.
- The client generates a zero-knowledge proof that the solution satisfies Sudoku constraints.
- The proof is sent to and verified by Soundness Layer.
- Verification returns a “valid/invalid” result without exposing the player’s solution.

## Future Work
- Add multiple Sudoku difficulty levels.
- Introduce a leaderboard based on proof submissions.
- Explore multiplayer “ZK Sudoku tournaments” where players prove they solved puzzles the fastest without leaking answers.

