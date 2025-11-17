use anchor_lang::prelude::*;
use anchor_lang::system_program::{self, Transfer};
use crate::state::*;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = authority,
        space = PrivacyPool::SIZE,
        seeds = [b"privacy_pool"],
        bump
    )]
    pub pool: Account<'info, PrivacyPool>,
    
    /// CHECK: Vault PDA for holding SOL
    #[account(
        mut,
        seeds = [b"vault", pool.key().as_ref()],
        bump
    )]
    pub vault: AccountInfo<'info>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<Initialize>) -> Result<()> {
    let pool = &mut ctx.accounts.pool;
    
    pool.authority = ctx.accounts.authority.key();
    pool.vault = ctx.accounts.vault.key();
    pool.total_locked = 0;
    pool.total_accounts = 0;
    pool.deposit_fee_bps = 10; // 0.1%
    pool.withdraw_fee_bps = 10; // 0.1%
    pool.paused = false;
    pool.bump = ctx.bumps.pool;
    
    msg!("Privacy pool initialized");
    msg!("Vault: {}", pool.vault);
    msg!("Deposit fee: {}bps", pool.deposit_fee_bps);
    msg!("Withdraw fee: {}bps", pool.withdraw_fee_bps);
    
    Ok(())
}
