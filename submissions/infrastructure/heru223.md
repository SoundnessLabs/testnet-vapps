# vApp Submission: SL Performance Leaderboard

## Verification
github_username: heru223  
discord_id: 1328329769317695558
timestamp: 2025-08-27

## Developer
- **Name**: Heru Prazz 
- **GitHub**: @heru223  
- **Discord**: @heeeruuuu123
- **Experience**: Infra & tooling background, exploring Soundness Layer testnet, skilled in Node.js/TS, Postgres, Docker, Next.js.  

## Project
### Name & Category
- **Project**: SL Performance Leaderboard  
- **Category**: infrastructure  

### Description
SL Performance Leaderboard is an ecosystem tool that **ranks vApps and zkApps** on Soundness Layer by their proof performance.  
It solves the problem of **lack of transparency and benchmarking** in the testnet: developers currently cannot easily compare their proof speed, reliability, or efficiency.  
With this leaderboard, developers get **clear visibility**, users gain trust, and the community is motivated through **healthy competition**.  

### SL Integration
- Use the **Soundness Layer CLI** to fetch proof lifecycle data (`pending → proving → submitted → valid/failed`) for each vApp.  
- Extract key metrics: average proof latency (p50, p95), success vs failure rates, and total proof counts.  
- Map proofs to their originating vApp/zkApp identifiers for fair ranking.  
- Update leaderboard in near real-time and expose results via **web dashboard + API**.  
- Optional: provide **Discord bot integration** so developers can query leaderboard standings directly from the community channel.  

## Technical

### Architecture
- **Collector**: runs CLI commands to fetch proof data from Soundness Layer.  
- **Database**: stores proof metrics (latency, success/failure, counts).  
- **Leaderboard Service**: aggregates metrics and calculates rankings.  
- **Web Dashboard**: displays leaderboard in real-time.  
- **Bot Integration (optional)**: query rankings from Discord.  

### Stack
- **Frontend**: Next.js (React), Tailwind, Recharts  
- **Backend**: Node.js (TypeScript), Express  
- **Blockchain**: Soundness Layer CLI  
- **Storage**: Postgres (metrics & rankings)  

### Features
1. Collect proof data from Soundness Layer via CLI.  
2. Aggregate metrics (latency, reliability, proof count).  
3. Display leaderboard with filters (by vApp, timeframe).  
4. Optional Discord bot for live queries.  

## Timeline

### PoC (2–4 weeks)
- [ ] Collector service connects to Soundness Layer CLI and ingests proof data.  
- [ ] Basic database schema for proofs & metrics.  
- [ ] Simple leaderboard API (rank by average latency & proof count).  
- [ ] Minimal web dashboard showing top vApps.  

### MVP (4–8 weeks)
- [ ] Full leaderboard with filters (timeframe, app type).  
- [ ] Success/failure rate tracking.  
- [ ] Discord bot integration for leaderboard queries.  
- [ ] UX improvements and documentation.  
- [ ] Testing with early developers in the Soundness testnet.  

## Innovation
Most existing tools around Soundness Layer focus on **proof visibility and cost estimation**, but none provide a way to **compare performance across different vApps**.  
The Performance Leaderboard introduces:  
- **Benchmarking transparency** → developers can see how their vApp performs relative to others.  
- **Gamification** → a healthy competition that motivates optimization and better proof pipelines.  
- **Ecosystem growth** → users and builders gain trust by seeing objective proof performance data.  

## Contact
- **GitHub:** heru223  
- **Discord:** @heeeruuuu123  
  

**Checklist before submitting:**
- [ ] All fields completed  
- [ ] GitHub username matches PR author (`heru223`)  
- [ ] SL integration explained (CLI, proof lifecycle, metrics)  
- [ ] Timeline is realistic (PoC 2–4w, MVP 4–8w)
