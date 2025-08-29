# vApp Proposal

**Project Name:** Mekong Node Hub

**Category:** infrastructure

**Description:**  
Mekong Node Hub is a vApp that provides node-as-a-service infrastructure and monitoring dashboards for Web3 projects.  
We operate full nodes, validators, and RPC endpoints while offering stable APIs for developers to easily integrate.  

**Use case / Problem being solved:**  
- Allow developers and dApps to access blockchain data without running their own nodes.  
- Support projects in both testnet and mainnet with professional validator services.  
- Monitoring & alerting (Prometheus + Grafana + Zabbix) ensure high uptime and security.  

**Technology / How it works:**  
- Node RPC + gRPC + API endpoints deployed across multi-cloud environments (Hetzner, AWS, GCP).  
- Validator nodes with auto-backup + sentry architecture for key protection.  
- Monitoring stack: Prometheus, Grafana, Loki, Alertmanager.  
- Explorer + indexer integration for fast data querying.  

**Why build on SoundnessLabs testnet:**  
- Enables testing of node setup and monitoring before scaling to mainnet.  
- Contributes to network decentralization and reliability.  
- Provides an opportunity for MekongLabs to share infrastructure operations experience with the community.  

**Team / Contributors:**  
- MekongLabs Validator & Infra team  
- GitHub: https://github.com/mekonglabs  
- Explorer service: https://explorer.provewithryd.xyz/  
- Validators: Namada, Story, Avail, Sunrise, Allora, Union...  

**Future vision:**  
- Expand into a node marketplace (RPC, gRPC, indexing).  
- Provide public monitoring dashboards for the community.  
- Support infra bridging across multiple testnets.  

