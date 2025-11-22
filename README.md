# pSOL Protocol

pSOL is a two-program privacy stack for Solana built with Anchor:

- **psol** – privacy pool that custodially holds SOL in a vault PDA while tracking private balances, commitments, and nullifiers.
- **psol_token** – a custom, non-transferable pSOL token that is only minted/burned through the privacy pool bridge flows.

Direct pSOL transfers are intentionally disabled; value moves either privately inside the pool or through burn/mint swaps coordinated by the token program.

## Getting started

### Prerequisites
- Rust stable toolchain
- Anchor CLI 0.32.x
- Solana CLI compatible with Anchor 0.32

### Build and test
```bash
anchor build
cargo test --workspace
```

### Localnet workflow
```bash
# Start a local validator in another shell
solana-test-validator

# In this repo
anchor deploy
```

## Programs

| Program | ID | Purpose |
| --- | --- | --- |
| psol | `2dJdyxoGmAoJLsZh7h8ma8xeyoaj7uiHFgrsgUAQMojv` | Privacy pool, vault management, commitments, nullifiers |
| psol_token | `CgERkyXGARsLFqgBHyTk4Njyt7nyq9foJfmV7ptadYLy` | Non-transferable pSOL mint/burn bridge |

### Core flows
1. **initialize_pool** (psol): creates the pool, vault PDA, and registries.
2. **initialize_token** (psol_token): configures the pSOL mint authority and binds it to the pool and psol program.
3. **init_privacy_account** (psol): user initializes their privacy account PDA.
4. **swap_to_psol** (psol_token): CPI into `deposit_private`, then mints pSOL to the user.
5. **transfer_private** (psol): move balances between privacy accounts using commitments/nullifiers.
6. **swap_to_sol** (psol_token): burn pSOL and CPI into `withdraw_private` to release SOL.

### Notes on cryptography
The current commitment and nullifier helpers are deterministic placeholders built on Solana hashes. They are **not** production-grade ZK primitives and should be replaced with audited confidential transaction logic in a future version.

## Repository layout
```
Anchor.toml
Cargo.toml
programs/
  psol/
  psol-token/
tests/
docs/
```

## Contact
- GitHub: https://github.com/grkhmz23/pSol


Telegram: @unc_gorkh  
Website: [pSolProtocol](https://psolprotocol.org/)

---

