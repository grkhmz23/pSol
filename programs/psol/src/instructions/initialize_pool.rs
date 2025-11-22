use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_instruction;
use anchor_lang::solana_program::system_program;

use crate::error::ErrorCode;
use crate::state::{CommitmentRegistry, NullifierRegistry, PrivacyPool};

#[derive(Accounts)]
pub struct InitializePool<'info> {
    #[account(init, payer = admin, space = 8 + PrivacyPool::SPACE)]
    pub pool: Account<'info, PrivacyPool>,
    #[account(
        init,
        payer = admin,
        space = CommitmentRegistry::SPACE,
        seeds = [b"commitment", pool.key().as_ref()],
        bump
    )]
    pub commitment_registry: Account<'info, CommitmentRegistry>,
    #[account(
        init,
        payer = admin,
        space = NullifierRegistry::SPACE,
        seeds = [b"nullifier", pool.key().as_ref()],
        bump
    )]
    pub nullifier_registry: Account<'info, NullifierRegistry>,
    #[account(mut)]
    pub admin: Signer<'info>,
    /// CHECK: vault PDA controlled by program
    #[account(mut, seeds = [b"vault", pool.key().as_ref()], bump)]
    pub vault: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

pub fn initialize_pool(ctx: Context<InitializePool>, fee_bps: u16) -> Result<()> {
    require!(fee_bps <= 10_000, ErrorCode::FeeTooHigh);

    let pool = &mut ctx.accounts.pool;
    let vault_bump = *ctx.bumps.get("vault").ok_or(ErrorCode::InvalidVault)?;
    let commitment_bump = *ctx
        .bumps
        .get("commitment_registry")
        .ok_or(ErrorCode::InvalidRegistry)?;
    let nullifier_bump = *ctx
        .bumps
        .get("nullifier_registry")
        .ok_or(ErrorCode::InvalidRegistry)?;

    // create vault PDA
    let rent_lamports = Rent::get()?.minimum_balance(0);
    let create_ix = system_instruction::create_account(
        &ctx.accounts.admin.key(),
        &ctx.accounts.vault.key(),
        rent_lamports,
        0,
        &system_program::ID,
    );
    let seeds: &[&[u8]] = &[b"vault", pool.key().as_ref(), &[vault_bump]];
    let signer_seeds = &[seeds];
    anchor_lang::solana_program::program::invoke_signed(
        &create_ix,
        &[
            ctx.accounts.admin.to_account_info(),
            ctx.accounts.vault.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
        ],
        signer_seeds,
    )?;

    pool.admin = ctx.accounts.admin.key();
    pool.vault_bump = vault_bump;
    pool.commitment_bump = commitment_bump;
    pool.nullifier_bump = nullifier_bump;
    pool.paused = false;
    pool.fee_bps = fee_bps;
    pool.total_locked = 0;

    ctx.accounts.commitment_registry.pool = pool.key();
    ctx.accounts.commitment_registry.count = 0;
    ctx.accounts.nullifier_registry.pool = pool.key();
    ctx.accounts.nullifier_registry.count = 0;

    Ok(())
}