### Soundness Layer Testnet vApps

Verify GitHub ownership and submit vApp and zkApp proposals for Soundness Layer testnet access.

[![Discord](https://img.shields.io/discord/1234567890?label=Discord&logo=discord)](https://discord.gg/soundnesslabs)
[![PRs](https://img.shields.io/github/issues-pr/soundlayer/testnet-vapps?label=Submissions)](https://github.com/SoundnessLabs/testnet-vapps/pulls)

## Quick Start

1. Fork this repository
2. Copy `TEMPLATE.md` to `submissions/{category}/{your-github-username}.md`
3. Fill out your vApp proposal
4. Create Pull Request
5. Join [Discord](https://discord.gg/soundnesslabs) and use `/submit-vapp` command

## Categories

- **gaming** - Games, NFTs, entertainment

## Requirements

- Valid GitHub username (must match PR author)
- Discord ID for verification
- Technical architecture with SL integration
- Realistic development timeline

## Process

1. **Submit PR** → Automated validation
2. **Team review** → 2-3 business days  
3. **Approval** → Sound_dev Discord role
4. **Build PoC** → Testnet access

## Author Information

- Github username: HoangDungNg
- Discord ID: 501331348460666881
- Role: Frontend Developer

## vApp Idea
A fun puzzle where users align 4 sliders so cubes pulse in sync. 
Solving it proves they are human while generating a proof that Soundness Layer can verify and store on-chain.

## Technical Architecture

Soundness Layer will store the puzzle completion proof in Walrus and create an attestation on Sui. 
Apps like NFT mints or DAOs can then check this attestation to prevent bots.

- Frontend: React + Canvas/GSAP
- Backend: Node.js (API for challenge seeds)
- Blockchain: Sui via Soundness CLI/SDK
- Storage: Walrus for proof blobs

Frontend Demo Link: https://soundness.netlify.app/

## Timeline

PoC: basic slider demo (2–3 weeks)
MVP: full Soundness integration (4 - 5 months)
  
## Resources

- [X](https://x.com/SoundnessLabs)
- [Discord](https://discord.gg/soundnesslabs)



**Ready to build?** [Submit Your vApp](/TEMPLATE.md) • [Join Discord](https://discord.gg/soundnesslabs)
