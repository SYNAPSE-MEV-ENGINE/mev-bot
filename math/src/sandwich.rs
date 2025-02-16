// This module contains mathematical functions related to sandwich attacks in Uniswap V3.
// It provides structures and methods for calculating reserves and validating trades.

use ethers::types::{Transaction, U256};
use mev_risk::{RiskError, RiskParameters};

/// Constant used for reserve calculations, representing 2^96.
const Q96: u128 = 0x1000000000000000000000000;

/// Structure representing a Uniswap V3 pool.
#[derive(Debug, Clone)]
pub struct UniswapV3Pool {
    /// Square root of the price in x96 format.
    pub sqrt_price_x96: U256,
    /// Liquidity of the pool.
    pub liquidity: U256,
    /// Fee tier of the pool.
    pub fee: u32,
}

impl UniswapV3Pool {
    /// Calculates the reserves of the pool based on liquidity and price.
    ///
    /// Returns a tuple of (reserve0, reserve1) as U256.
    pub fn get_reserves(&self) -> (U256, U256) {
        let sqrt_price = self.sqrt_price_x96.as_u128();
        let liquidity = self.liquidity.as_u128();

        // Calculate reserve0 by multiplying liquidity with Q96 and dividing by sqrt_price.
        let reserve0 = (liquidity * Q96) / sqrt_price;
        // Calculate reserve1 by multiplying liquidity with sqrt_price and dividing by Q96.
        let reserve1 = (liquidity * sqrt_price) / Q96;

        (U256::from(reserve0), U256::from(reserve1)) // Return reserves as U256.
    }
}

/// Structure for sandwich attack calculations and risk management.
#[derive(Debug)]
pub struct SandwichMath {
    /// The Uniswap V3 pool associated with the calculations.
    pub pool: UniswapV3Pool,
    /// Risk parameters for trade validation.
    pub risk_params: RiskParameters,
    /// Risk engine for managing trade risks.
    pub risk_engine: mev_risk::RiskEngine,
}

impl SandwichMath {
    /// Extracts the input amount from a transaction.
    ///
    /// Returns the input amount as U256 if successful, or None if the transaction input is invalid.
    pub fn extract_input_amount(&self, tx: &Transaction) -> Option<U256> {
        // Get first 32 bytes of input (typical parameter size).
        let bytes = tx.input.get(0..32)?;

        // Convert byte slice to U256 using big-endian encoding.
        Some(U256::from_big_endian(bytes))
    }

    /// Validates the risk of a trade based on the input amount.
    ///
    /// Returns Ok(()) if the trade is valid, or Err(RiskError) if the trade is invalid.
    pub fn validate_risk(&self, amount: U256) -> Result<(), RiskError> {
        self.risk_engine.validate_trade(amount, U256::zero()) // Validate trade with zero output.
    }

    /// Calculates the optimal size for a trade based on the target transaction.
    ///
    /// Returns the optimal size as U256 if successful, or Err(MathError) if the calculation fails.
    pub fn calculate_optimal_size(&self, target_tx: &Transaction) -> Result<U256, MathError> {
        // Extract input amount from the target transaction.
        let input_amount = self
            .extract_input_amount(target_tx)
            .ok_or(MathError::InvalidTransaction)?; // Extract input amount or return error.

        // Calculate the maximum position using the risk engine.
        self.risk_engine
            .calculate_max_position(input_amount)
            .map_err(MathError::RiskValidationFailed)
    }
}

/// Error types for mathematical operations.
#[derive(Debug, thiserror::Error)]
pub enum MathError {
    /// Invalid transaction input.
    #[error("Invalid transaction")]
    InvalidTransaction,
    /// Arithmetic overflow occurred during calculation.
    #[error("Arithmetic overflow")]
    Overflow,
    /// Trade does not meet profit requirements.
    #[error("Trade doesn't meet profit requirements")]
    UnprofitableTrade,
    /// Risk validation failed.
    #[error("Risk validation failed")]
    RiskValidationFailed(#[from] RiskError),
}
