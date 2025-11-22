use anchor_lang::prelude::*;

pub mod crypto;
pub mod error;
pub mod instructions;
pub mod state;

pub use instructions::*;
pub use state::*;
pub use error::ErrorCode;

// keep your current program id here
declare_id!("2dJdyxoGmAoJLsZh7h8ma8xeyoaj7uiHFgrsgUAQMojv");

#[program]
pub mod psol {
    use super::*;

    pub fn initialize_pool(ctx: Context<InitializePool>, fee_bps: u16) -> Result<()> {
        instructions::initialize_pool::handler(ctx, fee_bps)
    }

    pub fn init_privacy_account(
        ctx: Context<InitPrivacyAccount>,
        encryption_key: [u8; 32],
    ) -> Result<()> {
        instructions::init_privacy_account::handler(ctx, encryption_key)
    }

    pub fn deposit_private(
        ctx: Context<DepositPrivate>,
        amount: u64,
        encrypted_amount: [u8; 64],
        proof: Vec<u8>,
    ) -> Result<()> {
        instructions::deposit_private::handler(ctx, amount, encrypted_amount, proof)
    }

    pub fn withdraw_private(
        ctx: Context<WithdrawPrivate>,
        amount: u64,
        nullifier: [u8; 32],
        proof: Vec<u8>,
    ) -> Result<()> {
        instructions::withdraw_private::handler(ctx, amount, nullifier, proof)
    }

    pub fn transfer_private(
        ctx: Context<TransferPrivate>,
        encrypted_amount: [u8; 64],
        proof: Vec<u8>,
    ) -> Result<()> {
        instructions::transfer_private::handler(ctx, encrypted_amount, proof)
    }

    pub fn admin_pause(ctx: Context<AdminPause>) -> Result<()> {
        instructions::admin_pause::handler(ctx)
    }

    pub fn admin_unpause(ctx: Context<AdminUnpause>) -> Result<()> {
        instructions::admin_unpause::handler(ctx)
    }

    pub fn admin_set_fees(ctx: Context<AdminSetFees>, fee_bps: u16) -> Result<()> {
        instructions::admin_set_fees::handler(ctx, fee_bps)
    }
}
