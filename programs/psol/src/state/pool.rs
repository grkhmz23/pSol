use anchor_lang::prelude::*;
use crate::error::ErrorCode;

#[account]
pub struct PrivacyPool {
    pub admin: Pubkey,
    pub vault_bump: u8,
    pub commitment_bump: u8,
    pub nullifier_bump: u8,
    pub paused: bool,
    pub fee_bps: u16,
    pub total_locked: u64,
    pub bump: u8,
}

impl PrivacyPool {
    pub const SIZE: usize = 8  // discriminator
        + 32                   // admin
        + 1                    // vault_bump
        + 1                    // commitment_bump
        + 1                    // nullifier_bump
        + 1                    // paused
        + 2                    // fee_bps
        + 8                    // total_locked
        + 1;                   // bump

    pub fn check_not_paused(&self) -> Result<()> {
        require!(!self.paused, ErrorCode::PoolPaused);
        Ok(())
    }

    pub fn lock(&mut self, amount: u64) -> Result<()> {
        self.total_locked = self
            .total_locked
            .checked_add(amount)
            .ok_or(ErrorCode::MathOverflow)?;
        Ok(())
    }

    pub fn unlock(&mut self, amount: u64) -> Result<()> {
        self.total_locked = self
            .total_locked
            .checked_sub(amount)
            .ok_or(ErrorCode::AmountTooSmall)?;
        Ok(())
    }
}
