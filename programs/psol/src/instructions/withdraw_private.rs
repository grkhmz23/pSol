use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke_signed;
use anchor_lang::solana_program::system_instruction;

use crate::error::ErrorCode;
use crate::state::{NullifierRegistry, PrivacyAccount, PrivacyPool};

#[derive(Accounts)]
pub struct WithdrawPrivate<'info> {
    #[account(mut)]
    pub pool: Account<'info, PrivacyPool>,
    /// CHECK: vault PDA
    #[account(mut, seeds = [b"vault", pool.key().as_ref()], bump = pool.vault_bump)]
    pub vault: UncheckedAccount<'info>,
    #[account(
        mut,
        seeds = [b"nullifier", pool.key().as_ref()],
        bump = pool.nullifier_bump
    )]
    pub nullifier_registry: Account<'info, NullifierRegistry>,
    #[account(mut, has_one = owner)]
    pub privacy_account: Account<'info, PrivacyAccount>,
    pub owner: Signer<'info>,
    /// CHECK: recipient for SOL withdrawal
    #[account(mut)]
    pub recipient: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

pub fn withdraw_private(
    ctx: Context<WithdrawPrivate>,
    amount: u64,
    nullifier: [u8; 32],
) -> Result<()> {
    ctx.accounts.pool.check_not_paused()?;
    ctx.accounts
        .nullifier_registry
        .register(&ctx.accounts.pool.key(), nullifier)?;

    ctx.accounts.privacy_account.withdraw(amount)?;
    let (net_amount, _) = ctx.accounts.pool.apply_fee(amount)?;

    ctx.accounts.pool.total_locked = ctx
        .accounts
        .pool
        .total_locked
        .checked_sub(net_amount)
        .ok_or(ErrorCode::MathOverflow)?;

    let seeds: &[&[u8]] = &[
        b"vault",
        ctx.accounts.pool.key().as_ref(),
        &[ctx.accounts.pool.vault_bump],
    ];
    invoke_signed(
        &system_instruction::transfer(
            &ctx.accounts.vault.key(),
            &ctx.accounts.recipient.key(),
            net_amount,
        ),
        &[
            ctx.accounts.vault.to_account_info(),
            ctx.accounts.recipient.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
        ],
        &[seeds],
    )?;

    Ok(())
}