# vApp Proposal - Soundness Layer Testnet

## GitHub Username
IBBRA533

## Discord ID
imas02485

## Category
identity

## Idea Summary
A decentralized attendance system using facial verification combined with zkProofs to preserve user privacy. The system allows organizations to manage attendance securely while ensuring that no raw biometric data is exposed.

## Technical Architecture
- **Frontend:** React + TailwindCSS for a responsive and modern UI.
- **Backend:** Node.js + Express for API and business logic.
- **Database:** PostgreSQL/MySQL for structured data management.
- **Soundness Layer Integration:**
  - zkProofs are generated locally during face verification, ensuring only proofs (not raw biometric data) are stored and validated.
  - Attendance verification requests are processed through the Soundness Layer to guarantee data integrity and authenticity.
- **Dashboard:** An admin dashboard provides real-time monitoring of who has checked in, with export functionality for reports.

## Development Timeline
- **Week 1–2:** Project setup, UI/UX design, and initial database schema.
- **Week 3–4:** Implementation of facial recognition + zkProof integration with Soundness Layer testnet.
- **Week 5:** Admin dashboard with attendance history and CSV/Excel export.
- **Week 6:** Testing, bug fixes, documentation, and Proof of Concept (PoC) deployment.

## Expected Outcome
- A privacy-preserving decentralized attendance solution.
- Demonstrates how Soundness Layer can be integrated into identity-based applications.
- Ready-to-demo PoC for Soundness Layer testnet validation.
