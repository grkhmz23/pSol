# Deployment Guide

## Prerequisites
- Anchor CLI 0.32.x installed and configured
- Solana CLI with keypair at `~/.config/solana/id.json`
- Local validator (for localnet) or access to devnet/mainnet RPC

## Build
```bash
anchor build
```

## Deploy to localnet
```bash
solana-test-validator --reset
anchor deploy
```

## Deploy to devnet/mainnet
Set your provider cluster in `Anchor.toml` or via env vars, then run:
```bash
solana config set --url https://api.devnet.solana.com
anchor deploy
```

## Initialization sequence
1. **initialize_pool** (program: `psol`)
   - Accounts: new `PrivacyPool`, `CommitmentRegistry` PDA, `NullifierRegistry` PDA, vault PDA, admin signer.
   - Input: `fee_bps` (max 10_000).
2. **initialize_token** (program: `psol_token`)
   - Accounts: Config PDA (`psol_config`), pSOL mint, mint authority PDA (`psol_mint_auth`), pool, psol program.
   - Ensures mint authority is set to the PDA.
3. For each user: **init_privacy_account** (program: `psol`) with seeds `["privacy", user]`.

## Swap flows
- **Deposit SOL → pSOL**: call `swap_to_psol` on `psol_token` with amount + nonce. CPI performs deposit into the pool then mints pSOL to the caller’s token account.
- **pSOL → SOL**: call `swap_to_sol` on `psol_token` with amount + nullifier. Burns pSOL then CPIs withdraw to release SOL to the recipient.

## Verification
- Program IDs are pinned in `Anchor.toml` and code via `declare_id!`:
  - psol: `2dJdyxoGmAoJLsZh7h8ma8xeyoaj7uiHFgrsgUAQMojv`
  - psol_token: `CgERkyXGARsLFqgBHyTk4Njyt7nyq9foJfmV7ptadYLy`

## Frontend hooks
- Derive vault PDA: `Pubkey::find_program_address(&[b"vault", pool_pubkey.as_ref()], &psol_program_id)`
- Derive mint authority PDA: `Pubkey::find_program_address(&[b"psol_mint_auth"], &psol_token_program_id)`
- Use the instruction set described in ARCHITECTURE.md to orchestrate deposits, private transfers, and withdrawals.
