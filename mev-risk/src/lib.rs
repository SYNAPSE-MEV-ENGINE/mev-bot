#![allow(unused)]

use ethers::types::U256;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct RiskParameters {
    pub max_position_size: U256,
    pub max_loss_percent: u8,
    pub min_profit_ratio: f64,
}

impl Default for RiskParameters {
    fn default() -> Self {
        Self {
            max_position_size: U256::from(500_000u64),
            max_loss_percent: 3,
            min_profit_ratio: 1.2,
        }
    }
}

#[derive(Debug, Default)]
pub struct RiskEngine {
    parameters: RiskParameters,
    daily_losses: HashMap<u64, U256>,
}

#[derive(Debug, thiserror::Error)]
pub enum RiskError {
    #[error("Position size exceeded")]
    PositionSizeExceeded,
    #[error("Daily loss limit exceeded")]
    DailyLossLimitExceeded,
    #[error("Position size exceeds maximum allowed")]
    PositionTooLarge,
    #[error("Insufficient profit margin")]
    InsufficientProfit,
    #[error("Volatility threshold exceeded")]
    VolatilityThreshold,
    #[error("Invalid risk parameters")]
    InvalidParameters,
}

impl RiskEngine {
    pub fn validate_trade(&self, size: U256, potential_loss: U256) -> Result<(), RiskError> {
        if size > self.parameters.max_position_size {
            return Err(RiskError::PositionSizeExceeded);
        }
        Ok(())
    }

    pub fn calculate_max_position(&self, input_amount: U256) -> Result<U256, RiskError> {
        let max_position = self.parameters.max_position_size;
        if input_amount > max_position {
            Err(RiskError::PositionTooLarge)
        } else {
            Ok(input_amount)
        }
    }
}
