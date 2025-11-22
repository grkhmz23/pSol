use anchor_lang::prelude::*;

use crate::error::ErrorCode;
use crate::state::PrivacyPool;

#[derive(Accounts)]
pub struct AdminSetFees<'info> {
    #[account(mut, has_one = admin @ ErrorCode::Unauthorized)]
    pub pool: Account<'info, PrivacyPool>,
    pub admin: Signer<'info>,
}

pub fn admin_set_fees(ctx: Context<AdminSetFees>, fee_bps: u16) -> Result<()> {
    require!(fee_bps <= 10_000, ErrorCode::FeeTooHigh);
    ctx.accounts.pool.fee_bps = fee_bps;
    Ok(())
}