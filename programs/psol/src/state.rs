use anchor_lang::prelude::*;

/// Global privacy pool state
#[account]
pub struct PrivacyPool {
    /// Authority for the pool
    pub authority: Pubkey,
    
    /// SOL vault holding all deposited SOL
    pub vault: Pubkey,
    
    /// Total SOL locked in pool
    pub total_locked: u64,
    
    /// Total privacy accounts created
    pub total_accounts: u64,
    
    /// Deposit fee in basis points (100 = 1%)
    pub deposit_fee_bps: u16,
    
    /// Withdraw fee in basis points
    pub withdraw_fee_bps: u16,
    
    /// Protocol paused
    pub paused: bool,
    
    /// Bump for PDA
    pub bump: u8,
}

impl PrivacyPool {
    pub const SIZE: usize = 8 + // discriminator
        32 + // authority
        32 + // vault
        8 +  // total_locked
        8 +  // total_accounts
        2 +  // deposit_fee_bps
        2 +  // withdraw_fee_bps
        1 +  // paused
        1;   // bump
}

/// User's private account
#[account]
pub struct PrivacyAccount {
    /// Owner of this account
    pub owner: Pubkey,
    
    /// Encrypted balance (ElGamal ciphertext)
    pub encrypted_balance: [u8; 64],
    
    /// Public key for encryption
    pub encryption_key: [u8; 32],
    
    /// Commitment for zero-knowledge proofs
    pub commitment: [u8; 32],
    
    /// Last update slot
    pub last_update: u64,
    
    /// Total deposits
    pub total_deposits: u64,
    
    /// Total withdrawals
    pub total_withdrawals: u64,
    
    /// Bump for PDA
    pub bump: u8,
}

impl PrivacyAccount {
    pub const SIZE: usize = 8 +   // discriminator
        32 +  // owner
        64 +  // encrypted_balance
        32 +  // encryption_key
        32 +  // commitment
        8 +   // last_update
        8 +   // total_deposits
        8 +   // total_withdrawals
        1;    // bump
}

/// Nullifier set to prevent double-spending
#[account]
pub struct NullifierSet {
    /// The pool this belongs to
    pub pool: Pubkey,
    
    /// Nullifier hash
    pub nullifier: [u8; 32],
    
    /// Slot when created
    pub slot: u64,
    
    /// Bump for PDA
    pub bump: u8,
}

impl NullifierSet {
    pub const SIZE: usize = 8 +   // discriminator
        32 +  // pool
        32 +  // nullifier
        8 +   // slot
        1;    // bump
}
