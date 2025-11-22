use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_instruction;

use crate::crypto;
use crate::error::ErrorCode;
use crate::state::{CommitmentRegistry, PrivacyAccount, PrivacyPool};

#[derive(Accounts)]
pub struct DepositPrivate<'info> {
    #[account(mut)]
    pub pool: Account<'info, PrivacyPool>,
    /// CHECK: vault PDA
    #[account(mut, seeds = [b"vault", pool.key().as_ref()], bump = pool.vault_bump)]
    pub vault: UncheckedAccount<'info>,
    #[account(
        mut,
        seeds = [b"commitment", pool.key().as_ref()],
        bump = pool.commitment_bump
    )]
    pub commitment_registry: Account<'info, CommitmentRegistry>,
    #[account(
        mut,
        seeds = [b"privacy", user.key().as_ref()],
        bump
    )]
    pub privacy_account: Account<'info, PrivacyAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn deposit_private(ctx: Context<DepositPrivate>, amount: u64, nonce: u64) -> Result<()> {
    let pool = &mut ctx.accounts.pool;
    pool.check_not_paused()?;

    let (net_amount, _) = pool.apply_fee(amount)?;

    let transfer_ix =
        system_instruction::transfer(&ctx.accounts.user.key(), &ctx.accounts.vault.key(), amount);
    anchor_lang::solana_program::program::invoke(
        &transfer_ix,
        &[
            ctx.accounts.user.to_account_info(),
            ctx.accounts.vault.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
        ],
    )?;

    pool.total_locked = pool
        .total_locked
        .checked_add(net_amount)
        .ok_or(ErrorCode::MathOverflow)?;

    ctx.accounts.privacy_account.deposit(net_amount)?;

    let commitment = crypto::commitment(&ctx.accounts.user.key(), net_amount, nonce);
    ctx.accounts
        .commitment_registry
        .add_commitment(&pool.key(), commitment)?;

    Ok(())
}