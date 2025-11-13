use anchor_lang::prelude::*;
use crate::state::*;
use crate::error::ErrorCode;
use crate::crypto;

#[derive(Accounts)]
#[instruction(amount: u64, nullifier: [u8; 32])]
pub struct Withdraw<'info> {
    #[account(
        mut,
        seeds = [b"privacy_pool"],
        bump = pool.bump
    )]
    pub pool: Account<'info, PrivacyPool>,
    
    #[account(
        mut,
        seeds = [b"privacy_account", owner.key().as_ref()],
        bump = privacy_account.bump,
        has_one = owner
    )]
    pub privacy_account: Account<'info, PrivacyAccount>,
    
    #[account(
        init,
        payer = owner,
        space = NullifierSet::SIZE,
        seeds = [b"nullifier", nullifier.as_ref()],
        bump
    )]
    pub nullifier_account: Account<'info, NullifierSet>,
    
    /// CHECK: Vault PDA
    #[account(
        mut,
        seeds = [b"vault", pool.key().as_ref()],
        bump
    )]
    pub vault: AccountInfo<'info>,
    
    /// CHECK: Recipient can be any address
    #[account(mut)]
    pub recipient: AccountInfo<'info>,
    
    #[account(mut)]
    pub owner: Signer<'info>,
    
    pub system_program: Program<'info, System>,
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
    let vault = &ctx.accounts.vault;
    let recipient = &ctx.accounts.recipient;
    
    require!(!pool.paused, ErrorCode::ProtocolPaused);
    require!(amount > 0, ErrorCode::InvalidAmount);
    
    // Verify zero-knowledge proof
    let public_inputs = [
        privacy_account.commitment,
        nullifier,
    ];
    
    require!(
        crypto::verify_proof(&proof, &public_inputs)?,
        ErrorCode::InvalidProof
    );
    
    // Calculate fee
    let fee = amount
        .checked_mul(pool.withdraw_fee_bps as u64)
        .ok_or(ErrorCode::ArithmeticOverflow)?
        .checked_div(10000)
        .ok_or(ErrorCode::ArithmeticOverflow)?;
    
    let net_amount = amount
        .checked_sub(fee)
        .ok_or(ErrorCode::ArithmeticOverflow)?;
    
    // Update encrypted balance
    let encrypted_amount = crypto::encrypt_amount(
        amount,
        &privacy_account.encryption_key
    );
    
    privacy_account.encrypted_balance = crypto::homomorphic_sub(
        &privacy_account.encrypted_balance,
        &encrypted_amount
    );
    
    // Record nullifier
    nullifier_account.pool = pool.key();
    nullifier_account.nullifier = nullifier;
    nullifier_account.slot = Clock::get()?.slot;
    nullifier_account.bump = ctx.bumps.nullifier_account;
    
    // Transfer SOL from vault to recipient
    let pool_key = pool.key();
    let vault_seeds = &[
        b"vault",
        pool_key.as_ref(),
        &[ctx.bumps.vault],
    ];
    let signer_seeds = &[&vault_seeds[..]];
    
    **vault.try_borrow_mut_lamports()? = vault
        .lamports()
        .checked_sub(net_amount)
        .ok_or(ErrorCode::InsufficientBalance)?;
    
    **recipient.try_borrow_mut_lamports()? = recipient
        .lamports()
        .checked_add(net_amount)
        .ok_or(ErrorCode::ArithmeticOverflow)?;
    
    // Update pool state
    pool.total_locked = pool.total_locked
        .checked_sub(amount)
        .ok_or(ErrorCode::InsufficientBalance)?;
    
    privacy_account.total_withdrawals = privacy_account.total_withdrawals
        .checked_add(amount)
        .ok_or(ErrorCode::ArithmeticOverflow)?;
    
    privacy_account.last_update = Clock::get()?.slot;
    
    msg!("Withdrew {} SOL (fee: {} SOL)", net_amount, fee);
    msg!("Total locked: {} SOL", pool.total_locked);
    
    Ok(())
}
