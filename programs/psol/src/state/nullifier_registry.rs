use anchor_lang::prelude::*;
use crate::error::ErrorCode;

#[account]
pub struct NullifierRegistry {
    pub pool: Pubkey,
    pub count: u64,
    pub nullifiers: [[u8; 32]; 1024],
}

impl NullifierRegistry {
    pub const MAX: usize = 1024;
    pub const SIZE: usize = 8  // discriminator
        + 32                   // pool
        + 8                    // count
        + (32 * Self::MAX);    // nullifiers

    pub fn register(&mut self, pool: &Pubkey, nullifier: [u8; 32]) -> Result<()> {
        require_keys_eq!(self.pool, *pool, ErrorCode::InvalidRegistry);

        for i in 0..(self.count as usize) {
            if self.nullifiers[i] == nullifier {
                return err!(ErrorCode::NullifierAlreadyUsed);
            }
        }

        if (self.count as usize) >= Self::MAX {
            return err!(ErrorCode::CommitmentRegistryFull);
        }

        self.nullifiers[self.count as usize] = nullifier;
        self.count = self.count.checked_add(1).ok_or(ErrorCode::MathOverflow)?;
        Ok(())
    }
}
