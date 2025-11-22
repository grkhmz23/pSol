use crate::crypto;
use crate::error::ErrorCode;
use crate::state::*;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct TransferPrivate<'info> {
    #[account(
        seeds = [b"privacy_pool"],
        bump = pool.bump
    )]
    pub pool: Account<'info, PrivacyPool>,

    #[account(
        mut,
        seeds = [b"privacy_account", sender.key().as_ref()],
        bump = sender_account.bump,
        constraint = sender_account.owner == sender.key() @ ErrorCode::Unauthorized
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
    ctx: Context<TransferPrivate>,
    encrypted_amount: [u8; 64],
    proof: Vec<u8>,
) -> Result<()> {
    let sender_account = &mut ctx.accounts.sender_account;
    let recipient_account = &mut ctx.accounts.recipient_account;

    require!(
        crypto::verify_transfer_proof(
            &sender_account.encrypted_balance,
            &encrypted_amount,
            &sender_account.commitment,
            &proof,
        ),
        ErrorCode::InvalidProof
    );

    sender_account.encrypted_balance =
        crypto::subtract_encrypted(&sender_account.encrypted_balance, &encrypted_amount)?;

    recipient_account.encrypted_balance =
        crypto::add_encrypted(&recipient_account.encrypted_balance, &encrypted_amount)?;

    sender_account.last_update = Clock::get()?.slot;
    recipient_account.last_update = Clock::get()?.slot;

    msg!("Private transfer completed");
    Ok(())
}
