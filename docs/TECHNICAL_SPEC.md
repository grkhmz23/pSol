# Technical Specification

## Tooling
- Anchor 0.32.x
- Solana SDK matching Anchor 0.32
- Rust 2021 edition

## Programs
### psol
- **Program ID:** 2dJdyxoGmAoJLsZh7h8ma8xeyoaj7uiHFgrsgUAQMojv
- **Vault PDA:** `seeds=["vault", pool]`
- **Commitment Registry PDA:** `seeds=["commitment", pool]`
- **Nullifier Registry PDA:** `seeds=["nullifier", pool]`
- **Privacy Account PDA:** `seeds=["privacy", owner]`

State sizes include the 8-byte account discriminator.

### Data structures
- `PrivacyPool { admin: Pubkey, vault_bump: u8, commitment_bump: u8, nullifier_bump: u8, paused: bool, fee_bps: u16, total_locked: u64 }`
- `CommitmentRegistry { pool: Pubkey, count: u64, commitments: [[u8;32]; MAX_COMMITMENTS] }` with `MAX_COMMITMENTS = 64`
- `NullifierRegistry { pool: Pubkey, count: u64, nullifiers: [[u8;32]; MAX_NULLIFIERS] }` with `MAX_NULLIFIERS = 128`
- `PrivacyAccount { owner: Pubkey, balance: u64 }`

### Instruction behavior
- **initialize_pool**: validates `fee_bps <= 10_000`, creates vault PDA with rent-exempt lamports, seeds registries, writes bumps.
- **init_privacy_account**: initializes PDA for the signer with zero balance.
- **deposit_private(amount, nonce)**: checks pause flag, transfers SOL to vault, applies fee, credits balance, updates `total_locked`, and records a commitment `hash(owner || amount || nonce)`.
- **transfer_private(amount, nullifier, nonce)**: requires unique `nullifier`, debits sender, credits receiver, and records recipient commitment.
- **withdraw_private(amount, nullifier)**: validates pause flag, requires unique nullifier, debits balance, applies fee, reduces `total_locked`, and transfers net SOL from the vault PDA to recipient.
- **admin_set_fees(fee_bps)**: admin-only; caps at 10_000 bps.
- **admin_pause / admin_unpause**: toggles pool availability.

### Cryptography
Commitments and nullifiers are derived from Solana hashes (see `crypto.rs`). They are deterministic placeholders and must be replaced before production deployment.

### psol_token
- **Program ID:** CgERkyXGARsLFqgBHyTk4Njyt7nyq9foJfmV7ptadYLy
- **Config PDA:** `seeds=["psol_config"]`
- **Mint authority PDA:** `seeds=["psol_mint_auth"]`

State: `Config { admin, psol_program, pool, psol_mint, mint_authority_bump, bump }`

Instructions:
- **initialize_token**: writes config, asserts program/pool, and sets the mint authority to the PDA if needed.
- **swap_to_psol(amount, nonce)**: CPI `deposit_private`, then mints pSOL equal to the net amount after pool fees.
- **swap_to_sol(amount, nullifier)**: burns user pSOL, then CPIs `withdraw_private` to release SOL.
- **transfer_psol**: always returns `TransfersDisabled`.

## Safety considerations
- Registries are fixed-size; hitting capacity halts new commitments/nullifiers until upgraded.
- Fees are bounded to avoid overflow and are applied symmetrically on deposit/withdrawal.
- All critical accounts are PDAs with explicit seeds to prevent spoofing.