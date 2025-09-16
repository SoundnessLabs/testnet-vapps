# vApp Submission: Chain Detective AI

## Verification
```yaml
github_username: "YvesBrenno"
discord_id: "1128690795868868649"
timestamp: "2025-16-09"
```

## Developer
- **Name**: Fernando Bu
- **GitHub**: @YvesBrenno
- **Discord**: fernando_bu
- **Experience**: AI Engineer with experience in building intelligent agents, RAG systems, blockchain data pipelines, and Python/Node integrations.

## Project

### Name & Category
- **Project**: Chain Detective AI
- **Category**: infrastructure

### Description
An AI-powered detective agent for blockchain transactions.
Users provide a suspicious transaction hash, and the agent traces back the wallets that interacted with the dApp, identifying origins and building a connection graph.
The goal is to improve transparency, track suspicious activity, and assist with compliance and risk assessment.

### SL Integration  
Leverage Soundness CLI to fetch raw transaction data, integrate with the Soundness Layer to extract wallet and dApp interaction details and generate risk scores and structured reports based on transaction graphs.

## Technical

### Architecture
User submits transaction hash → backend queries SL → AI agent processes data → frontend displays investigation report.
Core focus: wallet origin tracing + transaction graph visualization.

### Stack
- **Frontend**: React (dashboard & visualization)
- **Backend**: Python and Node.js
- **Blockchain**: Soundness Layer
- **Storage**: PostgreSQL (logs) + IPFS (report export)

### Features
1 - Transaction hash input
2 - Wallet origin tracing
3 - Graph-based visualization of interactions
4 - Risk scoring using AI-based heuristics

## Timeline

### PoC (2-4 weeks)
- [ ] Backend API receives transaction hash and queries SL
- [ ] Returns JSON with basic wallet interactions
- [ ] Simple prototype chat

### MVP (4-8 weeks)  
- [ ] Graph visualization of transactions
- [ ] Risk scoring and anomaly detection
- [ ] Continuous SL integration

## Innovation
There are no AI agents that act as transaction detectives on dApps integrated with Soundness Layer.
This project uniquely combines on-chain analysis + AI intelligence to enhance transparency, security, and compliance.

## Contact
Preferred: x: @fernando_bu_ or Discord: fernando_bu
Updates: GitHub and X

**Checklist before submitting:**
- [ ] All fields completed
- [ ] GitHub username matches PR author  
- [ ] SL integration explained
- [ ] Timeline is realistic