use crate::error::ErrorCode;
use crate::state::TokenPrivacyLink;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct TransferPsol<'info> {
    #[account(
        mut,
        seeds = [b"privacy_link", sender.key().as_ref()],
        bump = sender_link.bump,
        has_one = sender @ ErrorCode::Unauthorized
    )]
    pub sender_link: Account<'info, TokenPrivacyLink>,

    #[account(
        mut,
        seeds = [b"privacy_link", recipient.key().as_ref()],
        bump = recipient_link.bump
    )]
    pub recipient_link: Account<'info, TokenPrivacyLink>,

    #[account(mut)]
    pub sender: Signer<'info>,

    /// CHECK: recipient authority
    pub recipient: AccountInfo<'info>,
}

pub fn handler(ctx: Context<TransferPsol>, amount: u64, _encrypted_amount: [u8; 64]) -> Result<()> {
    require!(amount > 0, ErrorCode::InvalidAmount);

    let sender_link = &mut ctx.accounts.sender_link;
    let recipient_link = &mut ctx.accounts.recipient_link;

    sender_link.last_sync = Clock::get()?.slot;
    recipient_link.last_sync = Clock::get()?.slot;

    Ok(())
}
