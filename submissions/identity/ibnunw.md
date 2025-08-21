# vApp Submission: Password Proof Generator

## Verification
```yaml
github_username: "ibnunw"
discord_id: "815144001753317377"
timestamp: "2025-08-21"
```

## Developer
- **Name**: Sinunu
- **GitHub**: @ibnunw
- **Discord**: sinunu182
- **Experience**: Web developer with basic knowledge of JavaScript and CLI tools. Learning zero-knowledge applications.

## Project

### Name & Category
- **Project**: Password Proof Generator
- **Category**: identity

### Description
A simple web tool that demonstrates zero-knowledge proof fundamentals. Users input a password and generate cryptographic proof that they know the password without revealing the actual password. This showcases the core privacy-preserving capability of zk-technology.

### SL Integration  
Using Soundness Layer for proof generation:
1. Generate zk-proofs from password inputs using SL CLI
2. Simple proof verification demonstration
3. Utilizes SL's proof generation runtime for hash preimage verification

## Technical

### Architecture
Web Interface → Password Input → Proof Generation (SL CLI/WASM) → Proof Display

### Stack
- **Frontend**: HTML/CSS/JavaScript
- **Backend**: None (or minimal serverless function if needed)
- **Blockchain**: Soundness Layer (proof generation only)
- **Storage**: None (all client-side)

### Features
1. Password input field
2. Proof generation button
3. Proof display output
4. Basic verification demonstration

## Timeline

### PoC (2 weeks)
- [ ] Basic web interface
- [ ] SL CLI integration for proof generation
- [ ] Proof display functionality
- [ ] Local verification setup

### MVP (3 weeks)
- [ ] Improved UI/UX
- [ ] WASM-based proof generation
- [ ] Enhanced verification features
- [ ] Documentation and examples

## Innovation
Demonstrates fundamental zk-concept in simplest form - proving knowledge without revelation. Perfect educational tool for understanding ZKP basics while providing practical SL integration example.

## Contact
Discord: sinunu182. Will share progress in Soundness Discord and maintain GitHub repository.

**Checklist before submitting:**
- [x] All fields completed
- [x] GitHub username matches PR author
- [x] SL integration explained
- [x] Timeline is realistic
