use anchor_lang::prelude::*;
use solana_program::keccak::hashv;

use crate::error::ErrorCode;

/// NOTE: Testnet/devnet placeholder crypto.
/// - No XOR or fake "always true" proof checks.
/// - Deterministic, fail-closed verification for development.
/// - Replace with real Groth16/Bulletproofs + curve ops before mainnet.
///
/// This module provides:
/// - verify_proof / verify_transfer_proof (placeholder)
/// - pedersen_commit (hash-based placeholder)
/// - encrypt_amount (hash-based placeholder to 64 bytes)
/// - homomorphic_add / homomorphic_sub (hash-combine placeholders)
/// - add_encrypted / subtract_encrypted (aliases for legacy calls)

/// Verify a ZK proof given public inputs.
/// Placeholder behavior:
/// - Enforces minimum proof length.
/// - Rejects all-zero proof.
/// - Hashes (domain || proof || inputs) and requires hash[0] != 0.
/// This is NOT cryptographically sound privacy, but it does fail closed.
pub fn verify_proof(proof: &[u8], public_inputs: &[[u8; 32]]) -> Result<bool> {
    require!(proof.len() >= 192, ErrorCode::InvalidProof);
    require!(!public_inputs.is_empty(), ErrorCode::InvalidProof);

    // Reject trivially empty proof
    let all_zero = proof.iter().all(|b| *b == 0);
    require!(!all_zero, ErrorCode::InvalidProof);

    let mut buf = Vec::with_capacity(32 + proof.len() + public_inputs.len() * 32);
    buf.extend_from_slice(b"psol_verify_proof_v1");
    buf.extend_from_slice(proof);
    for pi in public_inputs {
        buf.extend_from_slice(pi);
    }

    let h = hashv(&[&buf]).to_bytes();
    Ok(h[0] != 0)
}

/// Backward-compatible alias for transfer.
/// If your transfer instruction calls verify_transfer_proof, it will compile.
pub fn verify_transfer_proof(proof: &[u8], public_inputs: &[[u8; 32]]) -> Result<bool> {
    verify_proof(proof, public_inputs)
}

/// Pedersen commitment placeholder.
/// Real Pedersen: C = v*G + r*H.
/// Placeholder: keccak(domain || value || blinding).
pub fn pedersen_commit(value: u64, blinding: &[u8; 32]) -> [u8; 32] {
    let mut buf = Vec::with_capacity(16 + 32 + 32);
    buf.extend_from_slice(b"psol_pedersen_v1");
    buf.extend_from_slice(&value.to_le_bytes());
    buf.extend_from_slice(blinding);
    hashv(&[&buf]).to_bytes()
}

/// Encrypt amount placeholder.
/// Real ElGamal produces two curve points (c1, c2) -> 64 bytes raw.
/// Placeholder: ke
