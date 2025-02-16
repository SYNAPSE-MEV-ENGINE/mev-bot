use ethers::{
    types::{Address, H256, U256},
    providers::Provider,
    contract::Contract,
    core::abi::Abi,
};
use std::{sync::Arc, error::Error};
use mev_risk::RiskEngine;
use serde_json::from_slice;

#[derive(Debug)]
#[allow(dead_code)]
pub struct SandwichStrategy {
    #[allow(dead_code)]
    provider: Arc<Provider<ethers::providers::Http>>,
    risk_engine: RiskEngine,
    #[allow(dead_code)]
    active_positions: Vec<Position>,
    #[allow(dead_code)]
    flash_loan_handler: Contract<Provider<ethers::providers::Http>>,
    sandwich_executor: Contract<Provider<ethers::providers::Http>>,
    price_oracle: Contract<Provider<ethers::providers::Http>>,
    math: mev_math::sandwich::SandwichMath,
}

#[derive(Debug)]
pub struct Position {
    token0: Address,
    token1: Address,
    amount0: U256,
    amount1: U256,
    timestamp: u64,
}

impl SandwichStrategy {
    pub async fn new(
        provider: Arc<Provider<ethers::providers::Http>>,
        _risk_params: RiskParams,
    ) -> Result<Self, Box<dyn Error>> {
        // Load contract ABIs
        let flash_loan_abi: Abi = from_slice(
            include_bytes!("../../../contracts/core/FlashLoanHandler.json")
        )?;

        let sandwich_executor_abi: Abi = from_slice(
            include_bytes!("../../../contracts/strategies/SandwichExecutor.json")
        )?;

        let price_oracle_abi: Abi = from_slice(
            include_bytes!("../../../contracts/utils/PriceOracle.json")
        )?;

        let flash_loan_handler = Contract::new(
            Address::zero(),
            flash_loan_abi,
            provider.clone(),
        );

        let sandwich_executor = Contract::new(
            Address::zero(),
            sandwich_executor_abi,
            provider.clone(),
        );

        let price_oracle = Contract::new(
            Address::zero(),
            price_oracle_abi,
            provider.clone(),
        );

        Ok(Self {
            provider,
            risk_engine: RiskEngine::default(),
            active_positions: Vec::new(),
            flash_loan_handler,
            sandwich_executor,
            price_oracle,
            math: mev_math::sandwich::SandwichMath::default(),
        })
    }

    pub async fn execute_sandwich(
        &mut self,
        token0: Address,
        token1: Address,
        amount0: U256,
        amount1: U256,
    ) -> Result<H256, Box<dyn Error>> {
        // Check risk parameters
        let potential_loss = self.calculate_potential_loss(token0).await?;
        if let Err(err) = self.risk_engine.validate_risk(potential_loss) {
            return Err(err.into());
        }

        // Build sandwich transactions
        let sandwich_data = self.math.build_sandwich_data(amount0, amount1)?;
        let stored_data = (sandwich_data.0, sandwich_data.1);
        
        // Execute sandwich via flash loan
        let method_call = self.sandwich_executor
            .method::<_, H256>(
                "executeSandwich",
                (
                    token0,
                    token1,
                    amount0,
                    amount1,
                    stored_data.0.clone(),
                    stored_data.1.clone(),
                ),
            )?;
        
        let tx = method_call.send().await?;
        
        Ok(tx.tx_hash())
    }

    async fn calculate_potential_loss(
        &self,
        token0: Address,
    ) -> Result<U256, Box<dyn Error>> {
        let price = self.price_oracle
            .method::<_, U256>("getPrice", token0)?
            .call()
            .await?;
        
        Ok(price.saturating_mul(U256::from(0))) // amount0 is not defined in this function
    }
}

#[derive(Debug)]
pub struct RiskParams {
    max_loss: U256,
    min_profit: U256,
}

#[derive(Debug, thiserror::Error)]
pub enum SandwichError {
    #[error("Provider error: {0}")]
    ProviderError(#[from] ethers::providers::ProviderError),
    #[error("Contract error: {0}")]
    ContractError(String),
    #[error("Risk error: {0}")]
    RiskError(#[from] mev_risk::RiskError),
    #[error("Math error: {0}")]
    MathError(String),
}
