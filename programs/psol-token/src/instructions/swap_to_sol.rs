use crate::error::ErrorCode;
use crate::state::{TokenPrivacyLink, TokenVault};
use anchor_lang::prelude::*;
use solana_program::{program::invoke_signed, system_instruction};

#[derive(Accounts)]
#[instruction(amount: u64, nullifier: [u8; 32])]
pub struct SwapToSol<'info> {
    #[account(
        mut,
        seeds = [b"token_vault"],
        bump = token_vault.bump
    )]
    pub token_vault: Account<'info, TokenVault>,

    #[account(
        mut,
        seeds = [b"sol_vault", token_vault.key().as_ref()],
        bump
    )]
    pub sol_vault: SystemAccount<'info>,

    #[account(
        mut,
        seeds = [b"privacy_link", user.key().as_ref()],
        bump = token_privacy_link.bump,
        has_one = user @ ErrorCode::Unauthorized
    )]
    pub token_privacy_link: Account<'info, TokenPrivacyLink>,

    /// CHECK: tracks nullifier usage
    #[account(
        init,
        payer = user,
        space = 8,
        seeds = [b"nullifier", nullifier.as_ref()],
        bump
    )]
    pub nullifier_record: SystemAccount<'info>,

    #[account(mut)]
    pub user: Signer<'info>,

    /// CHECK: recipient of SOL
    #[account(mut)]
    pub recipient: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<SwapToSol>, amount: u64, _nullifier: [u8; 32]) -> Result<()> {
    let vault = &mut ctx.accounts.token_vault;
    require!(!vault.paused, ErrorCode::ProtocolPaused);
    require!(amount > 0, ErrorCode::InvalidAmount);
    require!(vault.total_locked >= amount, ErrorCode::InsufficientBalance);

    vault.total_locked = vault
        .total_locked
        .checked_sub(amount)
        .ok_or(ErrorCode::ArithmeticOverflow)?;
    vault.total_supply = vault
        .total_supply
        .checked_sub(amount)
        .ok_or(ErrorCode::ArithmeticOverflow)?;

    let ix = system_instruction::transfer(
        &ctx.accounts.sol_vault.key(),
        &ctx.accounts.recipient.key(),
        amount,
    );

    let signer_seeds: &[&[u8]] = &[
        b"sol_vault",
        vault.key().as_ref(),
        &[ctx.bumps.sol_vault],
    ];

    invoke_signed(
        &ix,
        &[
            ctx.accounts.sol_vault.to_account_info(),
            ctx.accounts.recipient.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
        ],
        &[signer_seeds],
    )?;

    ctx.accounts.token_privacy_link.last_sync = Clock::get()?.slot;
    Ok(())
}
