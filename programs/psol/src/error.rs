use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Amount must be greater than zero.")]
    InvalidAmount,

    #[msg("Unauthorized.")]
    Unauthorized,

    #[msg("Fee basis points too high.")]
    FeeTooHigh,

    #[msg("Vault PDA mismatch.")]
    InvalidVault,

    #[msg("Registry PDA mismatch.")]
    InvalidRegistry,

    #[msg("Commitment registry full.")]
    CommitmentRegistryFull,

    #[msg("Nullifier already used.")]
    NullifierAlreadyUsed,

    #[msg("Pool is paused.")]
    PoolPaused,

    // Some files may use this older name — keep both to avoid edits elsewhere.
    #[msg("Protocol is paused.")]
    ProtocolPaused,

    #[msg("Insufficient balance.")]
    InsufficientBalance,

    // Keep both names because different files reference different ones.
    #[msg("Arithmetic overflow.")]
    MathOverflow,

    #[msg("Arithmetic overflow.")]
    ArithmeticOverflow,

    #[msg("Amount too small.")]
    AmountTooSmall,

    #[msg("Invalid proof.")]
    InvalidProof,

    #[msg("Reentrancy detected.")]
    ReentrancyDetected,
}
