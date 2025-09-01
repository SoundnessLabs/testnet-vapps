# vApp Submission: [Your Project Name]

## Verification
```yaml
github_username: "VanZ27"
discord_id: "1041471918391427196"
timestamp: "2025-09-01"
```

## Developer
- **Name**: Van Z
- **GitHub**: @VanZ27
- **Discord**: 0x123van
- **Experience**: Web3 enthusiast & aspiring developer. Experienced with basic JavaScript, GitHub Codespaces, and deploying small web apps.

## Project

### Name & Category
- **Project**: Random Quote Generator
- **Category**: social

### Description
Random Quote Generator is a simple vApp that displays random inspirational quotes.
The app ensures transparency by using Soundness Layer to generate cryptographic proofs that the quotes come from an official dataset and have not been altered.

### SL Integration  
We will integrate Soundness Layer to:
-Generate verifiable proofs ensuring the quote is randomly selected.
-Allow users to verify each displayed quote independently.
-Use the Soundness CLI to handle proof generation and validation.

## Technical

### Architecture
The app follows a client-first architecture:
1. User clicks a button to request a random quote.
2. The app fetches a quote from a pre-defined JSON dataset.
3. A zero-knowledge proof is generated via Soundness SDK.
4. The proof is verified before displaying the quote.
5. Users can optionally share verified quotes.

### Stack
- **Frontend**: HTML, TailwindCSS, JavaScript
- **Backend**: Node.js 
- **Blockchain**: Soundness Layer for proofs
- **Storage**: Static JSON file (official quotes dataset)

### Features
1. Display random quotes instantly.
2. Generate cryptographic proofs for each quote.  
3. Allow users to verify quote authenticity via Soundness Layer.

## Timeline

### PoC (2-4 weeks)
- [ ] Setup basic UI for displaying random quotes.
- [ ] Integrate Soundness CLI for proof generation.
- [ ] Display verified quotes.

### MVP (4-8 weeks)  
- [ ] Add interactive UI & animations.
- [ ] Deploy hosted version for public testing.
- [ ] Complete Soundness Layer integration.

## Innovation
Most random quote apps just fetch data from an API, but this vApp proves that the quote truly comes from an official dataset using Soundness Layer.
This ensures transparency, prevents manipulation, and makes it trustless.

## Contact
Preferred Contact: Discord → 0x123van
Updates: Project updates will be shared via GitHub and Discord.

