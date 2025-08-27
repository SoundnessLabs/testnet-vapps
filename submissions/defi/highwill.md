# GasShield

## Category
defi

## Project Title
GasShield: Batched Proof-of-Intent Swaps

## Summary
GasShield allows users to create lightweight swap intents off-chain. Executors collect these intents, simulate multiple routes, select the best one, and then prove via the Soundness Layer (SL) that the swap execution was efficient and transparent.  
This way, users don’t need to worry about complex gas parameters or MEV risks, because a verifiable proof (blob id) is published to guarantee the swap was executed under the best available conditions at that time.

## Architecture & SL Integration
- **Frontend**: a simple swap form where users sign an intent (token in/out, amount, slippage).  
- **Relayer/Executor**: collects intents, simulates candidate routes, selects the optimal one, requests a proof from SL, and publishes the blob id.  
- **Soundness Layer**: generates a proof that the chosen swap route meets the constraints (slippage & optimality) and stores it as a blob for verification.  
- **Verifier (optional)**: a lightweight on-chain contract to verify intents & proof references.

## MVP Scope
- 1 testnet chain (Base Sepolia).  
- 1–2 simple DEX routes.  
- Minimal intent format (tokenIn, tokenOut, amount, maxSlippage).  
- A demo page displaying swap results along with the blob id proof.  

## Milestones & Timeline
- **Week 1–2**: Design minimal intent + simple relayer, integrate with Soundness Layer (initial proof flow).  
- **Week 3–4**: Implement proof publishing via SL, deliver end-to-end demo with blob id displayed.  

## Risks & Dependencies
- Proof generation latency could slow execution.  
- Fast market price changes may affect the definition of the “best” route.  
- Dependence on testnet DEX/aggregator endpoints.  

## Success Criteria
- Proofs available for ≥90% of demo executions.  
- Time from intent submission → proof availability < a few minutes.  
- Public demo: users can see swap results + blob id proof link.  

## Contact
- **GitHub**: [highwill](https://github.com/highwill)  
- **Discord**: williamkasadi
