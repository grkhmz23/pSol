# Security

## Status

This is experimental software in active development.

**Not production-ready. Not audited. Not suitable for significant funds.**

## Known Limitations

### Cryptographic Implementation

Current implementation uses placeholder algorithms:

- Zero-knowledge proofs are simplified (require full Bulletproofs or Groth16 implementation)
- ElGamal encryption uses basic operations (requires production curve arithmetic)
- Pedersen commitments are incomplete (need full elliptic curve implementation)

### Required for Production

1. Complete ZK-SNARK implementation with proper circuits
2. Production-grade elliptic curve operations
3. Professional security audit by reputable firm
4. Extensive testing (10,000+ transactions)
5. Bug bounty program
6. Formal verification

## Reporting Vulnerabilities

If you discover a security issue:

1. Do not open a public issue
2. Email: security@psol.finance (example - update with your contact)
3. Include detailed description and reproduction steps
4. Allow time for fix before public disclosure

## Audit Status

Not audited.

Audit required before mainnet deployment.

## Disclaimer

Use at your own risk. This software is provided as-is without warranties.
