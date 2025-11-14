# pSOL Token Architecture

## System Overview

The pSOL ecosystem consists of two integrated layers:

**Layer 1: Privacy Protocol** - Core privacy infrastructure  
**Layer 2: pSOL Token** - User-facing privacy token

## Architecture
```
┌─────────────────────────────────────────────────────────────┐
│                     pSOL ECOSYSTEM                          │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  LAYER 2: pSOL TOKEN (User Product)                        │
│  ┌───────────────────────────────────────────────────┐    │
│  │  pSOL SPL Token                                    │    │
│  │  - Token Mint: pSOL                                │    │
│  │  - 1:1 backed by SOL                               │    │
│  │  - Shows in wallet                                 │    │
│  │  - Encrypted amounts via privacy layer            │    │
│  └───────────────────────────────────────────────────┘    │
│         ↕ Integration                                      │
│  ┌───────────────────────────────────────────────────┐    │
│  │  Token Bridge                                      │    │
│  │  - Swap: SOL ↔ pSOL                               │    │
│  │  - Vault management                                │    │
│  │  - Links token to privacy account                  │    │
│  └───────────────────────────────────────────────────┘    │
│         ↕                                                  │
│  LAYER 1: PRIVACY PROTOCOL (Foundation)                    │
│  ┌───────────────────────────────────────────────────┐    │
│  │  Privacy Pool                                      │    │
│  │  - Encrypted balances                              │    │
│  │  - Privacy accounts                                │    │
│  │  - Private transfers                               │    │
│  │  - ZK proof system                                 │    │
│  └───────────────────────────────────────────────────┘    │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

## Program IDs

**Privacy Protocol:** `2dJdyxoGmAoJLsZh7h8ma8xeyoaj7uiHFgrsgUAQMojv`  
**Token Program:** `CgERkyXGARsLFqgBHyTk4Njyt7nyq9foJfmV7ptadYLy`  
**Network:** Devnet

## Key Features

### Privacy Protocol
- Encrypted on-chain balances
- Private peer-to-peer transfers
- Zero-knowledge proof system
- Composable privacy primitives

### pSOL Token
- SPL token standard
- 1:1 backing with SOL
- Swap functionality (SOL ↔ pSOL)
- Private transfer amounts
- Shows in standard wallets

## Technical Stack

- **Framework:** Anchor 0.30.1
- **Language:** Rust
- **Blockchain:** Solana
- **Token Standard:** SPL Token
- **Cryptography:** ElGamal encryption, ZK-SNARKs

## State Accounts

### TokenVault
- Manages pSOL mint and SOL backing
- Tracks total supply and locked SOL
- Configurable swap fees (0.1% default)
- Emergency pause mechanism

### TokenPrivacyLink
- Links token account to privacy account
- Syncs encrypted balances
- Tracks last update timestamp

## Instructions

1. **initialize_token** - Set up token system
2. **swap_to_psol** - Convert SOL to pSOL
3. **swap_to_sol** - Convert pSOL to SOL
4. **transfer_psol** - Private token transfer

## Security Considerations

- All swaps use atomic transactions
- Token amounts encrypted via privacy layer
- Vault secured with PDA authority
- Fee collection for sustainability
- Emergency pause functionality

## Future Enhancements

- Production-grade ZK proofs (Groth16/Bulletproofs)
- Cross-chain bridges
- DeFi protocol integrations
- Mobile SDK
- Hardware wallet support

For detailed implementation, see source code in `programs/psol-token/`
