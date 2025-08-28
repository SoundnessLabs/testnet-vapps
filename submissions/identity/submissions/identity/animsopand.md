# vApp Proposal: Identity Verification via IP Tracking

## Category
identity

## Project Name
IP-Based Identity Checker

## Description
A vApp designed to verify user authenticity by detecting potential multi-account behavior based on **IP addresses**.  
The main goal is to prevent abuse (e.g., farming incentives with multiple fake accounts) and ensure fairness within the ecosystem.

## Motivation
Many users attempt to create multiple accounts (multi-accounting) to gain unfair advantages.  
With IP-based verification, the system can:  
- Identify if a single IP is used by multiple accounts.  
- Prevent fraudulent activities such as spam registrations.  
- Improve overall system reputation and keep the community fair.

## Features
- **IP Address Logging**: every time a user registers/logs in, their IP address is logged.  
- **Multi-Account Detection**: if more than *N* accounts are linked to the same IP, the system raises a flag.  
- **Monitoring Dashboard**: admins can monitor IPs and associated users.  
- **Privacy Aware**: IPs are only used internally for verification and never exposed publicly.  

## Technical Implementation
- Backend service records the user’s IP address during interactions.  
- IP data is encrypted to ensure user privacy.  
- A rule-based system detects suspicious patterns (e.g., >3 accounts on the same IP within a short time).  
- Integration with smart contracts: users flagged as “suspicious multi-account” may face limited access.  

## Benefits
- Reduces abuse of incentives (sybil attacks / multi-accounting).  
- Rewards genuine users fairly.  
- Supports Soundness Layer in maintaining ecosystem fairness.

## Your GitHub Username
animsopand
