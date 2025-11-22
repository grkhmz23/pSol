use anchor_lang::prelude::*;

#[account]
pub struct PrivacyAccount {
    pub owner: Pubkey,
    pub encrypted_balance: [u8; 64],
    pub commitment: [u8; 32],
    pub encryption_key: [u8; 32],
    pub nonce: u64,
    pub total_deposits: u64,
    pub total_withdrawals: u64,
    pub last_update: u64,
    pub bump: u8,
}

impl PrivacyAccount {
    pub const SIZE: usize = 8   // discriminator
        + 32                    // owner
        + 64                    // encrypted_balance
        + 32                    // commitment
        + 32                    // encryption_key
        + 8                     // nonce
        + 8                     // total_deposits
        + 8                     // total_withdrawals
        + 8                     // last_update
        + 1;                    // bump
}
