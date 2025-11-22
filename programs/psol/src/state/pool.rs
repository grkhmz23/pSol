use anchor_lang::prelude::*;

#[account]
pub struct PrivacyPool {
    pub admin: Pubkey,
    pub vault_bump: u8,
    pub commitment_bump: u8,
    pub nullifier_bump: u8,
    pub paused: bool,
    pub fee_bps: u16,
    pub total_locked: u64,
}

impl PrivacyPool {
    pub const SPACE: usize = 32 + 1 + 1 + 1 + 1 + 2 + 8;

    pub fn check_not_paused(&self) -> Result<()> {
        require!(!self.paused, crate::error::ErrorCode::PoolPaused);
        Ok(())
    }

    pub fn apply_fee(&self, amount: u64) -> Result<(u64, u64)> {
        let fee = amount
            .checked_mul(self.fee_bps as u64)
            .and_then(|v| v.checked_div(10_000))
            .ok_or(crate::error::ErrorCode::MathOverflow)?;
        let net = amount
            .checked_sub(fee)
            .ok_or(crate::error::ErrorCode::AmountTooSmall)?;
        Ok((net, fee))
    }
}