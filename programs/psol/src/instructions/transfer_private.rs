use anchor_lang::prelude::*;

use crate::crypto;
use crate::state::{CommitmentRegistry, NullifierRegistry, PrivacyAccount, PrivacyPool};

#[derive(Accounts)]
pub struct TransferPrivate<'info> {
    #[account(mut)]
    pub pool: Account<'info, PrivacyPool>,
    #[account(
        mut,
        seeds = [b"commitment", pool.key().as_ref()],
        bump = pool.commitment_bump
    )]
    pub commitment_registry: Account<'info, CommitmentRegistry>,
    #[account(
        mut,
        seeds = [b"nullifier", pool.key().as_ref()],
        bump = pool.nullifier_bump
    )]
    pub nullifier_registry: Account<'info, NullifierRegistry>,
    #[account(mut, has_one = owner)]
    pub from_account: Account<'info, PrivacyAccount>,
    pub owner: Signer<'info>,
    #[account(mut)]
    pub to_account: Account<'info, PrivacyAccount>,
}

pub fn transfer_private(
    ctx: Context<TransferPrivate>,
    amount: u64,
    nullifier: [u8; 32],
    nonce: u64,
) -> Result<()> {
    ctx.accounts.pool.check_not_paused()?;
    ctx.accounts
        .nullifier_registry
        .register(&ctx.accounts.pool.key(), nullifier)?;

    ctx.accounts.from_account.withdraw(amount)?;
    ctx.accounts.to_account.deposit(amount)?;

    let commitment = crypto::commitment(&ctx.accounts.to_account.owner, amount, nonce);
    ctx.accounts
        .commitment_registry
        .add_commitment(&ctx.accounts.pool.key(), commitment)?;
    Ok(())
}