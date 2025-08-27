cat > submissions/proof-explorer.md <<'EOF'
# Soundness Game Vapp — PR-Ready Proposal

**One-liner**  
A lightweight, educational, and interactive game Vapp that turns Soundness proof generation and blob workflows into hands-on challenges and puzzles — helping developers learn, test, and debug Soundness mechanics while having fun.

---

## Problem Statement
- Proof generation and blob handling are technical and can be opaque for newcomers.  
- Developers and contributors lack an approachable, interactive way to practice workflows and reproduce common issues.  
- Community onboarding and testing currently rely on CLI usage and written docs — low engagement and slower learning.

**Goal:** create an engaging, low-friction game that teaches Soundness concepts, reproduces common proof scenarios, and produces real proof metadata that can be used for debugging and metrics.

---

## Concept & Core Mechanics
**Game premise:** Players (developers/testers) complete levels that are mapped to real Soundness operations. Each level is a challenge such as: generate a proof under certain constraints, minimize generation time, fix a failing proof, or retrieve a blob and validate it.

**Mechanics**
- **Levels** — short tasks with clear objectives (e.g., “Generate a proof < 5s for file X”, “Find which flag caused failure”).
- **Live Integration** — when a player runs a level, the Vapp triggers or guides them to run Soundness CLI commands. CLI emits proof metadata which the game ingests to evaluate success.
- **Scoring** — metrics based on generation time, proof size, success/failure, and use of suggested optimizations.
- **Hints & Walkthroughs** — contextual hints teach how CLI flags or inputs affect proofs.
- **Leaderboard / Progress** — optional local leaderboard for team competitions and a progress tracker per user.
- **Sandbox Mode** — safe environment to replay scenarios with cached / synthetic proof data.

---

## Benefits
- **Faster onboarding** — new developers learn by doing, not only reading docs.  
- **Reproducible testing** — common failure modes are codified into levels so maintainers can reproduce issues.  
- **Data generation** — gameplay produces real proof metadata useful for monitoring and debugging.  
- **Community engagement** — gamification increases participation and contributions.

---

## MVP Feature List
- Level selection UI with 6–8 starter challenges (tutorial + simple/advanced).  
- Integration hooks that either:
  - Instruct players to run CLI commands locally and paste emitted JSON, **or**
  - (Optional) Provide an import endpoint to accept CLI-emitted JSON automatically.
- Proof validation engine that reads the CLI metadata schema and checks level success criteria.
- Feedback panel showing proof details: blobId, proofHash, sizeBytes, generatedAt, generationMs, status, and source command.
- Basic scoring and local leaderboard (per browser / per-user).
- Simple analytics endpoint to aggregate generated proof metrics (read-only).
- Level authoring file format so new challenges can be added by maintainers.

---

## Example Level Types
1. **Tutorial - Hello Proof**: generate any successful proof and import metadata.  
2. **Speed Runner**: produce a proof under a target generation time.  
3. **Mini Forensic**: given a failing proof, identify which CLI flag or input likely caused the failure (hint system).  
4. **Size Optimizer**: reduce proof size under a threshold.  
5. **Blob Hunt**: retrieve a blobId and validate chain of linked artifacts.

---

## Integration & Data Contract
### Suggested CLI Metadata Schema (same as Proof Explorer)
\`\`\`json
{
  "blobId": "string",
  "proofHash": "string",
  "sizeBytes": 123456,
  "generatedAt": "2025-08-21T09:42:13Z",
  "generationMs": 7421,
  "source": {
    "command": "soundness proof generate --input ...",
    "version": "cli-1.2.3",
    "commit": "abcdef1",
    "flags": ["--fast", "--format=json"]
  },
  "status": "success" // or "failed",
  "notes": "optional free text"
}
\`\`\`
- The game will validate required fields; optional fields can be used for richer scoring.

---

## Architecture (High Level)
- **Frontend**: SPA (React / Next.js) that handles UI, levels, hints, and local leaderboard.  
- **Backend**: Minimal Node/Serverless service that accepts proof metadata (read/write for gameplay), validates JSON, evaluates level criteria, and stores short-term session records. A read-only analytics API can be exposed for maintainers.  
- **Storage**: lightweight DB (SQLite/Postgres) or JSONL file for MVP.  
- **CLI Hook**: either manual paste/import or small \`soundness\` CLI flag to POST metadata to local game endpoint (documented, opt-in).

---

## API Examples (MVP)
- \`POST /api/session/:sessionId/proofs\` — accepts CLI metadata JSON to evaluate current level.  
- \`GET  /api/levels\` — list of available levels and constraints.  
- \`GET  /api/leaderboard?level=:id\` — leaderboard for a level.  
- \`GET  /api/proofs/:blobId\` — fetch proof detail (for debug/share).

---

## Level Authoring Format (YAML Example)
\`\`\`yaml
id: speed-runner-1
title: Speed Runner — small file
description: Produce a proof for sample.txt under 5000 ms.
target:
  generationMs: { "<": 5000 }
rewards:
  points: 100
hints:
  - "Try using --fast flag"
  - "Consider using smaller input"
\`\`\`

---

## Acceptance Criteria (MVP)
- Player can complete tutorial level end-to-end: run CLI/export metadata → import into game → get validated success/failure feedback.  
- At least 6 playable levels with defined pass/fail criteria.  
- Backend accepts and validates proof metadata and returns actionable feedback.  
- Basic leaderboard and session persistence work for single user.  
- Documentation includes CLI instructions and level authoring guide.

---

## UX / Accessibility Notes
- Clear step-by-step instructions per level (CLI command examples).  
- Keyboard accessibility, readable fonts, and mobile-friendly layout.  
- Friendly failure states with tips to debug.

---

## Security & Privacy
- Gameplay data is optional and user-initiated.  
- Avoid storing sensitive CLI flags or secret paths; recommend redaction guidance in docs.  
- Default mode: local session + optional opt-in upload for team leaderboards.

---

## Directory Structure (Suggestion)
\`\`\`
soundness-game/
├─ README.md
├─ web/
│  └─ src/
│     └─ ... (React app)
├─ api/
│  └─ sessions.ts
│  └─ levels.ts
├─ data/
│  └─ levels/
│     └─ speed-runner-1.yaml
├─ scripts/
│  └─ cli-post-helper.js
└─ docs/
   └─ CLI_INTEGRATION.md
\`\`\`

---

## Quickstart Snippet (README)
\`\`\`markdown
# Soundness Game — Quickstart (MVP)

## Dev setup
1. `pnpm install`
2. `pnpm dev`  # runs web + api locally
3. Open the app at http://localhost:3000

## Playing a level
1. Open the desired level in the UI.
2. Follow the CLI example shown (or run your own).
3. Save the CLI JSON output and import via the game's "Import proof" button,
   or use the helper: `node scripts/cli-post-helper.js /path/to/proof.json`.

## Authoring a level
- Add a YAML level file to `data/levels/` following the authoring schema.
\`\`\`

---

## PR Description Template (copy-paste)
\`\`\`markdown
**Title:** Soundness Game Vapp — Interactive learning & testing game for proofs

**Summary**
This PR adds a new Vapp proposal: Soundness Game. The app turns common Soundness CLI/proof workflows into playable levels that teach and reproduce proof scenarios, producing usable proof metadata for debugging.

**Motivation**
- Improve onboarding and learning
- Create reproducible test scenarios for maintainers
- Increase community engagement

**Scope (MVP)**
- Tutorial + 5 gameplay levels
- CLI metadata import (manual + helper)
- Backend session evaluation + local leaderboard
- Level authoring format + documentation

**Non-Goals**
- Full multi-tenant leaderboard
- Production analytics (post-MVP)

**Testing**
- Manual tests for level validation using sample proof JSONs.

**Follow-ups**
- Add more levels, team leaderboards, and CI integration for level validity.
\`\`\`

---

## Discord Submission Helper
After you open the PR, submit to the dev bot:
\`\`\`
/submit_vapp url:<PR link> title:"Soundness Game Vapp" description:"Interactive game to teach and test Soundness proof flows — includes levels, CLI import, and scoring."
\`\`\`

---
EOF
