# vApp Submission: Verifiable Random Number Generator (VRNG) for On-Chain Gaming

## Verification
```yaml
github_username: "0xhours"
discord_id: "1184705235747340369"
timestamp: "2025-08-25"
````

## Developer

* **Name**: Rizal Gilang Permana
* **GitHub**: [@0xhours](https://github.com/0xhours)
* **Discord**: 0xhours
* **Twitter/X**: [@0xhours](https://x.com/0xhours)
* **Email**: [0xhours.eth@gmail.com](mailto:0xhours.eth@gmail.com)

### Experience

I am a blockchain developer with extensive hands-on experience in smart contract development, deployment, and node testing across multiple ecosystems, including **Ethereum-compatible chains, Solana, IBC, and others**.

Highlights:

* Deployed a basic **“Hello World” smart contract** on Swisstronik testnet.
* Built **ERC20 tokens** with mint, burn, transfer, and balance check functionality.
* Developed **ERC721 NFTs** with mint & burn support.
* Implemented **PERC20 privacy-enhanced tokens**.
* Set up nodes and validators on **Solana**, **Cosmos/IBC**, and **EVM chains** for interoperability testing.

Stack: Solidity, Rust (Sui/Move, Solana), Go (Cosmos/IBC), JavaScript/TypeScript, OpenZeppelin, Anchor, zkVMs (SP1, RISC0).

---

## Project

### Name & Category

* **Project**: Sound-UP (VRNG vApp)
* **Category**: Gaming

### Description

**Sound-UP** is a decentralized **Verifiable Random Number Generator (VRNG)** vApp for on-chain gaming, lotteries, and NFTs.

Unlike blockhash-based RNG or oracles (which can be manipulated), this vApp ensures **tamper-resistant, verifiable randomness** using **Zero-Knowledge Proofs (ZKPs)**.

**Features**

* Multi-party entropy commitments.
* ZK-verified randomness mixing.
* Fair game integration (dice, cards, loot drops).
* Cross-chain support via **Soundness Layer (SL)** with <1s latency.

---

## Technical

### Architecture

* **Frontend**: React dApp with Sui Wallet integration.
* **Backend**: Rust vApp on SL testnet; optional settlement on Sui Move.
* **ZK**: SP1/RISC0 zkVM circuits validate SHA-3 entropy mixing.
* **SL**: SL verifiers for proof attestation, Walrus for DA, cross-chain proof export.
* **Flow**:
  `Users commit entropy → vApp aggregates → zk proof → SL verifies → attestation → game contract consumes seed`.

#### Example Handler (Simplified Rust Pseudocode)

```rust
mod VRNG {
    use vapp::*;

    #[derive(ProvableState)]
    pub struct State {
        entropy_pool: Vec<[u8; 32]>,
        random_seed: [u8; 32],
    }

    #[derive(ProvableTx)]
    pub struct SubmitEntropy {
        commitment: [u8; 32],
    }

    #[derive(ProvableTx)]
    pub struct GenerateRandom {}

    #[vApp::Handler]
    pub fn submit_entropy(tx: Context<SubmitEntropy>) -> Result<()> {
        let mut pool = State().entropy_pool;
        pool.push(tx.commitment);
        State().entropy_pool = pool;
        Ok(())
    }

    #[vApp::Handler]
    pub fn generate_random(tx: Context<GenerateRandom>) -> Result<()> {
        let pool = State().entropy_pool;
        require!(pool.len() >= MIN_CONTRIBUTORS, ErrInsufficientEntropy);

        let mut seed = [0u8; 32];
        for commit in pool {
            seed = sha3(seed + commit); // Simplified, real version runs in zkVM
        }

        State().random_seed = seed;
        emit_event(RandomGenerated { seed });
        Ok(())
    }
}
```

---

## Timeline

**PoC (4 weeks)**

* Week 1: Setup vApp SDK, research SL/Walrus, implement entropy submission.
* Week 2: ZK circuits + zkVM integration.
* Week 3: Frontend build, E2E test on SL.
* Week 4: Optimize latency, audit review, MVP deployment.

**MVP (4–8 weeks)**

* Expand to full VRNG service.
* Cross-chain randomness export demo.
* User testing & audit readiness.

---

## Innovation

* First **multi-party ZK-powered RNG** on **Soundness Layer**.
* Combines **entropy commitments + zk proofs + cross-chain attestations**.
* Enables **real-time provably fair gaming** (<1s latency).
* Removes manipulation risks in Web3 gaming.

---

## Contact

* **Discord**: @0xhours
* **Twitter/X**: @0xhours
* **Email**: [0xhours.eth@gmail.com](mailto:0xhours.eth@gmail.com)

---

## Checklist

* [x] All fields completed
* [x] GitHub username matches PR author
* [x] SL integration explained
* [x] Timeline realistic
* [x] Example handler included
