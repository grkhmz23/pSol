use anchor_lang::prelude::*;

#[account]
pub struct TokenVault {
    pub authority: Pubkey,
    pub psol_mint: Pubkey,
    pub sol_vault: Pubkey,
    pub privacy_pool: Pubkey,
    pub total_supply: u64,
    pub total_locked: u64,
    pub swap_fee_bps: u16,
    pub paused: bool,
    pub bump: u8,
}

impl TokenVault {
    pub const SIZE: usize = 8 + 32 + 32 + 32 + 32 + 8 + 8 + 2 + 1 + 1;
}

#[account]
pub struct TokenPrivacyLink {
    pub owner: Pubkey,
    pub token_account: Pubkey,
    pub privacy_account: Pubkey,
    pub encrypted_balance: [u8; 64],
    pub last_sync: u64,
    pub bump: u8,
}

impl TokenPrivacyLink {
    pub const SIZE: usize = 8 + 32 + 32 + 32 + 64 + 8 + 1;
}
