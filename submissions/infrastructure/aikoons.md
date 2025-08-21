# vApp Submission: SL Ping

## Verification
```yaml
github_username: "aikoons"
discord_id: "676291916039585832"
timestamp: "2025-01-15"
```

## Developer
- **Name**: Mahdy Saifan
- **GitHub**: @aikoons
- **Discord**: ohaikoo#0
- **Experience**: experience in Linux servers, simple APIs (Node.js/Python), and basic blockchain RPC integrations.

## Project

### Name & Category
- **Project**: SL ping
- **Category**: infrastructure / monitoring

### Description
SL Ping is a lightweight health checker for Soundness Layer RPC endpoints.
Problem it solves:

Node operators and developers often don’t know if an endpoint is healthy or lagging.

SL Ping provides a simple status page showing:

RPC availability (✅ online / ❌ offline)

Block height returned by the node

Response time in milliseconds

This gives operators quick visibility without needing a full monitoring suite.

### SL Integration  
SL Integration

Query SL RPC methods (eth_blockNumber, net_version, etc.) to check liveness.

Compare block height with a reference RPC to detect sync lag.

Optionally store recent ping results on WALRUS for historical status snapshots.

## Technical

### Architecture
Backend script pings configured SL RPC endpoints every few seconds.

Results are stored in memory (or WALRUS for persistence).

Frontend dashboard fetches and displays status, block height, and latency.

### Stack
Frontend: React + Tailwind (minimal dashboard)

Backend: Node.js (Express) or Python (FastAPI)

Blockchain: Soundness Layer (via RPC)

Storage: WALRUS (for optional history)

### Features
Check endpoint availability (online/offline).

Show current block height & detect sync lag.

Display latency (ping time).

Simple UI with ✅/❌ indicators.

## Timeline

### PoC (2-4 weeks)
- [ ] Basic functionality
- [ ] SL integration
- [ ] Simple UI

### MVP (4-8 weeks)  
- [ ] Full features
- [ ] Production ready
- [ ] User testing

## Innovation
Extremely lightweight and easy to deploy (one small script + simple UI).

Tailored for Soundness Layer, unlike generic “ping” tools.

Helps developers and node operators quickly verify RPC health.

## Contact
discord : ohaikoo#0 
twitter : aikoons1


**Checklist before submitting:**
- [ ] All fields completed
- [ ] GitHub username matches PR author  
- [ ] SL integration explained
- [ ] Timeline is realistic
