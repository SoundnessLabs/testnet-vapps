# vApp Proposal - Infrastructure

## Project Name
Soundness Infra Monitor

## Category
infrastructure (Tools, analytics, monitoring)

## GitHub Username
tonihendra

## Discord ID
dencuan#9886

## Description
Soundness Infra Monitor is a vApp for monitoring the health of validator nodes, transactions, and zkApp integrations on the Soundness Layer network.
It focuses on providing a real-time dashboard displaying metrics such as:
- Node uptime & peer connections
- Transaction throughput & latency
- zkApp execution monitoring
- Alerting for error dan downtime

## Technical Architecture
- **Frontend:** React + Tailwind dashboard  
- **Backend:** Node.js (Express) API  
- **Database:** PostgreSQL for saved historical metrics  
- **Integration with SL:**  
  - Query Soundness Layer RPC endpoints untuk mendapatkan data node & transaction  
  - use WebSocket subscription for real-time event monitoring  
  - Expose REST API agar bisa dipakai tools lain  

## Development Timeline
- **Week 1-2:** Setup project skeleton (backend + frontend) and integrated RPC  
- **Week 3:** Tambahkan database + historical metrics storage  
- **Week 4:** Real-time monitoring (WebSocket events)  
- **Week 5:** UI dashboard untuk validator & zkApp metrics  
- **Week 6:** Testing, bug fixing, deployment (PoC testnet)

## Team
- Solo developer (dencuan)

## Additional Notes
- PoC akan open-source on GitHub  
- Plans to integrate into Discord for alerting (node ​​down, transaction fail, etc.) 
