# pSOL - Privacy Protocol for Solana

[![Solana](https://img.shields.io/badge/Solana-Devnet-purple)](https://explorer.solana.com/address/2dJdyxoGmAoJLsZh7h8ma8xeyoaj7uiHFgrsgUAQMojv?cluster=devnet)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Anchor](https://img.shields.io/badge/Anchor-0.30.1-blue)](https://www.anchor-lang.com/)

Privacy-preserving SOL transactions on Solana using encrypted balances and zero-knowledge proofs.

[View on Solana Explorer](https://explorer.solana.com/address/2dJdyxoGmAoJLsZh7h8ma8xeyoaj7uiHFgrsgUAQMojv?cluster=devnet) | [Documentation](#documentation) | [Deployment Guide](./docs/DEPLOYMENT.md)

---

## Overview

pSOL is a privacy protocol built on Solana that enables confidential transactions through on-chain encrypted balances. The protocol allows users to deposit SOL into a privacy pool where balances are stored as encrypted ciphertext, making transaction amounts invisible to third-party observers.

### Key Features

- **Encrypted Balances**: User balances stored as ElGamal ciphertext on-chain
- **Privacy Pool**: Collective pool for deposits and withdrawals with mixing
- **Zero-Knowledge Proofs**: Cryptographic proofs for balance verification (v2)
- **Low Fees**: 0.1% protocol fee on deposits and withdrawals
- **Solana Performance**: Sub-second finality with minimal transaction costs

---

## Technical Specification

### Program Information

| Parameter | Value |
|-----------|-------|
| Program ID | `2dJdyxoGmAoJLsZh7h8ma8xeyoaj7uiHFgrsgUAQMojv` |
| Network | Solana Devnet |
| Framework | Anchor 0.30.1 |
| Rust Version | 1.75.0+ |
| Status | Deployed and Tested |

### Protocol Parameters

| Parameter | Value |
|-----------|-------|
| Deposit Fee | 10 bps (0.1%) |
| Withdrawal Fee | 10 bps (0.1%) |
| Minimum Deposit | 0.01 SOL |
| Maximum Deposit | No limit |

---

## Architecture

### Program Structure

```
programs/psol/src/
├── lib.rs                  # Program entry point and instruction handlers
├── state.rs                # Account structures and data models
├── error.rs                # Custom error definitions
├── crypto.rs               # Cryptographic primitives
└── instructions/
    ├── initialize.rs       # Privacy pool initialization
    ├── deposit.rs          # SOL deposit with encryption
    ├── withdraw.rs         # SOL withdrawal with proof verification
    └── transfer.rs         # Private peer-to-peer transfers
```

### Account Types

#### PrivacyPool

Global state account managing the protocol.

```rust
pub struct PrivacyPool {
    pub authority: Pubkey,          // Protocol authority
    pub vault: Pubkey,              // SOL vault PDA
    pub total_locked: u64,          // Total SOL in pool
    pub total_accounts: u64,        // Number of privacy accounts
    pub deposit_fee_bps: u16,       // Deposit fee in basis points
    pub withdraw_fee_bps: u16,      // Withdrawal fee in basis points
    pub paused: bool,               // Emergency pause flag
    pub bump: u8,                   // PDA bump seed
}
```

#### PrivacyAccount

User-specific account containing encrypted balance.

```rust
pub struct PrivacyAccount {
    pub owner: Pubkey,                      // Account owner
    pub encrypted_balance: [u8; 64],        // ElGamal ciphertext
    pub encryption_key: [u8; 32],           // Public encryption key
    pub commitment: [u8; 32],               // Pedersen commitment
    pub last_update: u64,                   // Last modification slot
    pub total_deposits: u64,                // Cumulative deposits
    pub total_withdrawals: u64,             // Cumulative withdrawals
    pub bump: u8,                           // PDA bump seed
}
```

#### NullifierSet

Nullifier tracking to prevent double-spending.

```rust
pub struct NullifierSet {
    pub pool: Pubkey,           // Associated privacy pool
    pub nullifier: [u8; 32],    // Unique nullifier hash
    pub slot: u64,              // Creation slot
    pub bump: u8,               // PDA bump seed
}
```

---

## Installation

### Prerequisites

- Solana CLI 1.18.0 or higher
- Anchor CLI 0.30.1
- Rust 1.75.0 or higher
- Node.js 18.0 or higher

### Build Instructions

```bash
git clone https://github.com/grkhmz23/pSol.git
cd pSol/psol
npm install
anchor build
```

### Testing

```bash
anchor test --skip-local-validator --provider.cluster devnet
```

### Deployment

```bash
solana config set --url devnet
anchor deploy --provider.cluster devnet
```

---

## Usage

### Initializing the Protocol

```typescript
import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";

const program = anchor.workspace.Psol as Program<Psol>;
const [poolPda] = PublicKey.findProgramAddressSync(
  [Buffer.from("privacy_pool")],
  program.programId
);

await program.methods
  .initialize()
  .accounts({
    pool: poolPda,
    vault: vaultPda,
    authority: provider.wallet.publicKey,
    systemProgram: SystemProgram.programId,
  })
  .rpc();
```

### Depositing SOL

```typescript
const depositAmount = new anchor.BN(1_000_000_000); // 1 SOL in lamports

await program.methods
  .deposit(depositAmount)
  .accounts({
    pool: poolPda,
    privacyAccount: privacyAccountPda,
    vault: vaultPda,
    user: wallet.publicKey,
    systemProgram: SystemProgram.programId,
  })
  .rpc();
```

### Querying Encrypted Balance

```typescript
const privacyAccount = await program.account.privacyAccount.fetch(
  privacyAccountPda
);

console.log("Encrypted Balance:", privacyAccount.encryptedBalance);
console.log("Total Deposits:", privacyAccount.totalDeposits.toString());
```

Note: The encrypted balance appears as an array of bytes and cannot be decoded without the private decryption key.

---

## Cryptographic Implementation

### Current Implementation (MVP)

The MVP version uses simplified cryptographic primitives suitable for demonstration purposes:

- **Encryption**: XOR-based cipher for balance encryption
- **Commitments**: Hash-based Pedersen commitments
- **Proofs**: Placeholder verification logic

### Production Implementation (Planned)

Future versions will implement industry-standard cryptography:

- **Encryption**: Twisted ElGamal encryption with homomorphic properties
- **Commitments**: Proper Pedersen commitments on elliptic curves
- **Proofs**: Bulletproofs or Groth16 for zero-knowledge verification
- **Nullifiers**: Poseidon hash-based nullifier system

---

## Security Considerations

### Current Status

This codebase represents an MVP implementation and has not undergone formal security auditing. The following limitations apply:

1. Simplified cryptographic primitives
2. Placeholder zero-knowledge proof verification
3. Basic key management without hardware wallet integration
4. No formal verification of smart contract logic

### Production Requirements

Before mainnet deployment, the following steps are required:

- Complete security audit by reputable firm (Halborn, OtterSec, or equivalent)
- Implementation of production-grade cryptography
- Formal verification of critical contract logic
- Comprehensive integration testing
- Bug bounty program
- Gradual rollout with TVL caps

**Warning**: Do not deploy to mainnet or use with real funds without proper security measures.

---

## Roadmap

### Phase 1: MVP (Completed)

- Core privacy pool infrastructure
- Basic encrypted balance storage
- Deposit and withdrawal functionality
- Test coverage and devnet deployment

### Phase 2: Enhanced Cryptography (In Progress)

- Production-grade ElGamal encryption
- Bulletproofs implementation
- Range proof integration
- Nullifier system enhancement

### Phase 3: Audit and Mainnet (Planned)

- Security audit
- Performance optimization
- Mainnet deployment
- Monitoring infrastructure

### Phase 4: Protocol Expansion (Future)

- Cross-program composability
- SPL token privacy
- Layer 2 scaling solutions
- Governance implementation

---

## Performance Metrics

### Transaction Costs (Devnet)

| Operation | Compute Units | Approximate Cost |
|-----------|---------------|------------------|
| Initialize | ~45,000 | ~0.00002 SOL |
| Deposit | ~85,000 | ~0.00004 SOL |
| Withdraw | ~95,000 | ~0.00005 SOL |
| Transfer | ~78,000 | ~0.00004 SOL |

### Scalability

- Theoretical throughput: 65,000 TPS (Solana network limit)
- Practical throughput: Limited by cryptographic operations
- Account size: 193 bytes per privacy account
- Storage efficiency: Minimal on-chain footprint

---

## API Reference

### Instructions

#### `initialize()`
Initializes the privacy pool. Can only be called once.

**Accounts:**
- `pool` (write, PDA): Privacy pool state account
- `vault` (write, PDA): SOL vault account
- `authority` (signer): Protocol authority
- `system_program`: Solana system program

#### `deposit(amount: u64)`
Deposits SOL into the privacy pool and encrypts the balance.

**Accounts:**
- `pool` (write): Privacy pool state
- `privacy_account` (write, PDA): User's privacy account
- `vault` (write): SOL vault
- `user` (signer, write): User making deposit
- `system_program`: Solana system program

**Arguments:**
- `amount`: Amount in lamports to deposit

#### `withdraw(amount: u64, nullifier: [u8; 32], proof: Vec<u8>)`
Withdraws SOL from the privacy pool with zero-knowledge proof.

**Accounts:**
- `pool` (write): Privacy pool state
- `privacy_account` (write): User's privacy account
- `nullifier_account` (write, PDA): Nullifier tracking account
- `vault` (write): SOL vault
- `recipient` (write): Withdrawal recipient
- `owner` (signer, write): Account owner
- `system_program`: Solana system program

**Arguments:**
- `amount`: Amount in lamports to withdraw
- `nullifier`: Unique nullifier hash
- `proof`: Zero-knowledge proof bytes

#### `transfer(encrypted_amount: [u8; 64], proof: Vec<u8>)`
Transfers between privacy accounts without revealing amounts.

**Accounts:**
- `pool`: Privacy pool state
- `sender_account` (write): Sender's privacy account
- `recipient_account` (write): Recipient's privacy account
- `sender` (signer): Transaction sender
- `recipient`: Transfer recipient

**Arguments:**
- `encrypted_amount`: Encrypted transfer amount
- `proof`: Zero-knowledge proof bytes

---

## Development

### Running Tests

```bash
# Run all tests
anchor test --skip-local-validator --provider.cluster devnet

# Run specific test file
anchor test tests/psol.ts --skip-local-validator --provider.cluster devnet

# Generate test coverage
cargo tarpaulin --out Html
```

### Code Style

This project follows Rust and TypeScript standard formatting:

```bash
# Format Rust code
cargo fmt

# Lint Rust code
cargo clippy

# Format TypeScript
npm run format

# Lint TypeScript
npm run lint
```

### Contributing

Contributions are welcome. Please follow these guidelines:

1. Fork the repository
2. Create a feature branch
3. Make changes with tests
4. Submit pull request with detailed description
5. Ensure CI passes

---

## Documentation

- [Technical Specification](./docs/TECHNICAL_SPEC.md)
- [Deployment Guide](./docs/DEPLOYMENT.md)
- [API Documentation](./docs/API.md)
- [Architecture Overview](./docs/ARCHITECTURE.md)

---

## License

This project is licensed under the MIT License. See [LICENSE](./LICENSE) file for details.

---

## Acknowledgments

Built using:
- [Anchor Framework](https://www.anchor-lang.com/) - Solana development framework
- [Solana Labs](https://solana.com/) - Blockchain infrastructure

Cryptographic research references:
- Bulletproofs: Benedikt Bünz et al.
- ElGamal Encryption: Taher Elgamal
- Pedersen Commitments: Torben Pryds Pedersen

---

## Disclaimer

This software is provided "as is" without warranty of any kind, express or implied. The software is currently deployed on Solana devnet for testing and development purposes only. Users should not deploy to mainnet or use with production funds without conducting proper security audits and risk assessment.

The cryptographic implementations in the current version are simplified for MVP purposes and do not represent production-ready privacy guarantees. Formal security analysis and cryptographic review are required before any production deployment.

---

## Contact

- **Repository**: https://github.com/grkhmz23/pSol
- **Telegram**: [@unc_gorkh](https://t.me/unc_gorkh)
- **Website**: Coming Soon
- **Twitter/X**: Coming Soon
- **Issues**: https://github.com/grkhmz23/pSol/issues
- **Explorer**: https://explorer.solana.com/address/2dJdyxoGmAoJLsZh7h8ma8xeyoaj7uiHFgrsgUAQMojv?cluster=devnet
