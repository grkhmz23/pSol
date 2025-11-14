use anchor_lang::prelude::*;

declare_id!("11111111111111111111111111111111"); // Will update after deployment

pub mod state;
pub mod instructions;
pub mod error;

use instructions::*;

#[program]
pub mod psol_token {
    use super::*;

    /// Initialize the token system
    pub fn initialize_token(ctx: Context<InitializeToken>) -> Result<()> {
        instructions::initialize_token::handler(ctx)
    }

    /// Swap SOL to pSOL
    pub fn swap_to_psol(ctx: Context<SwapToPsol>, amount: u64) -> Result<()> {
        instructions::swap_to_psol::handler(ctx, amount)
    }

    /// Swap pSOL to SOL
    pub fn swap_to_sol(
        ctx: Context<SwapToSol>,
        amount: u64,
        nullifier: [u8; 32],
    ) -> Result<()> {
        instructions::swap_to_sol::handler(ctx, amount, nullifier)
    }

    /// Transfer pSOL privately
    pub fn transfer_psol(
        ctx: Context<TransferPsol>,
        amount: u64,
        encrypted_amount: [u8; 64],
    ) -> Result<()> {
        instructions::transfer_psol::handler(ctx, amount, encrypted_amount)
    }
}
