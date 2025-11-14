use anchor_lang::prelude::*;

declare_id!("2dJdyxoGmAoJLsZh7h8ma8xeyoaj7uiHFgrsgUAQMojv");

pub mod state;
pub mod instructions;
pub mod error;
pub mod crypto;

use instructions::*;

#[program]
pub mod psol {
    use super::*;

    /// Initialize the privacy pool
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        instructions::initialize::handler(ctx)
    }

    /// Wrap SOL into privacy pool
    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        instructions::deposit::handler(ctx, amount)
    }

    /// Unwrap SOL from privacy pool
    pub fn withdraw(
        ctx: Context<Withdraw>,
        amount: u64,
        nullifier: [u8; 32],
        proof: Vec<u8>,
    ) -> Result<()> {
        instructions::withdraw::handler(ctx, amount, nullifier, proof)
    }

    /// Transfer within privacy pool
    pub fn transfer(
        ctx: Context<Transfer>,
        encrypted_amount: [u8; 64],
        proof: Vec<u8>,
    ) -> Result<()> {
        instructions::transfer::handler(ctx, encrypted_amount, proof)
    }
}
