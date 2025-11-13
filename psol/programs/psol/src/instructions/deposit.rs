use anchor_lang::prelude::*;
use anchor_lang::system_program::{self, Transfer};
use crate::state::*;
use crate::error::ErrorCode;
use crate::crypto;

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(
        mut,
        seeds = [b"privacy_pool"],
        bump = pool.bump
    )]
    pub pool: Account<'info, PrivacyPool>,
    
    #[account(
        init_if_needed,
        payer = user,
        space = PrivacyAccount::SIZE,
        seeds = [b"privacy_account", user.key().as_ref()],
        bump
    )]
    pub privacy_account: Account<'info, PrivacyAccount>,
    
    /// CHECK: Vault PDA
    #[account(
        mut,
        seeds = [b"vault", pool.key().as_ref()],
        bump
    )]
    pub vault: AccountInfo<'info>,
    
    #[account(mut)]
    pub user: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<Deposit>, amount: u64) -> Result<()> {
    let pool = &mut ctx.accounts.pool;
    let privacy_account = &mut ctx.accounts.privacy_account;
    let user = &ctx.accounts.user;
    let vault = &ctx.accounts.vault;
    
    require!(!pool.paused, ErrorCode::ProtocolPaused);
    require!(amount > 0, ErrorCode::InvalidAmount);
    
    // Calculate fee
    let fee = amount
        .checked_mul(pool.deposit_fee_bps as u64)
        .ok_or(ErrorCode::ArithmeticOverflow)?
        .checked_div(10000)
        .ok_or(ErrorCode::ArithmeticOverflow)?;
    
    let net_amount = amount
        .checked_sub(fee)
        .ok_or(ErrorCode::ArithmeticOverflow)?;
    
    // Transfer SOL to vault
    let transfer_ctx = CpiContext::new(
        ctx.accounts.system_program.to_account_info(),
        Transfer {
            from: user.to_account_info(),
            to: vault.to_account_info(),
        },
    );
    system_program::transfer(transfer_ctx, amount)?;
    
    // Initialize privacy account if needed
    if privacy_account.owner == Pubkey::default() {
        privacy_account.owner = user.key();
        privacy_account.encryption_key = [0u8; 32]; // User will set this
        privacy_account.encrypted_balance = [0u8; 64];
        privacy_account.commitment = [0u8; 32];
        privacy_account.last_update = Clock::get()?.slot;
        privacy_account.total_deposits = 0;
        privacy_account.total_withdrawals = 0;
        privacy_account.bump = ctx.bumps.privacy_account;
        
        pool.total_accounts = pool.total_accounts
            .checked_add(1)
            .ok_or(ErrorCode::ArithmeticOverflow)?;
    }
    
    // Update encrypted balance (simplified for MVP)
    let encrypted_amount = crypto::encrypt_amount(
        net_amount,
        &privacy_account.encryption_key
    );
    
    privacy_account.encrypted_balance = crypto::homomorphic_add(
        &privacy_account.encrypted_balance,
        &encrypted_amount
    );
    
    // Update pool state
    pool.total_locked = pool.total_locked
        .checked_add(net_amount)
        .ok_or(ErrorCode::ArithmeticOverflow)?;
    
    privacy_account.total_deposits = privacy_account.total_deposits
        .checked_add(net_amount)
        .ok_or(ErrorCode::ArithmeticOverflow)?;
    
    privacy_account.last_update = Clock::get()?.slot;
    
    msg!("Deposited {} SOL (fee: {} SOL)", net_amount, fee);
    msg!("Total locked: {} SOL", pool.total_locked);
    
    Ok(())
}
