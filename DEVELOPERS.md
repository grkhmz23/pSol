# Developer Guide – pSol Protocol

This document provides technical guidelines for contributors working on the pSol Protocol.

## 1. Requirements
- Rust (latest stable)
- Solana CLI (latest stable)
- Anchor Framework
- Node.js + pnpm
- GitHub Codespaces or local dev environment

## 2. Repository Structure
programs/
  psol/
  psol-token/
tests/
docs/
scripts/

## 3. Development Workflow
1. git checkout -b feature/<component>
2. Implement your module
3. anchor test
4. cargo fmt --all
5. cargo clippy --all -- -D warnings
6. Open PR

## 4. Code Standards
- Rust 2021 edition
- No warnings
- Rustdoc required
- Tests required

## 5. Commit Convention
feat:
fix:
docs:
chore:
refactor:

## 6. Security
- No unsafe Rust unless justified
- All cryptographic primitives must be reviewed
- Avoid external dependencies without approval

## 7. Testing
- Unit tests
- Integration tests
- ZK proof tests
- Fuzz tests where necessary

