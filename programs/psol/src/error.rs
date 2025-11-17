use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Protocol is paused")]
    ProtocolPaused,
    
    #[msg("Invalid amount")]
    InvalidAmount,
    
    #[msg("Insufficient balance")]
    InsufficientBalance,
    
    #[msg("Invalid proof")]
    InvalidProof,
    
    #[msg("Nullifier already used")]
    NullifierUsed,
    
    #[msg("Invalid encryption key")]
    InvalidEncryptionKey,
    
    #[msg("Arithmetic overflow")]
    ArithmeticOverflow,
    
    #[msg("Unauthorized")]
    Unauthorized,
    
    #[msg("Invalid fee")]
    InvalidFee,
}
