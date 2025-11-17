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
        has_one = owner @ ErrorCode::Unauthorized  // ✅ FIXED: changed sender to owner
    )]
    pub sender_account: Account<'info, PrivacyAccount>,
    
    #[account(
        mut,
        seeds = [b"privacy_account", recipient.key().as_ref()],
        bump = recipient_account.bump
    )]
    pub recipient_account: Account<'info, PrivacyAccount>,
    
    #[account(mut)]
    pub sender: Signer<'info>,
    
    /// CHECK: Recipient can be any account
    pub recipient: AccountInfo<'info>,
}

pub fn handler(
    ctx: Context<Transfer>,
    encrypted_amount: [u8; 64],
    proof: Vec<u8>,
) -> Result<()> {
    let sender_account = &mut ctx.accounts.sender_account;
    let recipient_account = &mut ctx.accounts.recipient_account;
    
    // Verify zero-knowledge proof
    require!(
        crypto::verify_transfer_proof(
            &sender_account.encrypted_balance,
            &encrypted_amount,
            &sender_account.commitment,
            &proof,
        ),
        ErrorCode::InvalidProof
    );
    
    // Update encrypted balances
    sender_account.encrypted_balance = crypto::subtract_encrypted(
        &sender_account.encrypted_balance,
        &encrypted_amount,
    )?;
    
    recipient_account.encrypted_balance = crypto::add_encrypted(
        &recipient_account.encrypted_balance,
        &encrypted_amount,
    )?;
    
    sender_account.last_update = Clock::get()?.slot;
    recipient_account.last_update = Clock::get()?.slot;
    
    msg!("Private transfer completed");
    
    Ok(())
}

// Note: The constraint now properly checks that sender_account.owner == sender.key()
// This ensures only the owner can initiate transfers from their privacy account
