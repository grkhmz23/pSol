use anchor_lang::prelude::*;

use crate::state::PrivacyAccount;

#[derive(Accounts)]
pub struct InitPrivacyAccount<'info> {
    #[account(
        init,
        payer = owner,
        space = 8 + PrivacyAccount::SPACE,
        seeds = [b"privacy", owner.key().as_ref()],
        bump
    )]
    pub privacy_account: Account<'info, PrivacyAccount>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn init_privacy_account(ctx: Context<InitPrivacyAccount>) -> Result<()> {
    ctx.accounts.privacy_account.owner = ctx.accounts.owner.key();
    ctx.accounts.privacy_account.balance = 0;
    Ok(())
}