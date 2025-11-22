use anchor_lang::prelude::*;

use crate::error::ErrorCode;

#[account]
pub struct PrivacyAccount {
    pub owner: Pubkey,
    pub balance: u64,
}

impl PrivacyAccount {
    pub const SPACE: usize = 32 + 8;

    pub fn deposit(&mut self, amount: u64) -> Result<()> {
        self.balance = self
            .balance
            .checked_add(amount)
            .ok_or(ErrorCode::MathOverflow)?;
        Ok(())
    }

    pub fn withdraw(&mut self, amount: u64) -> Result<()> {
        require!(self.balance >= amount, ErrorCode::InsufficientBalance);
        self.balance = self
            .balance
            .checked_sub(amount)
            .ok_or(ErrorCode::MathOverflow)?;
        Ok(())
    }
}