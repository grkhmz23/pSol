use anchor_lang::prelude::*;
use anchor_lang::system_program::{self, Transfer};
use anchor_spl::token::{self, Token, Mint, TokenAccount, MintTo};
use anchor_spl::associated_token::AssociatedToken;
use crate::state::*;
use crate::error::ErrorCode;

#[derive(Accounts)]
pub struct SwapToPsol<'info> {
    #[account(
        mut,
        seeds = [b"token_vault"],
        bump = vault.bump,
        has_one = psol_mint,
        has_one = sol_vault,
    )]
    pub vault: Account<'info, TokenVault>,

    #[account(
        mut,
        seeds = [b"psol_mint"],
        bump
    )]
    pub psol_mint: Account<'info, Mint>,

    /// CHECK: SOL vault PDA
    #[account(
        mut,
        seeds = [b"sol_vault", vault.key().as_ref()],
        bump
    )]
    pub sol_vault: AccountInfo<'info>,

    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = psol_mint,
        associated_token::authority = user
    )]
    pub user_psol_account: Account<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = user,
        space = TokenPrivacyLink::SIZE,
        seeds = [b"token_privacy_link", user.key().as_ref()],
        bump
    )]
    pub privacy_link: Account<'info, TokenPrivacyLink>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(ctx: Context<SwapToPsol>, amount: u64) -> Result<()> {
    let vault = &mut ctx.accounts.vault;
    require!(!vault.paused, ErrorCode::ProtocolPaused);

    require!(amount > 0, ErrorCode::InvalidAmount);
    require!(amount <= u64::MAX / 10000, ErrorCode::ArithmeticOverflow);

    let fee = amount
        .checked_mul(vault.swap_fee_bps as u64)
        .ok_or(ErrorCode::ArithmeticOverflow)?
        .checked_div(10000)
        .ok_or(ErrorCode::ArithmeticOverflow)?;

    let net_amount = amount
        .checked_sub(fee)
        .ok_or(ErrorCode::ArithmeticOverflow)?;

    // Transfer SOL to vault
    let cpi_context = CpiContext::new(
        ctx.accounts.system_program.to_account_info(),
        Transfer {
            from: ctx.accounts.user.to_account_info(),
            to: ctx.accounts.sol_vault.to_account_info(),
        },
    );
    system_program::transfer(cpi_context, amount)?;

    // Mint pSOL to user (vault PDA is mint authority)
    let seeds = &[
        b"token_vault",
        &[vault.bump],
    ];
    let signer = &[&seeds[..]];

    let mint_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        MintTo {
            mint: ctx.accounts.psol_mint.to_account_info(),
            to: ctx.accounts.user_psol_account.to_account_info(),
            authority: vault.to_account_info(),
        },
        signer,
    );
    token::mint_to(mint_ctx, net_amount)?;

    vault.total_supply = vault.total_supply
        .checked_add(net_amount)
        .ok_or(ErrorCode::ArithmeticOverflow)?;

    vault.total_locked = vault.total_locked
        .checked_add(amount)
        .ok_or(ErrorCode::ArithmeticOverflow)?;

    // Initialize privacy link if first time
    let privacy_link = &mut ctx.accounts.privacy_link;
    if privacy_link.owner == Pubkey::default() {
        privacy_link.owner = ctx.accounts.user.key();
        privacy_link.token_account = ctx.accounts.user_psol_account.key();
        privacy_link.privacy_account = Pubkey::default();
        privacy_link.encrypted_balance = [0u8; 64];
        privacy_link.last_sync = Clock::get()?.slot;
        privacy_link.bump = ctx.bumps.privacy_link;
    }

    msg!(
        "Swapped {} SOL to {} pSOL (fee: {} SOL)",
        amount,
        net_amount,
        fee
    );

    emit!(SwapToPsolEvent {
        user: ctx.accounts.user.key(),
        sol_amount: amount,
        psol_amount: net_amount,
        fee,
        total_supply: vault.total_supply,
    });

    Ok(())
}

#[event]
pub struct SwapToPsolEvent {
    pub user: Pubkey,
    pub sol_amount: u64,
    pub psol_amount: u64,
    pub fee: u64,
    pub total_supply: u64,
}
