use anchor_lang::prelude::*;
use anchor_lang::solana_program::program_option::COption;
use anchor_spl::token::{self, Burn, Mint, MintTo, SetAuthority, Token, TokenAccount};
use psol::program::Psol;
use psol::{
    self, state::CommitmentRegistry, state::NullifierRegistry, state::PrivacyAccount,
    state::PrivacyPool,
};

pub mod error;
pub mod state;

pub use error::ErrorCode;
use state::Config;

declare_id!("CgERkyXGARsLFqgBHyTk4Njyt7nyq9foJfmV7ptadYLy");

const MINT_AUTH_SEED: &[u8] = b"psol_mint_auth";
const CONFIG_SEED: &[u8] = b"psol_config";

#[program]
pub mod psol_token {
    use super::*;

    pub fn initialize_token(ctx: Context<InitializeToken>) -> Result<()> {
        let config = &mut ctx.accounts.config;
        config.admin = ctx.accounts.admin.key();
        config.pool = ctx.accounts.pool.key();
        config.psol_mint = ctx.accounts.psol_mint.key();
        config.psol_program = ctx.accounts.psol_program.key();
        config.mint_authority_bump = *ctx.bumps.get("mint_authority").unwrap();
        config.bump = *ctx.bumps.get("config").unwrap();

        if ctx.accounts.psol_mint.mint_authority != COption::Some(ctx.accounts.mint_authority.key())
        {
            let cpi_ctx = CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                SetAuthority {
                    current_authority: ctx.accounts.admin.to_account_info(),
                    account_or_mint: ctx.accounts.psol_mint.to_account_info(),
                },
            );
            token::set_authority(
                cpi_ctx,
                anchor_spl::token::spl_token::instruction::AuthorityType::MintTokens,
                Some(ctx.accounts.mint_authority.key()),
            )?;
        }

        Ok(())
    }

    pub fn swap_to_psol(ctx: Context<SwapToPsol>, amount: u64, nonce: u64) -> Result<()> {
        require_keys_eq!(
            ctx.accounts.config.pool,
            ctx.accounts.pool.key(),
            ErrorCode::InvalidPool
        );
        require_keys_eq!(
            ctx.accounts.config.psol_program,
            ctx.accounts.psol_program.key(),
            ErrorCode::InvalidProgram
        );

        let seeds: &[&[u8]] = &[MINT_AUTH_SEED, &[ctx.accounts.config.mint_authority_bump]];

        let cpi_accounts = psol::cpi::accounts::DepositPrivate {
            pool: ctx.accounts.pool.to_account_info(),
            vault: ctx.accounts.vault.to_account_info(),
            commitment_registry: ctx.accounts.commitment_registry.to_account_info(),
            privacy_account: ctx.accounts.privacy_account.to_account_info(),
            user: ctx.accounts.user.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(ctx.accounts.psol_program.to_account_info(), cpi_accounts);
        psol::cpi::deposit_private(cpi_ctx, amount, nonce)?;

        let (net_amount, _) = ctx.accounts.pool.apply_fee(amount)?;
        let mint_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            MintTo {
                mint: ctx.accounts.psol_mint.to_account_info(),
                to: ctx.accounts.user_psol_account.to_account_info(),
                authority: ctx.accounts.mint_authority.to_account_info(),
            },
            &[seeds],
        );
        token::mint_to(mint_ctx, net_amount)?;
        Ok(())
    }

    pub fn swap_to_sol(ctx: Context<SwapToSol>, amount: u64, nullifier: [u8; 32]) -> Result<()> {
        require_keys_eq!(
            ctx.accounts.config.pool,
            ctx.accounts.pool.key(),
            ErrorCode::InvalidPool
        );
        require_keys_eq!(
            ctx.accounts.config.psol_program,
            ctx.accounts.psol_program.key(),
            ErrorCode::InvalidProgram
        );

        let burn_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Burn {
                mint: ctx.accounts.psol_mint.to_account_info(),
                from: ctx.accounts.user_psol_account.to_account_info(),
                authority: ctx.accounts.user.to_account_info(),
            },
        );
        token::burn(burn_ctx, amount)?;

        let cpi_accounts = psol::cpi::accounts::WithdrawPrivate {
            pool: ctx.accounts.pool.to_account_info(),
            vault: ctx.accounts.vault.to_account_info(),
            nullifier_registry: ctx.accounts.nullifier_registry.to_account_info(),
            privacy_account: ctx.accounts.privacy_account.to_account_info(),
            owner: ctx.accounts.user.to_account_info(),
            recipient: ctx.accounts.recipient.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(ctx.accounts.psol_program.to_account_info(), cpi_accounts);
        psol::cpi::withdraw_private(cpi_ctx, amount, nullifier)?;
        Ok(())
    }

    pub fn transfer_psol(_ctx: Context<TransferPsol>, _amount: u64) -> Result<()> {
        err!(ErrorCode::TransfersDisabled)
    }
}

#[derive(Accounts)]
pub struct InitializeToken<'info> {
    #[account(
        init,
        payer = admin,
        space = 8 + Config::SPACE,
        seeds = [CONFIG_SEED],
        bump
    )]
    pub config: Account<'info, Config>,
    pub psol_program: Program<'info, Psol>,
    pub pool: Account<'info, PrivacyPool>,
    #[account(mut)]
    pub psol_mint: Account<'info, Mint>,
    /// CHECK: PDA mint authority
    #[account(seeds = [MINT_AUTH_SEED], bump)]
    pub mint_authority: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct SwapToPsol<'info> {
    #[account(mut, seeds = [CONFIG_SEED], bump = config.bump)]
    pub config: Account<'info, Config>,
    pub psol_program: Program<'info, Psol>,
    #[account(mut)]
    pub pool: Account<'info, PrivacyPool>,
    /// CHECK: vault managed by psol
    #[account(mut, seeds = [b"vault", pool.key().as_ref()], bump = pool.vault_bump)]
    pub vault: UncheckedAccount<'info>,
    /// CHECK: registry PDAs validated in CPI
    #[account(mut, seeds = [b"commitment", pool.key().as_ref()], bump = pool.commitment_bump)]
    pub commitment_registry: Account<'info, CommitmentRegistry>,
    #[account(mut, seeds = [b"privacy", user.key().as_ref()], bump)]
    pub privacy_account: Account<'info, PrivacyAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub user_psol_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub psol_mint: Account<'info, Mint>,
    /// CHECK: PDA mint authority
    #[account(seeds = [MINT_AUTH_SEED], bump = config.mint_authority_bump)]
    pub mint_authority: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SwapToSol<'info> {
    #[account(mut, seeds = [CONFIG_SEED], bump = config.bump)]
    pub config: Account<'info, Config>,
    pub psol_program: Program<'info, Psol>,
    #[account(mut)]
    pub pool: Account<'info, PrivacyPool>,
    /// CHECK: vault managed by psol
    #[account(mut, seeds = [b"vault", pool.key().as_ref()], bump = pool.vault_bump)]
    pub vault: UncheckedAccount<'info>,
    /// CHECK: registry PDAs validated in CPI
    #[account(mut, seeds = [b"nullifier", pool.key().as_ref()], bump = pool.nullifier_bump)]
    pub nullifier_registry: Account<'info, NullifierRegistry>,
    #[account(mut, seeds = [b"privacy", user.key().as_ref()], bump)]
    pub privacy_account: Account<'info, PrivacyAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub user_psol_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub psol_mint: Account<'info, Mint>,
    /// CHECK: recipient of SOL
    #[account(mut)]
    pub recipient: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct TransferPsol {}