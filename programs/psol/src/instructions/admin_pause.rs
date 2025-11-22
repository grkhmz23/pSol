use anchor_lang::prelude::*;

use crate::error::ErrorCode;
use crate::state::PrivacyPool;

#[derive(Accounts)]
pub struct AdminPause<'info> {
    #[account(mut, has_one = admin @ ErrorCode::Unauthorized)]
    pub pool: Account<'info, PrivacyPool>,
    pub admin: Signer<'info>,
}

pub fn admin_pause(ctx: Context<AdminPause>) -> Result<()> {
    ctx.accounts.pool.paused = true;
    Ok(())
}
