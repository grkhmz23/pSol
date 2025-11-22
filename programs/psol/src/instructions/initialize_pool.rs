use anchor_lang::prelude::*;
use crate::state::*;
use crate::error::ErrorCode;

#[derive(Accounts)]
pub struct InitializePool<'info> {
    #[account(
        init,
        payer = admin,
        space = PrivacyPool::SIZE,
        seeds = [b"privacy_pool"],
        bump
    )]
    pub pool: Account<'info, PrivacyPool>,

    /// CHECK: SOL vault PDA owned by program
    #[account(
        mut,
        seeds = [b"vault", pool.key().as_ref()],
        bump
    )]
    pub vault: AccountInfo<'info>,

    #[account(
        init,
        payer = admin,
        space = CommitmentRegistry::SIZE,
        seeds = [b"commitment", pool.key().as_ref()],
        bump
    )]
    pub commitment_registry: Account<'info, CommitmentRegistry>,

    #[account(
        init,
        payer = admin,
        space = NullifierRegistry::SIZE,
        seeds = [b"nullifier", pool.key().as_ref()],
        bump
    )]
    pub nullifier_registry: Account<'info, NullifierRegistry>,

    #[account(mut)]
    pub admin: Signer<'info>,

    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(ctx: Context<InitializePool>, fee_bps: u16) -> Result<()> {
    require!(fee_bps <= 10_000, ErrorCode::FeeTooHigh);

    let pool = &mut ctx.accounts.pool;

    pool.admin = ctx.accounts.admin.key();
    pool.vault_bump = ctx.bumps.vault;
    pool.commitment_bump = ctx.bumps.commitment_registry;
    pool.nullifier_bump = ctx.bumps.nullifier_registry;
    pool.paused = false;
    pool.fee_bps = fee_bps;
    pool.total_locked = 0;
    pool.bump = ctx.bumps.pool;

    ctx.accounts.commitment_registry.pool = pool.key();
    ctx.accounts.commitment_registry.count = 0;

    ctx.accounts.nullifier_registry.pool = pool.key();
    ctx.accounts.nullifier_registry.count = 0;

    Ok(())
}
