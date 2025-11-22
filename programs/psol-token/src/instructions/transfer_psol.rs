use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer as TokenTransfer};
use crate::state::*;
use crate::error::ErrorCode;

#[derive(Accounts)]
pub struct TransferPsol<'info> {
    #[account(
        seeds = [b"token_vault"],
        bump = vault.bump,
    )]
    pub vault: Account<'info, TokenVault>,

    #[account(
        mut,
        associated_token::mint = vault.psol_mint,
        associated_token::authority = sender,
        constraint = sender_psol_account.amount >= amount @ ErrorCode::InsufficientBalance
    )]
    pub sender_psol_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = vault.psol_mint,
        associated_token::authority = recipient
    )]
    pub recipient_psol_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [b"token_privacy_link", sender.key().as_ref()],
        bump = sender_link.bump,
        has_one = owner,
        constraint = sender_link.owner == sender.key() @ ErrorCode::Unauthorized
    )]
    pub sender_link: Account<'info, TokenPrivacyLink>,

    #[account(
        init_if_needed,
        payer = sender,
        space = TokenPrivacyLink::SIZE,
        seeds = [b"token_privacy_link", recipient.key().as_ref()],
        bump
    )]
    pub recipient_link: Account<'info, TokenPrivacyLink>,

    #[account(mut)]
    pub sender: Signer<'info>,

    /// CHECK: any valid recipient
    pub recipient: AccountInfo<'info>,
    pub owner: Pubkey,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<TransferPsol>,
    amount: u64,
    encrypted_amount: [u8; 64],
) -> Result<()> {
    require!(!ctx.accounts.vault.paused, ErrorCode::ProtocolPaused);
    require!(amount > 0, ErrorCode::InvalidAmount);

    let transfer_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        TokenTransfer {
            from: ctx.accounts.sender_psol_account.to_account_info(),
            to: ctx.accounts.recipient_psol_account.to_account_info(),
            authority: ctx.accounts.sender.to_account_info(),
        },
    );
    token::transfer(transfer_ctx, amount)?;

    let sender_link = &mut ctx.accounts.sender_link;
    let recipient_link = &mut ctx.accounts.recipient_link;

    if recipient_link.owner == Pubkey::default() {
        recipient_link.owner = ctx.accounts.recipient.key();
        recipient_link.token_account = ctx.accounts.recipient_psol_account.key();
        recipient_link.privacy_account = Pubkey::default();
        recipient_link.encrypted_balance = [0u8; 64];
        recipient_link.last_sync = Clock::get()?.slot;
        recipient_link.bump = ctx.bumps.recipient_link;
    }

    // Placeholder encrypted balance sync
    sender_link.encrypted_balance = encrypted_amount;
    recipient_link.last_sync = Clock::get()?.slot;
    sender_link.last_sync = Clock::get()?.slot;

    msg!(
        "Transferred {} pSOL from {} to {}",
        amount,
        ctx.accounts.sender.key(),
        ctx.accounts.recipient.key()
    );

    emit!(TransferPsolEvent {
        sender: ctx.accounts.sender.key(),
        recipient: ctx.accounts.recipient.key(),
        amount,
        encrypted_amount,
        timestamp: Clock::get()?.unix_timestamp,
    });

    Ok(())
}

#[event]
pub struct TransferPsolEvent {
    pub sender: Pubkey,
    pub recipient: Pubkey,
    pub amount: u64,
    pub encrypted_amount: [u8; 64],
    pub timestamp: i64,
}
