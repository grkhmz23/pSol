# pSOL

Privacy protocol for native SOL on Solana.

## Overview

pSOL enables confidential transactions on Solana by wrapping native SOL with cryptographic privacy guarantees. Users can wrap SOL into an encrypted balance, transfer privately, and unwrap back to SOL.

## Architecture

### Core Components

- **PrivacyPool**: Global state managing vault and protocol parameters
- **PrivacyAccount**: User accounts with encrypted balances
- **NullifierSet**: Prevents double-spending
- **SOL Vault**: Holds all deposited SOL (1:1 backing)

### Instructions

- `initialize`: Create privacy pool
- `deposit`: Wrap SOL → pSOL (0.1% fee)
- `withdraw`: Unwrap pSOL → SOL (0.1% fee, requires ZK proof)
- `transfer`: Private transfer within pool (no fee)

## Build

```bash
anchor build
```

## Test

```bash
anchor test
```

## Deploy

```bash
# Devnet
solana config set --url devnet
anchor deploy

# Mainnet (not recommended yet - see Security)
solana config set --url mainnet-beta
anchor deploy
```

## Security

**Status**: Development  
**Audit**: Not audited  
**Production**: Not ready

Current implementation uses placeholder cryptography. Production deployment requires:

- Full zero-knowledge proof system (Bulletproofs or Groth16)
- Production-grade elliptic curve operations
- Professional security audit
- Extensive testing

Do not use with significant funds.

## Technical Details

### Cryptography (Current)

- ElGamal encryption (simplified)
- Pedersen commitments (basic)
- Zero-knowledge proofs (placeholder)
- Homomorphic operations (partial)

### Economics

- 1 pSOL = 1 SOL (always)
- Deposit fee: 10 basis points (0.1%)
- Withdrawal fee: 10 basis points (0.1%)
- Transfer fee: 0

### Account Structure

```rust
PrivacyPool {
    authority: Pubkey,
    vault: Pubkey,
    total_locked: u64,
    total_accounts: u64,
    deposit_fee_bps: u16,
    withdraw_fee_bps: u16,
    paused: bool,
}

PrivacyAccount {
    owner: Pubkey,
    encrypted_balance: [u8; 64],
    encryption_key: [u8; 32],
    commitment: [u8; 32],
    last_update: u64,
    total_deposits: u64,
    total_withdrawals: u64,
}

NullifierSet {
    pool: Pubkey,
    nullifier: [u8; 32],
    slot: u64,
}
```

## Program Address

- **Devnet**: `PSoL1111111111111111111111111111111111111111`
- **Mainnet**: TBD

## Dependencies

- Rust 1.75.0+
- Solana 1.18.22
- Anchor 0.30.1
- Node.js 18+

## License

MIT
