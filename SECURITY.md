# Security Policy

We take the confidentiality of user balances and the integrity of token backing seriously.

## Supported Versions

Security fixes target the latest `main` branch and active deployment configurations.

## Reporting a Vulnerability

Contact:
- Telegram: @unc_gorkh
- GitHub: grkhmz23 (open a private issue if needed)

Include:
- Program IDs
- Transaction signatures
- Reproduction steps

## Handling Guidelines

- Do not publicly disclose vulnerabilities before a coordinated release.
- Prefer devnet or testnet for testing.
- Mainnet-beta probing must be pre-approved.
- For severe issues, coordinated disclosure may be required.

## Scope

- On-chain programs under `programs/`
- Deployment scripts under `scripts/`
- Documentation that could leak operational details

## Out of Scope

- Third-party dependency issues unless directly exploitable inside pSOL
- Social engineering or physical security attacks
