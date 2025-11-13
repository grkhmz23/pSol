use anchor_lang::prelude::*;
use crate::state::*;
use crate::error::ErrorCode;
use crate::crypto;

#[derive(Accounts)]
pub struct Transfer<'info> {
    #[account(
        seeds = [b"privacy_pool"],
        bump = pool.bump
    )]
    pub pool: Account<'info, PrivacyPool>,
    
    #[account(
        mut,
        seeds = [b"privacy_account", sender.key().as_ref()],
        bump = sender_account.bump,
        has_one = sender @ ErrorCode::Unauthorized
    )]
    pub sender_account: Account<'info, PrivacyAccount>,
    
    #[account(
        mut,
        seeds = [b"privacy_account", recipient.key().as_ref()],
        bump = recipient_account.bump
    )]
    pub recipient_account: Account<'info, PrivacyAccount>,
    
    pub sender: Signer<'info>,
    
    /// CHECK: Recipient address
    pub recipient: AccountInfo<'info>,
}

pub fn handler(
    ctx: Context<Transfer>,
    encrypted_amount: [u8; 64],
    proof: Vec<u8>,
) -> Result<()> {
    let pool = &ctx.accounts.pool;
    let sender_account = &mut ctx.accounts.sender_account;
    let recipient_account = &mut ctx.accounts.recipient_account;
    
    require!(!pool.paused, ErrorCode::ProtocolPaused);
    
    // Verify zero-knowledge proof
    let public_inputs = [
        sender_account.commitment,
        recipient_account.commitment,
    ];
    
    require!(
        crypto::verify_proof(&proof, &public_inputs)?,
        ErrorCode::InvalidProof
    );
    
    // Update encrypted balances homomorphically
    sender_account.encrypted_balance = crypto::homomorphic_sub(
        &sender_account.encrypted_balance,
        &encrypted_amount
    );
    
    recipient_account.encrypted_balance = crypto::homomorphic_add(
        &recipient_account.encrypted_balance,
        &encrypted_amount
    );
    
    // Update timestamps
    let slot = Clock::get()?.slot;
    sender_account.last_update = slot;
    recipient_account.last_update = slot;
    
    msg!("Private transfer completed");
    
    Ok(())
}
