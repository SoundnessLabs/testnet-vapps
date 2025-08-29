# Proof of Contribution Tracker

**GitHub Username:** novalution  
**Discord Handle:** @novalution  
**Discord ID:**306756808197013506
**Submission Date:** 2025-08-28  

---

## 🧩 Public Idea

The **Proof of Contribution Tracker** is a platform on top of the Soundness Layer that allows projects and communities to **verify real developer contributions** in a transparent and immutable way.  
Instead of relying on self-reporting or Google Forms (which can be spammed), this system leverages Soundness proof generation to record and validate contributions such as:

- ✅ GitHub commits  
- ✅ Pull requests merged  
- ✅ Issues resolved  
- ✅ Code reviews and comments  

This ensures only **genuine contributions** are counted, creating a **trustworthy reputation system** for developers.

---

## ⚙️ Proof of Concept (PoC)

- Use **GitHub API** to fetch contribution metadata (commits, PRs, issues).  
- Convert the activity into **Soundness proofs** using the CLI.  
- Store proof data on-chain (or in verifiable blobs).  
- A simple dashboard (frontend) can show contribution history + score.  

**Technical Flow:**  
1. GitHub → Contribution data  
2. Soundness CLI → Generate proof  
3. Proof stored → Soundness Layer  
4. Dashboard → Fetch & display verified contributions  

---

## 🎯 Why It Matters

- Rewards **quality contributions**, not spam.  
- Helps projects **filter out fake contributors**.  
- Builds a **developer reputation layer** inside the Soundness ecosystem.  

---

## ✅ Next Steps

1. Implement a script to fetch GitHub activity.  
2. Integrate with Soundness CLI for proof generation.  
3. Deploy a minimal frontend for contribution tracking.  
4. Open-source the prototype for community testing.
