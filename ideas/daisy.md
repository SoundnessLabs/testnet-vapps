# **Soundness Layer Testnet vApp Submission**

## **Category**
**Identity – Authentication, Credentials, Reputation**

## **Project Name**
**DAISY (Decentralized Authentication Identity SYstem)**

## **GitHub Username**
**ramagumilar**

---

## **Overview**

DAISY is a **privacy-first** decentralized identity and KYC framework that binds **face biometrics + wallet ownership** to issue **Verifiable Credentials (VCs)** that can be proven on-chain using **zero-knowledge proofs (ZK)**.

Users can authenticate to any dApp (inside or outside Soundness) via **OIDC/SIWE bridge**, and selectively disclose specific facts (e.g., *is over 18*, *is from allowed region*, *passed liveness*) without revealing personally identifiable information (PII).

DAISY is designed to be **composable** across **DeFi, Social, Gaming, and NFT** sectors, enabling universal identity and reputation portability.

---

## **Problem Statement**

* Web3 lacks a standardized, privacy-preserving identity with **portable KYC**.  
* Each dApp re-implements verification → causing **friction, high cost, and data silos**.  
* Off-chain KYC introduces privacy risks; PII stored on-chain is unsafe.  
* No unified way to bridge **Web2 sign-in (OIDC)** with **Web3 wallets** and **ZK claims**.

---

## **Goals & Solution**

**DAISY provides:**

1. **Wallet + Face Binding**  
   One-time enrollment with liveness to bind wallet ownership to a human identity without storing raw biometrics on-chain.

2. **Verifiable Credentials (VCs)**  
   KYC/AML results, age, residency, uniqueness, reputation — issued off-chain, anchored on-chain as commitments.

3. **ZK Selective Disclosure**  
   Prove facts without revealing PII.

4. **Interop Bridges**  
   OIDC provider + SIWE for cross-ecosystem logins (inside/outside Soundness).

5. **Reputation Layer**  
   Optional score composed from verified events (payments, quests, contributions) using privacy-preserving attestations.

---

## **Key Features**

* **Face Liveness & Anti-Spoofing** (enrollment only; encrypted templates stored off-chain).  
* **Wallet Ownership Proof** (signature challenge; AA/4337 compatible).  
* **VC Issuance** (W3C VC/JSON-LD) by trusted issuers/KYC partners; user holds credentials.  
* **On-Chain Commitments** (nullifier/root) to register issuance & revocation.  
* **ZK Proofs (zk-SNARK):** age-over, residency-allowed, uniqueness, sanctions-screened.  
* **Selective Disclosure UI:** granular consent controls per dApp.  
* **Reputation Engine:** aggregate attestations (DeFi, social, gaming) → privacy-preserving composite score.  
* **Interoperability:** OIDC (Web2) + SIWE (Web3) with OAuth2 to JWT/session cookies.  
* **Revocation & Expiry:** issuer revocation registries and credential rotation.  
* **Developer SDK:** JS/TS SDK + smart-contract interface + sample integrations.

---

## **User Flows**

1. **Enroll:**  
   User scans face (liveness), signs wallet challenge → encrypted template stored off-chain → VC issued → on-chain commitment created.

2. **Prove:**  
   On any dApp, user selects a claim (e.g., *over 18*), generates ZK proof in browser/worker, submits proof + nullifier → contract verifies without PII.

3. **Authenticate:**  
   Web3 login (SIWE) or Web2 (OIDC) login trusting DAISY claims; dApp receives only scoped results.

4. **Revoke/Rotate:**  
   Issuer revokes credential; on-chain registry invalidates future proofs. User can rotate wallet binding according to issuer policy.

---

## **Technical Architecture**

### **On-Chain (Soundness Testnet)**

* `DaisyRegistry`: anchors issuer roots, credential schemas, and revocation trees.  
* `DaisyVerifier`: ZK verifier contracts (Groth16/Plonk) for standard circuits.  
* `DaisyReputation`: privacy-preserving reputation aggregator (score commitments + optional reveal gates).  
* `DaisyAccessControl`: RBAC/policy using ZK predicates (e.g., `require AgeOver18`).

### **Off-Chain Components**

* **Issuer Service:** performs KYC (via partner), issues VC to user’s wallet DID; posts commitment & revocation handle on-chain.  
* **Biometric Guard:** liveness + anti-spoofing; produces **non-invertible embeddings**; encrypted in user vault; never on-chain.  
* **Prover Client/Service:** WASM-based browser prover; fallback worker for low-power mobile devices.  
* **OIDC Bridge:** DAISY as OIDC Provider mapping ZK-verified claims to OIDC scopes for Web2 apps.

### **Standards & Schemas**

* DID (`did:key` / `did:pkh`) for wallets.  
* W3C Verifiable Credentials (JSON-LD, ECDSA/EdDSA signatures).  
* ZK Circuits: Age, Residency, Liveness-enrolled, Sanction-screened, Uniqueness, Credential-not-revoked (Merkle proof).

### **Storage**

* User-owned vault (browser + optional backup): encrypted VC & biometric embeddings (AES-GCM; key derived from wallet + device secret).  
* IPFS/Arweave for public metadata (schemas) — no PII.

---

## **Smart Contract Interfaces (Outline)**

```solidity
interface IDaisyRegistry {
    event SchemaRegistered(bytes32 schemaId, address issuer, string uri);
    event RootUpdated(bytes32 schemaId, bytes32 merkleRoot);
    event Revoked(bytes32 schemaId, bytes32 revocationLeaf);
}

interface IDaisyVerifier {
    function verifyAgeOver(bytes calldata proof, bytes32 nullifier) external view returns (bool);
    function verifyResidency(bytes calldata proof, bytes32 regionRoot, bytes32 nullifier) external view returns (bool);
    function verifyNotRevoked(bytes calldata proof, bytes32 schemaRoot) external view returns (bool);
}

interface IDaisyAccessControl {
    function allowIf(bytes4 predicateSelector, bytes calldata proofData) external;
}
```

---

## **Developer Experience (DX)**

* SDK: `@daisy-id/sdk` (TypeScript) → proof generation, wallet binding, OIDC helper.  
* Drop-in Widgets: *EnrollButton*, *ProveAge*, *ProveResidency*, *ReputationBadge*.  
* Integration templates: Next.js for DeFi/Social/Gaming/NFT.

---

## **Integration Examples (Composability)**

* **DeFi:** higher loan LTV for users proving *KYC-passed & reputation ≥ X* without disclosing identity.  
* **Social:** anti-sybil posting via *Uniqueness proof* + *ReputationBadge*.  
* **Gaming:** age-gated content (*AgeOver18*), quest proof → reputation.  
* **NFT:** mint allowlist via *ResidencyAllowed* + *Uniqueness* (anti-bot).

---

## **Security & Privacy**

* **Data Minimization:** only commitments on-chain; no PII.  
* **Liveness:** PAD (Presentation Attack Detection) with challenge–response.  
* **Anti-Linkability:** per-dApp nullifiers; unlinkable proofs.  
* **Revocation:** Merkle revocation trees; proofs include non-revocation path.  
* **Consent & Logs:** user-visible scopes; signed receipts; audit exports.  
* **Compliance:** KYC performed by partner issuers; DAISY only transports proofs.  
* **Recovery:** social recovery/guardian key for re-binding new wallet without re-KYC (if allowed by issuer).

---

## **Testing & QA**

* **Contracts:** Hardhat/Foundry unit & property tests; fuzzing for verifier & access policies.  
* **Circuits:** test vectors; constraint checks; proof verifier gas benchmarks.  
* **Security:** static analysis (Slither), invariant tests, per-scope consent tests.  
* **Load:** prover performance on mobile; fallback worker.

---

## **Risks & Mitigations**

* **Biometric sensitivity:** only encrypted embeddings stored in user vault; delete/export option available.  
* **Issuer trust:** multi-issuer federation; key rotation; transparency logs.  
* **Proof abuse:** nullifier schemes & rate limits; predicate whitelists.  
* **Regulatory variance:** credentials scoped per jurisdiction; issuers handle legalities.

---

## **Contribution to Soundness Ecosystem**

* Standardizes identity predicates (**Age/Residency/Uniqueness/Not-Revoked**) for all Soundness dApps.  
* Reduces onboarding friction with **one-time enrollment** & **portable proofs**.  
* Provides **open templates & SDK** to accelerate vApp development.  
* Bridges **Web2 ↔ Web3** through OIDC/SIWE with privacy by default.

---

## **Future Roadmap**

* DAO governance over schemas/predicates and issuer lists.  
* Hardware-backed enrollment (secure enclaves/Passkeys) for stronger binding.  
* Additional predicates: income bracket (ZK), cross-ecosystem proof-of-human.  
* Cross-chain relays for DAISY proofs on other L2/L3.

---

## **Why Soundness Layer?**

Soundness provides a verifiable execution environment ideal for anchoring commitments, revocation registries, and reusable access policies across the ecosystem.

---

## **Links**

* **GitHub Fork Base:** [https://github.com/ramagumilar/testnet-vapps](https://github.com/ramagumilar/testnet-vapps)  
* **Planned App Repo:** [https://github.com/ramagumilar/daisy](https://github.com/ramagumilar/daisy)

---

## **Contributor Info**

* **Name:** 0xramz  
* **GitHub:** @ramagumilar  
* **Discord:** bacod4783 (ID: 979405292414582864)

---


<img width="1536" height="1024" alt="image" src="https://github.com/user-attachments/assets/757f049b-bc84-40e1-bbe6-fbe284996eda" />


## **License**
MIT License – fully open-source and open for ecosystem contributions.
