use anchor_lang::prelude::*;

use crate::error::ErrorCode;

pub const MAX_NULLIFIERS: usize = 128;

#[account]
pub struct NullifierRegistry {
    pub pool: Pubkey,
    pub count: u64,
    pub nullifiers: [[u8; 32]; MAX_NULLIFIERS],
}

impl NullifierRegistry {
    pub const SPACE: usize = 8 + 32 + 8 + (MAX_NULLIFIERS * 32);

    pub fn register(&mut self, pool: &Pubkey, nullifier: [u8; 32]) -> Result<()> {
        require_keys_eq!(self.pool, *pool, ErrorCode::InvalidRegistry);
        if self.count as usize >= MAX_NULLIFIERS {
            return err!(ErrorCode::CommitmentRegistryFull);
        }
        for existing in &self.nullifiers[..self.count as usize] {
            if existing == &nullifier {
                return err!(ErrorCode::NullifierAlreadyUsed);
            }
        }
        self.nullifiers[self.count as usize] = nullifier;
        self.count += 1;
        Ok(())
    }
}