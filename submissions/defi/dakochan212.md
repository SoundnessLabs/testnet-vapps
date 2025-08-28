## **AutoYield vApp (Smart Yield Router)**

**Category**  
DeFi / Yield Optimization

**Problem**  
Users have to manually move funds between protocols to chase the best yield (staking, LP, lending). Gas fees and complexity discourage small investors.

**Solution**  
**AutoYield vApp**:

-   Users deposit stablecoins (e.g., USDC) into one contract.
    
-   Contract checks/testnet-oracles for best APR across multiple pools.
    
-   Funds are auto-routed to the best pool.
    
-   zk-proofs ensure rebalancing is **transparent + verifiable**.
    

**Why Valuable**

-   Brings DeFi efficiency to everyone (no need to be a DeFi degen).
    
-   Reduces gas waste by batching moves with proofs.
    
-   Great showcase for Soundness infra (cheap, provable automation).
    

**Architecture**

1.  Deposit → AutoYield contract.
    
2.  zk-proof verifies APR check + rebalance decision.
    
3.  Funds routed to best pool.
    
4.  User can withdraw anytime with proof of yield earned.
    

**Roadmap (Simple)**

-   **Week 1:** Deploy AutoYield contract + stablecoin deposit.
    
-   **Week 2:** Add mock pools + APR oracle.
    
-   **Week 3:** Implement zk-proof for rebalancing logic.
    
-   **Week 4:** Build frontend dashboard → “Deposit $10, earn auto-yield.”
