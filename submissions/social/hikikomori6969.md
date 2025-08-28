# vApp Submission: **Anonymous Proof of Participation**

## Verification
```yaml
github_username: "Hiki6969"
discord_id: "864532768201769041"
timestamp: "2025-08-23"
```

## Developer
- **Name**: Hiki Komori
- **GitHub**: @Hiki6969
- **Discord**: hikikomori6969
- **Experience**: Early-stage Web3 developer exploring zero-knowledge proofs and the Soundness ecosystem. 

## Project

### Name & Category
- **Project**: Anonymous Proof of Participation  
- **Category**: Social  

### Description
**Problem**: Right now if you join an event onchain, its always tied to your wallet. That means anyone can just look and see all the stuff you did, what events you went to, even connect it to your other wallets. It's kinda bad for privacy, because maybe you just wanna join some random community call or testnet event but dont want it forever stuck to your main address.  

**Solution**: Anonymous PoP will let a user scan a QR during an event, then generate a zk proof they were there. Later they can show that proof to claim perks or access a channel but it won’t reveal their actual wallet
- This can just plug into Sound L for the zk proof part
- They generate a zk proof confirming attendance  
- The proof can later be redeemed for community perks, gated content, or reputation. Without linking to their wallet identity.

### SL Integration  
PoP can run on top of Sound Layer by using it to handle the zk proofs plus  verification
- This can just plug into SL for the zk proof part  
- Users later use SL to generate zk proof that they are in the set of attendees 
- Verifier contract on SL checks proof before giving access or reward 

## Technical  

### Architecture  
- Front End = QR scan or code input
- Back End = Add commitment to set
- Sound Layer = generate + verify ZK proof 
- Smart Contract = Checks proof before reward or access

### Stack  
- Front End: react (or just simple web) 
- Backend: node or express 
- Smart Contract: solidity  
- ZK: handled with Sound Layer SDK 

### Features  
- Private check in with QR or a code
- ZK proof of attendance
- Works for rewards and/or gated channels
- Can scale to multiple events

## Timeline  

### PoC (2–4 weeks)  
- 1 event only. QR code  
- User gets in the set  
- Later can prove on Sound  Layer 
- Discord bot opens channel

### MVP (4–8 weeks)  
- Same flow but cleaned up UI  
- Multiple events supported  
- Rewards like NFT badge  



## Innovation  
- Most check in systems leak wallet history 
- this one keeps it private using zk plus Sound Layer  
- Its like a Proof of Attendance Protocol but without the tracking

## Contact  
- **Discord**: @hikikomori6969  
- **Email**: 3rdgenerationgodeaterjulius@gmail.com 
