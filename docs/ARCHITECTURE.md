# Architecture

The repository is a single Anchor workspace hosting two on-chain programs:

- `psol`: privacy pool and vault controller.
- `psol_token`: non-transferable pSOL token bridge that mints and burns against the pool.

## Accounts

### psol
- **PrivacyPool** – stores admin, fee basis points, pause flag, PDA bumps, and tracked `total_locked` SOL held in the vault PDA.
- **CommitmentRegistry** – fixed-size list of recent commitments for auditability and placeholder privacy tracking.
- **NullifierRegistry** – fixed-size list of used nullifiers to enforce one-time spends.
- **PrivacyAccount** – PDA per user storing their private balance.
- **Vault PDA** – system account derived with seeds `["vault", pool]` that actually custodies SOL.

### psol_token
- **Config** – ties the pSOL mint, the psol program ID, and the target pool together along with the PDA mint authority bump.
- **Mint authority PDA** – derived from seed `"psol_mint_auth"`; set as the mint authority so only the program can mint/burn.

## Instruction set

### psol
- `initialize_pool(fee_bps)` – creates pool, vault PDA, commitment and nullifier registries.
- `init_privacy_account()` – creates a user privacy account PDA.
- `deposit_private(amount, nonce)` – transfers SOL into the vault, credits the privacy balance, and records a commitment.
- `transfer_private(amount, nullifier, nonce)` – moves balance between privacy accounts while enforcing nullifier uniqueness.
- `withdraw_private(amount, nullifier)` – burns privacy balance, checks nullifier, and releases SOL from the vault.
- `admin_set_fees(fee_bps)` – updates fee schedule.
- `admin_pause` / `admin_unpause` – emergency stop controls.

### psol_token
- `initialize_token` – writes config and ensures the mint authority PDA controls the pSOL mint.
- `swap_to_psol(amount, nonce)` – CPI into `deposit_private` then mints pSOL (net of fees) to the caller.
- `swap_to_sol(amount, nullifier)` – burns pSOL then CPIs into `withdraw_private` to release SOL.
- `transfer_psol` – always fails; direct transfers are disabled by design.

## Privacy model

Current crypto helpers are deterministic Solana hashes that provide a stable interface but **not** production privacy. A future upgrade should replace them with audited ZK/CT primitives while retaining the program API.
