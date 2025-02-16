use ethers::{
    providers::{Provider, Middleware},
    types::{Bytes, BlockId, H160, U256},
};
use revm::{
    db::{CacheDB, EmptyDB},
    primitives::{Address as rAddress, U256 as rU256, TransactTo},
    EVM,
};
use std::convert;
use std::sync::Arc;

#[derive(Debug, PartialEq)]
pub struct SimulationResult {
    pub gas_used: U256,
    pub success: bool,
    pub logs: Vec<String>,
}

#[derive(Debug, thiserror::Error)]
pub enum SimulationError {
    #[error("EVM execution error: {0}")]
    ExecutionError(String),
    #[error("Provider error: {0}")]
    ProviderError(#[from] ethers::providers::ProviderError),
}

pub struct ForkSimulator {
    provider: Provider<ethers::providers::Http>,
}

impl ForkSimulator {
    pub fn new(provider: Provider<ethers::providers::Http>) -> Self {
        Self { provider }
    }

    pub async fn simulate(
        &self,
        tx: Bytes,
        _block_number: Option<BlockId>,
    ) -> Result<SimulationResult, SimulationError> {
        let mut evm = EVM::new();
        
        evm.database(CacheDB::new(EmptyDB::default()));
        
        evm.env.cfg.chain_id = self.provider.get_chainid().await?.as_u64();
        evm.env.block.number = rU256::from(self.provider.get_block_number().await?.as_u64());
        
        let to_addr = H160::from_slice(&tx[..20]);
        let data = tx[20..].to_vec();
        
        let to_revm_addr = rAddress::from_slice(to_addr.as_bytes());
        
        evm.env.tx.gas_limit = 1_000_000;
        evm.env.tx.data = data.into();
        evm.env.tx.caller = rAddress::repeat_byte(0);
        evm.env.tx.transact_to = TransactTo::Call(to_revm_addr);
        
        match evm.transact() {
            Ok(result) => Ok(SimulationResult {
                gas_used: U256::from(result.result.gas_used()),
                success: !result.result.is_success(),
                logs: vec![]
            }),
            Err(e) => Err(SimulationError::ExecutionError(e.to_string()))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;
    use tokio::test;

    fn create_test_provider() -> Provider<ethers::providers::Http> {
        Provider::<ethers::providers::Http>::try_from("http://localhost:8545").unwrap()
    }

    fn create_test_transaction() -> Bytes {
        let mut tx = vec![0u8; 84];
        let addr = H160::from_str("0x742d35Cc6634C0532925a3b844Bc454e4438f44e").unwrap();
        tx[..20].copy_from_slice(addr.as_bytes());
        tx[20..].copy_from_slice(&[1u8; 64]);
        Bytes::from(tx)
    }

    #[test]
    async fn test_simulation_success() {
        let provider = create_test_provider();
        let simulator = ForkSimulator::new(provider);
        let tx = create_test_transaction();

        let result = simulator.simulate(tx, None).await.unwrap();
        assert!(result.success, "Transaction simulation should succeed");
        assert!(result.gas_used > U256::zero(), "Gas used should be non-zero");
    }

    #[test]
    async fn test_invalid_transaction() {
        let provider = create_test_provider();
        let simulator = ForkSimulator::new(provider);
        let tx = Bytes::from(vec![0u8; 10]);

        let result = simulator.simulate(tx, None).await;
        assert!(matches!(result, Err(SimulationError::ExecutionError(_))));
    }

    #[test]
    async fn test_gas_limit() {
        let provider = create_test_provider();
        let simulator = ForkSimulator::new(provider);
        let tx = create_test_transaction();

        let result = simulator.simulate(tx, None).await.unwrap();
        assert!(result.gas_used <= U256::from(1_000_000), "Gas used should not exceed limit");
    }
}