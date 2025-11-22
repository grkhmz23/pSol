use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Invalid amount.")]
    InvalidAmount,

    #[msg("Insufficient balance.")]
    InsufficientBalance,

    #[msg("Invalid proof.")]
    InvalidProof,

    #[msg("Arithmetic overflow or underflow.")]
    ArithmeticOverflow,

    #[msg("Protocol is paused.")]
    ProtocolPaused,

    #[msg("Recipient is invalid.")]
    InvalidRecipient,

    #[msg("Unauthorized.")]
    Unauthorized,

    #[msg("Invalid authority.")]
    InvalidAuthority,

    #[msg("Nullifier already used.")]
    NullifierAlreadyUsed,

    #[msg("Reentrancy detected.")]
    ReentrancyDetected,

    #[msg("Operation not yet supported.")]
    NotYetSupported,

    #[msg("Invalid encryption key.")]
    InvalidEncryptionKey,

    #[msg("Invalid commitment.")]
    InvalidCommitment,

    #[msg("Invalid nullifier.")]
    InvalidNullifier,
}
