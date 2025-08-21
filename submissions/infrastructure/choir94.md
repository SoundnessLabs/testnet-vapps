# vApp Proposal: CLI Usage Tracker

**Category:** infrastructure  
**Author:** choir94  
**Discord ID:** choagam

## Overview
A CLI usage tracker vApp that records Soundness CLI command executions (command, timestamp, success/error, duration). Data is stored locally as JSON and verifiable via Soundness proof generation.

## Problem
There is limited visibility into developer flows using Soundness CLI during testnet. We need anonymized telemetry to identify performance issues and common pitfalls without compromising privacy.

## Solution
- A wrapper (`scli`) that routes all CLI calls through `tracker.js`
- Each run appends an entry to `cli_usage_log.json`
- A scheduled job can periodically generate `proof.json` via `soundness-cli proof generate`

## Architecture
1. Developer runs `./scli <args>` instead of `soundness-cli <args>`
2. `tracker.js` executes the command and logs `{command, status, duration_ms, timestamp}`
3. Optional timer invokes `soundness-cli proof generate` on the log to produce `proof.json`

## Development Timeline
- Week 1: MVP logger + wrapper
- Week 2: Proof automation + docs
- Week 3: PoC demo in Soundness_Dev playground

## Resources
- `scripts/cli-usage-tracker/tracker.js`
- README with setup & run instructions
