use anchor_lang::prelude::*;

use crate::error::ErrorCode;

pub const MAX_COMMITMENTS: usize = 64;

#[account]
pub struct CommitmentRegistry {
    pub pool: Pubkey,
    pub count: u64,
    pub commitments: [[u8; 32]; MAX_COMMITMENTS],
}

impl CommitmentRegistry {
    pub const SPACE: usize = 8 + 32 + 8 + (MAX_COMMITMENTS * 32);

    pub fn add_commitment(&mut self, pool: &Pubkey, commitment: [u8; 32]) -> Result<()> {
        require_keys_eq!(self.pool, *pool, ErrorCode::InvalidRegistry);
        if self.count as usize >= MAX_COMMITMENTS {
            return err!(ErrorCode::CommitmentRegistryFull);
        }
        self.commitments[self.count as usize] = commitment;
        self.count += 1;
        Ok(())
    }
}