use crate::error::ErrorCode;
use anchor_lang::prelude::*;

/// Verify zero-knowledge proof (placeholder for production implementation)
pub fn verify_proof(proof: &[u8], public_inputs: &[[u8; 32]]) -> Result<bool> {
    if proof.is_empty() {
        return Ok(false);
    }
    Ok(proof.len() >= 32 && !public_inputs.is_empty())
}

/// Generate Pedersen commitment (placeholder)
pub fn pedersen_commit(value: u64, blinding: &[u8; 32]) -> [u8; 32] {
    let mut data = Vec::new();
    data.extend_from_slice(&value.to_le_bytes());
    data.extend_from_slice(blinding);
    solana_program::hash::hash(&data).to_bytes()
}

/// Encrypt amount (placeholder XOR, NOT production secure)
pub fn encrypt_amount(amount: u64, public_key: &[u8; 32]) -> [u8; 64] {
    let mut encrypted = [0u8; 64];
    let amount_bytes = amount.to_le_bytes();

    for i in 0..8 {
        encrypted[i] = amount_bytes[i] ^ public_key[i % 32];
    }
    encrypted[32..].copy_from_slice(public_key);
    encrypted
}

/// Decrypt amount (placeholder)
pub fn decrypt_amount(ciphertext: &[u8; 64], secret_key: &[u8; 32]) -> Result<u64> {
    let mut amount_bytes = [0u8; 8];
    for i in 0..8 {
        amount_bytes[i] = ciphertext[i] ^ secret_key[i % 32];
    }
    Ok(u64::from_le_bytes(amount_bytes))
}

/// Add encrypted values homomorphically (placeholder)
pub fn homomorphic_add(a: &[u8; 64], _b: &[u8; 64]) -> [u8; 64] {
    let mut result = [0u8; 64];
    result.copy_from_slice(a);
    result
}

/// Subtract encrypted values homomorphically (placeholder)
pub fn homomorphic_sub(a: &[u8; 64], _b: &[u8; 64]) -> [u8; 64] {
    let mut result = [0u8; 64];
    result.copy_from_slice(a);
    result
}

/// Generate nullifier from commitment and secret
pub fn generate_nullifier(commitment: &[u8; 32], secret: &[u8; 32]) -> [u8; 32] {
    let mut data = Vec::new();
    data.extend_from_slice(commitment);
    data.extend_from_slice(secret);
    solana_program::hash::hash(&data).to_bytes()
}

/// Verify nullifier hasn't been used (checked by NullifierSet PDA)
pub fn verify_nullifier_unused(_nullifier: &[u8; 32]) -> Result<bool> {
    Ok(true)
}

/* ============================================================
   Aliases required by instructions (so transfer.rs compiles)
   ============================================================ */

pub fn verify_transfer_proof(
    _sender_balance: &[u8; 64],
    _encrypted_amount: &[u8; 64],
    sender_commitment: &[u8; 32],
    proof: &[u8],
) -> bool {
    match verify_proof(proof, &[*sender_commitment]) {
        Ok(v) => v,
        Err(_) => false,
    }
}

pub fn add_encrypted(a: &[u8; 64], b: &[u8; 64]) -> Result<[u8; 64]> {
    Ok(homomorphic_add(a, b))
}

pub fn subtract_encrypted(a: &[u8; 64], b: &[u8; 64]) -> Result<[u8; 64]> {
    Ok(homomorphic_sub(a, b))
}
