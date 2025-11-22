use anchor_lang::prelude::*;
use crate::error::ErrorCode;

#[account]
pub struct CommitmentRegistry {
    pub pool: Pubkey,
    pub count: u64,
    pub commitments: [[u8; 32]; 1024],
}

impl CommitmentRegistry {
    pub const MAX: usize = 1024;
    pub const SIZE: usize = 8  // discriminator
        + 32                   // pool
        + 8                    // count
        + (32 * Self::MAX);    // commitments

    pub fn add_commitment(&mut self, pool: &Pubkey, commitment: [u8; 32]) -> Result<()> {
        require_keys_eq!(self.pool, *pool, ErrorCode::InvalidRegistry);

        if (self.count as usize) >= Self::MAX {
            return err!(ErrorCode::CommitmentRegistryFull);
        }

        self.commitments[self.count as usize] = commitment;
        self.count = self.count.checked_add(1).ok_or(ErrorCode::MathOverflow)?;
        Ok(())
    }
}
