use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Protocol is paused")]
    ProtocolPaused,
    
    #[msg("Invalid amount")]
    InvalidAmount,
    
    #[msg("Insufficient balance")]
    InsufficientBalance,
    
    #[msg("Arithmetic overflow")]
    ArithmeticOverflow,
    
    #[msg("Invalid token mint")]
    InvalidTokenMint,
    
    #[msg("Unauthorized")]
    Unauthorized,
}
