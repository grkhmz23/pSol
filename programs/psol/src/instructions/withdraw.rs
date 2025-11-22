use anchor_lang::prelude::*;
use anchor_lang::system_program::{self, Transfer};

use crate::crypto;
use crate::error::ErrorCode;
use crate::state::*;

#[derive(Accounts)]
#[instruction(amount: u64, nullifier: [u8; 32])]
pub struct Withdraw<'info> {
    #[account(
        mut,
        seeds = [b"privacy_pool"],
        bump = pool.bump,
        constraint = !pool.paused @ ErrorCode::ProtocolPaused
    )]
    pub pool: Account<'info, PrivacyPool>,

    #[account(
        mut,
        seeds = [b"privacy_account", owner.key().as_ref()],
        bump = privacy_account.bump,
        has_one = owner
    )]
    pub privacy_account: Account<'info, PrivacyAccount>,

    // Nullifier PDA. init will fail if already used -> prevents replay.
    #[account(
        init,
        payer = owner,
        space = NullifierSet::SIZE,
        seeds = [b"nullifier", nullifier.as_ref()],
        bump
    )]
    pub nullifier_account: Account<'info, NullifierSet>,

    /// CHECK: Vault PDA holding SOL for pool.
    #[account(
        mut,
        seeds = [b"vault", pool.key().as_ref()],
        bump
    )]
    pub vault: AccountInfo<'info>,

    /// CHECK: Recipient of SOL. Must not equal vault.
    #[account(
        mut,
        constraint = recipient.key() != vault.key() @ ErrorCode::InvalidRecipient
    )]
    pub recipient: AccountInfo<'info>,

    #[account(mut)]
    pub owner: Signer<'info>,

    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(
    ctx: Context<Withdraw>,
    amount: u64,
    nullifier: [u8; 32],
    proof: Vec<u8>,
) -> Result<()> {
    let pool = &mut ctx.accounts.pool;
    let privacy_account = &mut ctx.accounts.privacy_account;
    let nullifier_account = &mut ctx.accounts.nullifier_account;

    // Input validation
    require!(amount > 0, ErrorCode::InvalidAmount);
    require!(amount <= pool.total_locked, ErrorCode::InsufficientBalance);
    require!(proof.len() >= 192, ErrorCode::InvalidProof);

    // Verify vault balance
    let vault_balance = ctx.accounts.vault.lamports();
    require!(vault_balance >= amount, ErrorCode::InsufficientBalance);

    // Proof verification (placeholder, fail-closed)
    let public_inputs = [privacy_account.commitment, nullifier];
    require!(
        crypto::verify_proof(&proof, &public_inputs)?,
        ErrorCode::InvalidProof
    );

    // Fee math with overflow protection
    let fee = amount
        .checked_mul(pool.withdraw_fee_bps as u64)
        .ok_or(ErrorCode::ArithmeticOverflow)?
        .checked_div(10_000)
        .ok_or(ErrorCode::ArithmeticOverflow)?;

    let net_amount = amount
        .checked_sub(fee)
        .ok_or(ErrorCode::ArithmeticOverflow)?;

    // Update encrypted balance (placeholder homomorphic subtraction)
    let encrypted_amount = crypto::encrypt_amount(amount, &privacy_account.encryption_key);
    privacy_account.encrypted_balance =
        crypto::homomorphic_sub(&privacy_account.encrypted_balance, &encrypted_amount);

    // Record nullifier
    nullifier_account.pool = pool.key();
    nullifier_account.nullifier = nullifier;
    nullifier_account.slot = Clock::get()?.slot;
    nullifier_account.bump = ctx.bumps.nullifier_account;

    // Transfer SOL via System Program CPI
    let pool_key = pool.key();
    let vault_seeds: &[&[u8]] = &[
        b"vault",
        pool_key.as_ref(),
        &[ctx.bumps.vault],
    ];
    let signer = &[vault_seeds];

    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.system_program.to_account_info(),
        Transfer {
            from: ctx.accounts.vault.to_account_info(),
            to: ctx.accounts.recipient.to_account_info(),
        },
        signer,
    );
    system_program::transfer(cpi_ctx, net_amount)?;

    // Update pool + account fields safely
    pool.total_locked = pool.total_locked
        .checked_sub(amount)
        .ok_or(ErrorCode::ArithmeticOverflow)?;

    privacy_account.total_withdrawals = privacy_account.total_withdrawals
        .checked_add(amount)
        .ok_or(ErrorCode::ArithmeticOverflow)?;

    privacy_account.last_update = Clock::get()?.slot;

    msg!("Withdrew {} SOL (fee: {} SOL)", net_amount, fee);

    emit!(WithdrawalEvent {
        pool: pool.key(),
        owner: ctx.accounts.owner.key(),
        recipient: ctx.accounts.recipient.key(),
        amount,
        fee,
        net_amount,
        nullifier,
        total_locked: pool.total_locked,
        timestamp: Clock::get()?.unix_timestamp,
    });

    Ok(())
}

#[event]
pub struct WithdrawalEvent {
    pub pool: Pubkey,
    pub owner: Pubkey,
    pub recipient: Pubkey,
    pub amount: u64,
    pub fee: u64,
    pub net_amount: u64,
    pub nullifier: [u8; 32],
    pub total_locked: u64,
    pub timestamp: i64,
}
