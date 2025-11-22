use anchor_lang::prelude::*;
use anchor_lang::system_program;
use anchor_spl::token::{self, Burn, Mint, Token, TokenAccount};
use crate::state::*;
use crate::error::ErrorCode;

#[derive(Accounts)]
#[instruction(amount: u64, nullifier: [u8; 32])]
pub struct SwapToSol<'info> {
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
        mut,
        associated_token::mint = psol_mint,
        associated_token::authority = user,
        constraint = user_psol_account.amount >= amount @ ErrorCode::InsufficientBalance
    )]
    pub user_psol_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [b"token_privacy_link", user.key().as_ref()],
        bump = privacy_link.bump,
        has_one = owner
    )]
    pub privacy_link: Account<'info, TokenPrivacyLink>,

    // Nullifier PDA to prevent replay.
    // If it already exists, init will fail -> double-spend prevented.
    #[account(
        init,
        payer = user,
        space = 8 + 32 + 32 + 8,
        seeds = [b"nullifier", nullifier.as_ref()],
        bump
    )]
    pub nullifier_account: AccountInfo<'info>,

    /// CHECK: Recipient of SOL
    #[account(mut)]
    pub recipient: AccountInfo<'info>,

    #[account(mut)]
    pub user: Signer<'info>,
    pub owner: Pubkey,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

pub fn handler(
    ctx: Context<SwapToSol>,
    amount: u64,
    nullifier: [u8; 32],
) -> Result<()> {
    let vault = &mut ctx.accounts.vault;
    require!(!vault.paused, ErrorCode::ProtocolPaused);

    require!(amount > 0, ErrorCode::InvalidAmount);
    require!(amount <= vault.total_supply, ErrorCode::InsufficientBalance);

    let fee = amount
        .checked_mul(vault.swap_fee_bps as u64)
        .ok_or(ErrorCode::ArithmeticOverflow)?
        .checked_div(10000)
        .ok_or(ErrorCode::ArithmeticOverflow)?;

    let net_amount = amount
        .checked_sub(fee)
        .ok_or(ErrorCode::ArithmeticOverflow)?;

    let vault_balance = ctx.accounts.sol_vault.lamports();
    require!(vault_balance >= net_amount, ErrorCode::InsufficientBalance);

    // Burn pSOL from user
    let burn_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        Burn {
            mint: ctx.accounts.psol_mint.to_account_info(),
            from: ctx.accounts.user_psol_account.to_account_info(),
            authority: ctx.accounts.user.to_account_info(),
        },
    );
    token::burn(burn_ctx, amount)?;

    // Transfer SOL from vault PDA to recipient via CPI
    let vault_key = vault.key();
    let sol_vault_seeds = &[
        b"sol_vault",
        vault_key.as_ref(),
        &[ctx.bumps.sol_vault],
    ];
    let signer = &[&sol_vault_seeds[..]];

    let transfer_ix = system_program::Transfer {
        from: ctx.accounts.sol_vault.to_account_info(),
        to: ctx.accounts.recipient.to_account_info(),
    };

    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.system_program.to_account_info(),
        transfer_ix,
        signer,
    );
    system_program::transfer(cpi_ctx, net_amount)?;

    vault.total_supply = vault.total_supply
        .checked_sub(amount)
        .ok_or(ErrorCode::ArithmeticOverflow)?;

    vault.total_locked = vault.total_locked
        .checked_sub(net_amount)
        .ok_or(ErrorCode::ArithmeticOverflow)?;

    // Store nullifier marker
    let nullifier_data = &mut ctx.accounts.nullifier_account.try_borrow_mut_data()?;
    nullifier_data[0..8].copy_from_slice(&[0xff; 8]);
    nullifier_data[8..40].copy_from_slice(vault.key().as_ref());
    nullifier_data[40..72].copy_from_slice(&nullifier);
    let slot = Clock::get()?.slot;
    nullifier_data[72..80].copy_from_slice(&slot.to_le_bytes());

    msg!(
        "Swapped {} pSOL to {} SOL (fee: {} pSOL)",
        amount,
        net_amount,
        fee
    );

    emit!(SwapToSolEvent {
        user: ctx.accounts.user.key(),
        recipient: ctx.accounts.recipient.key(),
        psol_amount: amount,
        sol_amount: net_amount,
        fee,
        nullifier,
        total_supply: vault.total_supply,
    });

    Ok(())
}

#[event]
pub struct SwapToSolEvent {
    pub user: Pubkey,
    pub recipient: Pubkey,
    pub psol_amount: u64,
    pub sol_amount: u64,
    pub fee: u64,
    pub nullifier: [u8; 32],
    pub total_supply: u64,
}
