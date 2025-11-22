use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    /// Direct transfers between wallets are not allowed for pSOL.
    #[msg("Direct pSOL transfers are disabled")]
    TransfersDisabled,

    /// The config account or its fields are inconsistent.
    #[msg("Invalid pSOL configuration")]
    InvalidConfig,

    /// The pool account passed does not match the pool configured
    /// in the pSOL token config.
    #[msg("Pool mismatch")]
    InvalidPool,

    /// The pSOL token program is being called with an unexpected
    /// psol program id or other program id mismatch.
    #[msg("Program mismatch")]
    InvalidProgram,

    /// The pSOL mint must be controlled by the PDA configured
    /// in the program; any other mint authority is rejected.
    #[msg("Mint authority must be the PDA")]
    InvalidMintAuthority,

    // Generic helpers the token program might also use
    #[msg("Invalid amount")]
    InvalidAmount,

    #[msg("Insufficient balance")]
    InsufficientBalance,

    #[msg("Arithmetic overflow")]
    ArithmeticOverflow,

    #[msg("Unauthorized")]
    Unauthorized,
}
