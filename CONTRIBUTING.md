# Contributing to pSOL

We welcome fixes, features, and documentation updates that strengthen privacy guarantees and developer ergonomics.

## Ground Rules

- Discuss substantial changes via an issue before opening a PR.
- Keep changes small and focused; include tests demonstrating correctness.
- Follow the coding standards in `DEVELOPERS.md`.
- Update diagrams under `docs/diagrams` when flows change.
- Avoid adding new dependencies unless essential and well-justified.

## Workflow

1. Create a branch from `main`.
2. Implement the change with clear, descriptive commit messages.
3. Run `anchor test` and ensure all tests pass.
4. Update documentation where relevant (README, ARCHITECTURE, diagrams).
5. Open a PR describing the motivation, approach, and testing performed.

## Code Quality

- Validate all accounts explicitly and return meaningful custom errors.
- Avoid `panic!`; use explicit error handling.
- Keep PDA seeds deterministic and document them.
- Document any cryptographic assumptions and security-relevant logic.

## Communication

- Security-sensitive findings must follow the process in `SECURITY.md`.
- Use GitHub issues for roadmap, refactors, and architectural discussions.
