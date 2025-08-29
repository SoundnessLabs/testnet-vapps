# vApp Submission: Gas Tracker

## Verification
```yaml
github_username: "atti1233"
discord_id: "428885188399988746"
timestamp: "2025-01-15"
```

## Developer
- **Name**: Attilio
- **GitHub**: @atti1233
- **Discord**: patrickstar0749
- **Experience**: Industrial Engineering Graduated Bachelor Degree, Node Runner, Testnet participants, Data Entry 

## Project

### Name & Category
- **Project**: Gas Tracker
- **Category**: infrastructure

### Description
What problem does your vApp solve? What does it do?
An application that monitors transaction fees on the Soundness testnet (like a mini explorer).
Simple: just fetch transaction data from RPC and display a graph/log.
### SL Integration  
How will you use Soundness Layer? What specific SL features?
The Gas Tracker application will utilize the Soundness Layer primarily as a data backbone to access transaction and block information on the testnet. Through the Soundness Layer, the application can retrieve detailed records of gas fees, block times, and transaction metadata in a way that is transparent and verifiable. By relying on SL’s verifiable data access, users can be confident that the gas usage and cost metrics displayed are accurate and derived directly from the network itself, without requiring trust in an external service.
In practice, the Gas Tracker will connect to the Soundness Layer’s transaction and block data APIs to read gas consumption per transaction, calculate average gas usage across blocks, and present these insights through a user-friendly dashboard. If event streaming or subscription features are available, the app will also use them to update the displayed metrics in real time whenever new transactions are confirmed on the testnet. This combination of features makes the Soundness Layer not just a source of raw data, but also a trusted foundation for building an accessible monitoring tool for the ecosystem.
## Technical
Gas Tracker is a lightweight infrastructure tool designed to monitor and display real-time gas usage and transaction costs on the Soundness Layer (SL) testnet.
It will consist of a frontend dashboard, a backend API service, and a direct integration with the Soundness Layer RPC/Graph API to fetch verifiable transaction and block data.
### Architecture
High-level system design and approach
1.Backend service connects to the SL RPC/Graph endpoint.
2.Backend extracts block data, transaction details, and gas usage.
3.Processed data is stored temporarily (in a database or cache).
4.Frontend queries backend API to visualize metrics in charts and tables.
5.If SL supports event streaming, backend subscribes and pushes real-time updates to the frontend.
### Stack
- **Frontend**: React (with TailwindCSS + Recharts for data visualization)
- **Backend**: Node.js (Express + WebSocket for real-time updates)
- **Blockchain**: Soundness Layer (primary), optional Ethereum testnet for comparison
- **Storage**: PostgreSQL for processed metrics, optional WALRUS/IPFS for long-term logs

### Features
1. Gas Fee Explorer, Display average gas fee per block and transaction.
2. Real-time Dashboard, Live updates of latest block gas consumption using SL data streams.
3. Historical Analytics, Store data in DB and show trends (daily/weekly average gas).

## Timeline
Week 1: Setup & Basic Integration
 Fork & initialize repository with base project structure.
 Setup backend (Node.js + Express) and connect to Soundness Layer RPC/Graph API.
 Fetch raw block and transaction data.
 Setup frontend project (React + TailwindCSS).
Week 2: Core Features Implementation
 Implement gas fee calculation (avg gas per transaction, per block).
 Store processed data into PostgreSQL.
 Create simple API endpoints for the frontend.
 Build UI components for displaying gas metrics (tables & basic charts).
Week 3: Real-Time Dashboard & SL Enhancements
 Integrate WebSocket/event listener for live transaction updates.
 Add real-time chart updates on frontend (Recharts).
 Enhance SL integration for verifiable data access.
 Conduct first round of internal testing.
Week 4: Refinement & Documentation
 Improve dashboard UI/UX.
 Add historical analytics view (daily/weekly trends).
 Write project documentation & usage guide.
 Deploy demo version of Gas Tracker (Codespaces/Netlify/Heroku).
 Submit PoC report to SoundnessLabs repo.
### PoC (2-4 weeks)
- [Week 1-2] Basic functionality: Connect to SL RPC and fetch block/transaction data.
- [Week 3] SL integration: Calculate average gas usage and verify data directly from SL.
- [Week 4] Simple UI: Build frontend dashboard with real-time chart updates.

### MVP (4-8 weeks)  
- [Week 5-6] Full Features
Expand historical analytics (daily, weekly, monthly gas cost trends).
Add filtering & sorting (by block range, transaction type, or account).
Implement user-friendly charts (Recharts advanced graphs).
Add export feature (CSV/JSON for developers).
Optimize backend queries for speed & scalability.
- [Week 7] Production Readiness
Set up CI/CD pipeline (GitHub Actions).
Deploy backend on cloud (e.g., AWS/Heroku).
Deploy frontend on Vercel/Netlify.
Configure database backups & monitoring.
Conduct security checks and stress testing.
- [Week 8] User Testing & Feedback
Invite early testers (Soundness community, Discord devs).
Collect feedback and bug reports.
Refine UI/UX based on tester input.
Document deployment steps and usage guide for developers.
Prepare final submission/demo to Soundness testnet showcase.
## Innovation
What makes this unique? Why will people use it?
What makes Gas Tracker unique is its trustless and verifiable approach: instead of relying on centralized APIs, it leverages the Soundness Layer’s verifiable data access. This ensures that gas metrics are accurate, transparent, and sourced directly from the blockchain.
Developers, testers, and community members will use it to understand the cost dynamics of the Soundness testnet, benchmark transaction efficiency, and identify potential optimizations. Unlike typical explorers, Gas Tracker focuses specifically on gas monitoring and trend analysis, making it a valuable tool for developers building on SL.
## Contact
Preferred contact method and where you'll share updates.
Twitter = @AttiJrX
Email = atti.leo87@gmail.com

**Checklist before submitting:**
- [✔] All fields completed
- [✔] GitHub username matches PR author  
- [✔] SL integration explained
- [✔] Timeline is realistic
