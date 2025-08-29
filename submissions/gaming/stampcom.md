---
title: zk-Treasure Hunt
author_github: stampcom
author_discord: raym0b
category: gaming
---

## Short Description
“zk-Treasure Hunt” is a browser-based adventure game combining puzzle solving and map exploration. Players search for hidden treasures and generate Zero-Knowledge Proofs (ZKPs) to prove their discovery without revealing the exact location.

## Background
The goal of this game is to provide a fun experience while showcasing how the Soundness Layer can be used in gaming. By leveraging ZKPs, players can cryptographically prove their achievements without exposing sensitive details. This project helps introduce the concept of proofs in an engaging and interactive way.

## Technical Architecture & Soundness Layer Integration
1. Frontend: Interactive browser-based UI (HTML/JavaScript), deployed on Walrus.  
2. Proof Generation:  
   - When a player finds a treasure, the game triggers Ligetron to generate a proof.  
   - The proof confirms “player found the treasure” without disclosing the location.  
3. Storage: Proof is uploaded to Walrus → generates a Blob ID.  
4. Soundness Layer: Blob ID is sent via soundness-cli send ... and recorded for verification.  
5. Payload JSON (example):
   `json
   {
     "player": "stampcom.eth",
     "event": "treasure_found",
     "map_id": "island-01",
     "timestamp": "2025-08-29T12:00:00Z",
     "proof_blob_id": "abc123xyz"
   }
