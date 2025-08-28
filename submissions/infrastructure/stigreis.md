# vApp Submission: Soundness Monitor vApp

## Verification
```yaml
github_username: "stigreis"
discord_id: "123456789012345678"
timestamp: "2025-08-27"
```

## Developer
- **Name**: Stig Reis
- **GitHub**: @stigreis
- **Discord**: StigReis#0000
- **Experience**: Full-stack developer with infra and blockchain monitoring experience; familiar with Prometheus/Grafana and Node.js/TypeScript stacks.

## Project

### Name & Category
- **Project**: Soundness Monitor vApp
- **Category**: infrastructure

### Description
A lightweight monitoring and alerting vApp designed for the Soundness Layer testnet. It continuously polls the SL RPC to observe chain head and finalized height, computes finality lag, exposes Prometheus metrics and a REST status API, and issues alerts (Discord webhook) when thresholds are exceeded. The vApp helps validators, node operators and dApp developers quickly detect and respond to network degradations or potential consensus stalls.

### SL Integration  
This vApp integrates with Soundness Layer by:
- Polling the SL RPC endpoint for chain head, finalized height and block metadata.
- Optionally querying validator set and block production metrics when SL exposes these endpoints.
- Emitting observability metrics that can be correlated with SL on-chain events and zk-proof lifecycle hooks (where available).
- Preparing hooks for future zkApp interactions to validate proof finality or on-chain state syncs.

## Technical

### Architecture
High-level components:
1. **Agent (Node/TypeScript)**: scheduled poller that fetches chain head/finality and updates internal state and Prometheus metrics.
2. **API Layer (Express)**: serves `/health`, `/status` and `/metrics` endpoints.
3. **Alerting**: Discord webhook sender that posts formatted alerts when thresholds are breached.
4. **Deployment**: Docker container; supports docker-compose and CI via GitHub Actions.
5. **Optional**: Grafana dashboard and exporter sidecar for advanced visualizations; WALRUS storage for long-term event logs and snapshots.

### Stack
- **Frontend**: (optional) React for a lightweight dashboard
- **Backend**: Node.js + TypeScript (current scaffold)
- **Blockchain**: Soundness Layer (SL) RPC + potential zkApp integration
- **Storage**: WALRUS for logs/snapshots; optional PostgreSQL for stateful retention; Prometheus for metrics

### Features
1. Poll SL RPC for head and finalizedHeight; calculate finality lag.
2. Expose `/metrics` (Prometheus) and `/status` (JSON) endpoints.
3. Send alerts to Discord when finality lag or RPC latency exceed thresholds.
4. Dockerized deployment and CI for reproducible builds.
5. Optional Grafana dashboard sample and WALRUS snapshot export for forensics.

## Timeline

### PoC (2-4 weeks)
- [ ] Basic functionality (polling, metrics, status endpoint)
- [ ] SL integration (wire up real RPC methods)
- [ ] Simple UI (optional minimal dashboard)

### MVP (4-8 weeks)  
- [ ] Full features (validator checks, latency histograms, retry/backoff)
- [ ] Production ready (health checks, containerization, monitoring)
- [ ] User testing (validators and developer feedback)

## Innovation
What makes this unique? The vApp is lightweight, specifically tailored to Soundness Layer's finality model and testnet workflows. It focuses on finality lag as a primary health indicator (critical for chains with probabilistic finality) and provides ready-to-use observability hooks (Prometheus + Discord). The scaffold also anticipates zk-proof integrity checks, enabling future-proof integrations with zkApps and audit trails stored in WALRUS.

## Contact
Preferred contact: GitHub issues and PRs at https://github.com/stigreis/soundness-monitor-vapp or Discord StigReis#0000. Updates and progress will be published to the repo and the Soundness Discord channel.

**Checklist before submitting:**
- [x] All fields completed
- [x] GitHub username matches PR author  
- [x] SL integration explained
- [x] Timeline is realistic
