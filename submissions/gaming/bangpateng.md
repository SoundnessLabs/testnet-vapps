# vApp Submission: [SoundDolphins]

## Verification
```yaml
github_username: "@bangpateng"
discord_id: "950283298171658252"
timestamp: "2025-08-21"
```

## Developer
- **Name**: Bang Pateng
- **GitHub**: @bangpateng
- **Discord**: mukidihajar#8675
- **Experience**: Brief background

## Project

### Name & Category
- **Project**: SoundDolphins
- **Category**: gaming/other

### Description
Sound Dolphins Is A Place To Create Images/Memes That Contain Many Dolphins With Many Attributes That Will Make Them Unique

### SL Integration  
Technical Architecture and Soundness Layer (SL) Integration
The SoundDolphins project will leverage the Soundness Layer (SL) to ensure the authenticity and unique rarity of every dolphin image created, without publicly revealing the raw data or creation process.

**1. Image Creation Process:**

- Users select various attributes for their dolphin (e.g., hat, glasses, background, etc.).
- These attributes are combined on the user's front-end to generate the final dolphin image.

**2. SL Integration (Zero-Knowledge Proofs):**

- Before the image is stored, we will generate a Zero-Knowledge Proof (ZK-Proof) using the Soundness Layer.
- This ZK-Proof will verify two main things:

1. Attribute Uniqueness: The ZK-Proof will prove that the selected combination of attributes (e.g., hat A, glasses B, background C) is unique and has not been used before by another user to create a dolphin image.
2. Image Integrity: The ZK-Proof will also prove that the generated image was genuinely created from the claimed combination of attributes.

**3. Data Storage:**

- The completed dolphin image will be uploaded to a storage service like IPFS or Arweave.
- On the blockchain, we will only store the hash of the image and the generated ZK-Proof. We will not store the raw attribute data.

**4. Benefits of SL Integration:**

- Privacy and Uniqueness: The attribute combination doesn't need to be revealed publicly. The ZK-Proof is sufficient to prove its uniqueness, protecting user data while guaranteeing rarity.
- Instant Verification: Anyone can quickly verify the authenticity and uniqueness of the image using the ZK-Proof stored on the blockchain, without needing to see the original attribute data.
- Security: The ZK-Proof prevents others from claiming the same attribute combination, ensuring that every dolphin NFT created is an original and cannot be duplicated.

With this architecture, we ensure that every dolphin generated on SoundDolphins is unique, authentic, and verifiable by anyone, all while maintaining user data privacy.

## Technical

### Architecture
Medium-level system design and approach

### Stack
- **Frontend**: React/Vue/etc
- **Backend**: Node.js/Python/etc  
- **Blockchain**: SL + others
- **Storage**: Database/WALRUS/IPFS/etc

### Features

1. AI-Powered Customization: Users can select from a wide range of AI-generated attributes to create a unique dolphin image.
2. Verified Uniqueness with ZK-Proofs: The project uses the Soundness Layer to prove the unique rarity of each dolphin's attributes, without revealing the underlying data.
3. Meme & NFT Creation: The platform allows users to instantly mint their unique dolphin images as NFTs or download them to be shared as memes on social media.

## Timeline

### PoC (2-4 weeks)
- [ ] Basic functionality
- [x] SL integration
- [x] Simple UI

### MVP (4-8 weeks)  
- [ ] Full features
- [ ] Production ready
- [x] User testing

## Innovation

What makes this unique? Why will people use it?

SoundDolphins is unique because we blend AI-powered creativity with Zero-Knowledge Proof (ZK-Proof) technology to solve a common problem in the NFT and social media space: publicly verifiable authenticity and rarity, all while maintaining privacy.

Currently, many NFT collections claim rarity, but the raw attribute data is often publicly accessible, potentially leading to scams or unverified claims. With the Soundness Layer, we don't need to publish sensitive attribute data. We can simply publish a ZK-Proof that mathematically proves the uniqueness of each attribute combination.

People will use it for two main reasons:

- Transparent Trust: Users can confidently verify that their dolphin image is truly unique and rare, something other applications can't easily guarantee.
- Simplicity & Flexibility: We offer a user-friendly interface for creating unique images, which can be instantly minted as an NFT or simply used as a meme. This removes the complexity of blockchain technology and appeals to a broader audience—both crypto-native enthusiasts and casual social media users.

## Contact

Telegram : [https://t.me/bg_pateng](https://t.me/bg_pateng)


**Checklist before submitting:**
- [ ] All fields completed
- [ ] GitHub username matches PR author  
- [ ] SL integration explained
- [ ] Timeline is realistic
