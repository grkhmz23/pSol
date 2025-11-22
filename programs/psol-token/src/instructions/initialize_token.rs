use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, Mint};
use crate::state::*;
use crate::error::ErrorCode;

#[derive(Accounts)]
pub struct InitializeToken<'info> {
    #[account(
        init,
        payer = authority,
        space = TokenVault::SIZE,
        seeds = [b"token_vault"],
        bump
    )]
    pub vault: Account<'info, TokenVault>,

    #[account(
        init,
        payer = authority,
        mint::decimals = 9,
        mint::authority = vault.key(),
        mint::freeze_authority = vault.key(),
        seeds = [b"psol_mint"],
        bump
    )]
    pub psol_mint: Account<'info, Mint>,

    /// CHECK: SOL vault PDA (system-owned vault account)
    /// Claude used AccountInfo with seeds. This PDA must exist to hold SOL.
    #[account(
        mut,
        seeds = [b"sol_vault", vault.key().as_ref()],
        bump
    )]
    pub sol_vault: AccountInfo<'info>,

    /// CHECK: Privacy pool reference (placeholder link to privacy protocol)
    #[account(
        seeds = [b"privacy_pool"],
        bump,
        seeds::program = crate::ID
    )]
    pub privacy_pool: AccountInfo<'info>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(ctx: Context<InitializeToken>) -> Result<()> {
    let vault = &mut ctx.accounts.vault;

    vault.authority = ctx.accounts.authority.key();
    vault.psol_mint = ctx.accounts.psol_mint.key();
    vault.sol_vault = ctx.accounts.sol_vault.key();
    vault.privacy_pool = ctx.accounts.privacy_pool.key();

    vault.total_supply = 0;
    vault.total_locked = 0;
    vault.swap_fee_bps = 10; // 0.1% fee
    vault.paused = false;
    vault.bump = ctx.bumps.vault;

    msg!("Token vault initialized");
    msg!("pSOL mint: {}", vault.psol_mint);
    msg!("SOL vault: {}", vault.sol_vault);
    msg!("Authority: {}", vault.authority);

    emit!(TokenVaultInitialized {
        vault: vault.key(),
        authority: vault.authority,
        psol_mint: vault.psol_mint,
        sol_vault: vault.sol_vault,
        swap_fee_bps: vault.swap_fee_bps,
    });

    Ok(())
}

#[event]
pub struct TokenVaultInitialized {
    pub vault: Pubkey,
    pub authority: Pubkey,
    pub psol_mint: Pubkey,
    pub sol_vault: Pubkey,
    pub swap_fee_bps: u16,
}
