#[derive(Debug, thiserror::Error)]
pub enum MathError {
    #[error("Invalid transaction input")]
    InvalidTransaction,
    #[error("Risk validation failed: {0}")]
    RiskValidationFailed(#[from] crate::risk::Error),
    #[error("Arithmetic overflow")]
    Overflow,
}
