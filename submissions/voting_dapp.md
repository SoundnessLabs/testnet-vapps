# VApp Idea Submission

## Project Name
Decentralized Voting dApp

## Category
social

## Description
A decentralized voting application built on top of the **Soundness layer**.  
This dApp allows users to create polls, vote anonymously, and verify the results using **zero-knowledge proofs generated via Soundness CLI**.  

The goal is to showcase how the Soundness layer can guarantee **fairness, privacy, and proof-verifiability** in online decision-making systems.

## Features
- [ ] Create and manage polls with multiple choices  
- [ ] Anonymous voting secured by Soundness proof system  
- [ ] On-chain verifiable results using blob IDs from Soundness  
- [ ] Web interface for easy interaction (React + Tailwind)  
- [ ] CLI integration for developers who want to test voting via **Soundness CLI**  

## Motivation
Voting systems are often questioned for their **transparency and fairness**.  
By integrating the **Soundness proof system**, this VApp will demonstrate how trust can be established without revealing sensitive user data.  

It will also serve as a **demo project** for other developers in the community who want to learn how to integrate Soundness proofs into real-world applications.

## Tech Stack
- Frontend: React + Tailwind  
- Backend: Node.js / Express  
- Smart Contracts: Solidity (optional, for on-chain poll registry)  
- **Soundness CLI & SDK** for generating and verifying proofs  

## Example Flow
1. User creates a poll → poll details stored on-chain or backend DB.  
2. Voter casts a vote → proof generated using **Soundness CLI**.  
3. Proof + blob ID submitted → stored and verifiable by anyone.  
4. Results tally → verified via Soundness layer to ensure correctness.  

## GitHub Handle
@auetgg11
