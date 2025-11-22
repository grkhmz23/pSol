use anchor_lang::prelude::*;

pub mod crypto;
pub mod error;
pub mod instructions;
pub mod state;

pub use error::ErrorCode;

declare_id!("2dJdyxoGmAoJLsZh7h8ma8xeyoaj7uiHFgrsgUAQMojv");

#[program]
pub mod psol {
    use super::*;

    pub fn initialize_pool(ctx: Context<instructions::InitializePool>, fee_bps: u16) -> Result<()> {
        instructions::initialize_pool(ctx, fee_bps)
    }

    pub fn init_privacy_account(ctx: Context<instructions::InitPrivacyAccount>) -> Result<()> {
        instructions::init_privacy_account(ctx)
    }

    pub fn deposit_private(
        ctx: Context<instructions::DepositPrivate>,
        amount: u64,
        nonce: u64,
    ) -> Result<()> {
        instructions::deposit_private(ctx, amount, nonce)
    }

    pub fn transfer_private(
        ctx: Context<instructions::TransferPrivate>,
        amount: u64,
        nullifier: [u8; 32],
        nonce: u64,
    ) -> Result<()> {
        instructions::transfer_private(ctx, amount, nullifier, nonce)
    }

    pub fn withdraw_private(
        ctx: Context<instructions::WithdrawPrivate>,
        amount: u64,
        nullifier: [u8; 32],
    ) -> Result<()> {
        instructions::withdraw_private(ctx, amount, nullifier)
    }

    pub fn admin_set_fees(ctx: Context<instructions::AdminSetFees>, fee_bps: u16) -> Result<()> {
        instructions::admin_set_fees(ctx, fee_bps)
    }

    pub fn admin_pause(ctx: Context<instructions::AdminPause>) -> Result<()> {
        instructions::admin_pause(ctx)
    }

    pub fn admin_unpause(ctx: Context<instructions::AdminUnpause>) -> Result<()> {
        instructions::admin_unpause(ctx)
    }
}