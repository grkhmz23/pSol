use anchor_lang::prelude::*;
use anchor_lang::solana_program::hash::hashv;

/// Placeholder deterministic helpers for commitments and nullifiers.
/// These are **not** production-grade ZK primitives but give a stable
/// interface for future upgrades.
pub fn commitment(owner: &Pubkey, amount: u64, nonce: u64) -> [u8; 32] {
    let hash = hashv(&[owner.as_ref(), &amount.to_le_bytes(), &nonce.to_le_bytes()]);
    hash.to_bytes()
}

pub fn nullifier(owner: &Pubkey, commitment: &[u8; 32]) -> [u8; 32] {
    let hash = hashv(&[owner.as_ref(), commitment]);
    hash.to_bytes()
}

pub fn encrypt_placeholder(data: &[u8]) -> Vec<u8> {
    data.iter().map(|b| b ^ 0xAA).collect()
}

pub fn decrypt_placeholder(data: &[u8]) -> Vec<u8> {
    encrypt_placeholder(data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn commitment_is_deterministic() {
        let owner = Pubkey::new_unique();
        let a = commitment(&owner, 42, 1);
        let b = commitment(&owner, 42, 1);
        assert_eq!(a, b);
        assert_ne!(a, commitment(&owner, 43, 1));
    }
}