# pSOL Platform Architecture

This document summarizes how the pSOL privacy stack fits together across on-chain programs, client integrations, and operational tooling. For visual call flows, see the diagrams in [docs/diagrams](docs/diagrams).

## High-Level Components

- **Privacy Protocol (Layer 1):** Anchor-based Solana program (under `programs/psol`) that will manage encrypted balances and, in future iterations, a zero-knowledge proof system for private transfers.
- **pSOL Token (Layer 2):** SPL token program (under `programs/psol-token`) used as the user-facing asset in wallets.
- **Token Backing Mechanism:** Logic that locks SOL, mints pSOL against a vault PDA, and maintains the 1:1 backing ratio on devnet.
- **Client Surfaces:** Wallets, CLI tools, and future SDKs that handle swaps, transfers, and later proof generation/verification.
- **Deployment & Ops:** Scripts in `scripts/` for building and deploying programs to Solana clusters (currently devnet).

## Data Flows

### 1. Minting & Backing (SOL → pSOL)

1. User requests a swap from SOL to pSOL.
2. SOL is locked into a vault PDA controlled by the program.
3. pSOL is minted at a 1:1 rate to the user's token account.
4. In future iterations, a linked privacy account will track the encrypted balance.

### 2. Private Transfers (Roadmap)

1. User generates a ZK proof off-chain for a transfer.
2. The proof and minimal metadata are submitted with a transfer instruction.
3. The privacy program updates encrypted balances while preserving confidentiality.

### 3. Redeeming (pSOL → SOL)

1. User burns pSOL via the program.
2. The program releases SOL from the vault PDA to the user's SOL account.

## Deployments

- **Network:** Devnet  
- **Privacy Protocol Program ID:** `2dJdyxoGmAoJLsZh7h8ma8xeyoaj7uiHFgrsgUAQMojv`  
- **pSOL Token Program ID:** `CgERkyXGARsLFqgBHyTk4Njyt7nyq9foJfmV7ptadYLy`

## Extensibility

- Future TypeScript and Rust SDKs for wallets and dApps.
- Modular privacy building blocks for developers once ZK and encrypted balances are integrated.
- Strong separation between asset layer (pSOL token) and privacy logic (protocol layer).
