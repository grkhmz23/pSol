use anchor_lang::prelude::*;

#[account]
pub struct Config {
    pub admin: Pubkey,
    pub psol_program: Pubkey,
    pub pool: Pubkey,
    pub psol_mint: Pubkey,
    pub mint_authority_bump: u8,
    pub bump: u8,
}

impl Config {
    pub const SPACE: usize = 32 + 32 + 32 + 32 + 1 + 1;
