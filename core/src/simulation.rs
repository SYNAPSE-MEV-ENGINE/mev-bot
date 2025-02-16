// This module simulates transactions on a forked blockchain node.
// It provides functionality to execute and evaluate transactions without affecting the main network.

use ethers::{
    providers::Provider,
    types::{BlockId, Bytes, U256},
};
use revm::{
    db::{CacheDB, EmptyDB},
    primitives::{ExecutionResult, Output, TransactTo},
    EVM,
};

// Result structure containing the outcome of a transaction simulation.
#[derive(Debug)]
pub struct SimulationResult {
    pub gas_used: U256, // Amount of gas used during the simulation.
    pub success: bool, // Indicates if the simulation was successful.
    pub logs: Vec<ethers::types::Log>, // Logs generated during the simulation.
}

// Error types for simulation operations.
#[derive(Debug, thiserror::Error)]
pub enum SimulationError {
    #[error("EVM execution error: {0}")]
    ExecutionError(String),
    #[error("Provider error: {0}")]
    ProviderError(#[from] ethers::providers::ProviderError),
}

// ForkSimulator struct handles the simulation of transactions on a forked node.
pub struct ForkSimulator {
    provider: Provider<ethers::providers::Http>, // Provider for interacting with the blockchain.
}

impl ForkSimulator {
    // Initializes a new ForkSimulator instance with the provided provider.
    pub fn new(provider: Provider<ethers::providers::Http>) -> Self {
        Self { provider }
    }

    // Simulates a transaction on the forked node.
    pub async fn simulate(
        &self,
        tx: Bytes,
        block_number: Option<BlockId>,
    ) -> Result<SimulationResult, SimulationError> {
        // Retrieve the block number from the provider if not specified.
        let block = block_number.unwrap_or(BlockId::Number(ethers::types::BlockNumber::Latest));
        
        // Initialize a new EVM instance.
        let mut evm = EVM::new();
        
        // Set up the EVM database.
        evm.database(CacheDB::new(EmptyDB::default()));
        
        // Configure the EVM environment.
        evm.env.cfg.chain_id = self.provider.get_chainid().await?.as_u64();
        evm.env.block.number = self.provider.get_block_number().await?.as_u64().into();
        
        // Execute the transaction on the EVM.
        let result = evm.transact(TransactTo::Call(tx.to_vec().into()), tx.to_vec().into())
            .map_err(|e| SimulationError::ExecutionError(e.to_string()))?;

        // Return the simulation result.
        Ok(SimulationResult {
            gas_used: result.gas_used.into(),
            success: matches!(result.result, ExecutionResult::Success { output: Output::Call(_), .. }),
            logs: vec![]
        })
    }
}