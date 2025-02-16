use ethers::{
    providers::{Http, Provider, Middleware}, 
    types::{Chain, U256},
};
use std::sync::Arc;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum BlockchainError {
    #[error("Provider initialization failed: {0}")]
    ProviderInitialization(String),
    #[error("Network mismatch: expected {expected:?}, got {actual:?}")]
    NetworkMismatch { expected: Chain, actual: Chain },
}

pub async fn init_blockchain(
    rpc_url: Option<&str>,
) -> Result<Arc<Provider<Http>>, BlockchainError> {
    let url = rpc_url.unwrap_or("http://localhost:8545");
    let provider = Provider::<Http>::try_from(url)
        .map_err(|e| BlockchainError::ProviderInitialization(e.to_string()))?;

    let chain_id = provider.get_chainid().await.map_err(|e| {
        BlockchainError::ProviderInitialization(format!("Chain ID check failed: {}", e))
    })?;

    let expected = Chain::PolygonMumbai;
    if chain_id != U256::from(expected as u64) { // Convert expected to U256 for comparison.

        return Err(BlockchainError::NetworkMismatch {
            expected,
            actual: Chain::try_from(chain_id.as_u64()).unwrap_or(Chain::PolygonMumbai),
        });
    }

    provider.get_block_number().await.map_err(|e| {
        BlockchainError::ProviderInitialization(format!("Health check failed: {}", e))
    })?;

    Ok(Arc::new(provider))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_local_provider() {
        let provider = init_blockchain(None).await.unwrap();
        let block = provider.get_block_number().await.unwrap();
        assert!(block > 0, "Should connect to local testnet");
    }
}
