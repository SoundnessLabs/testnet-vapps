# vApp Proposal: Proof-of-Participation Badges (PoP Badges)

## Verification

```yaml
github_username: 'toastyy00'
discord_id: '431732708842930186'
timestamp: '2025-08-27'
```

---

## Summary

PoP Badges is a verifiable participation system for online and offline events.  
Instead of relying on screenshots or manual check-ins, every participant generates a lightweight proof when joining or completing a session.  
The Soundness Layer verifies this proof, and attendance records are stored on Walrus DA.  
Communities can then issue badges as tamper-proof evidence of engagement.

---

## Architecture

- **Entry Point:** An event host (Discord, Telegram, Zoom, or in-person QR code) distributes a session key.
- **Proof Creation:** Participants use a bot or simple web client to generate a proof of attendance.
- **Verification:** The Soundness Layer verifies the proof in real time.
- **Data Availability:** Attendance records are stored in Walrus DA.
- **Badge Issuance:** Verified participants automatically receive a badge (NFT or credential) that can unlock Discord roles, DAO access, or future rewards.

---

## Timeline

- **Week 1:** Build minimal bot for proof-of-attendance generation.
- **Week 2:** Integrate proof verification with the Soundness Layer.
- **Week 3:** Store verified attendance logs in Walrus DA and issue badges.
- **Week 4:** Add Discord role integration + run a pilot event.
- **Week 5:** Expand to multi-community support and create a dashboard.

---

## Future Plans

- **Stackable Reputation:** Accumulate badges across events to build a verifiable participation history.
- **DAO Governance:** Communities can weight votes based on verified participation rather than wallet balance.
- **Cross-Platform Expansion:** Extend beyond Discord into Telegram, Twitter Spaces, and real-world events via QR codes.

---
