use ethers::{
    providers::{Http, Provider, Middleware},
    types::Chain,
};
use std::sync::Arc;

#[derive(Debug, thiserror::Error)]
pub enum BlockchainError {
    #[error("Provider error: {0}")]
    ProviderError(#[from] ethers::providers::ProviderError),
    #[error("Invalid chain ID: {0}")]
    InvalidChainId(u64),
    #[error("Network error: {0}")]
    NetworkError(String),
}

pub struct BlockchainConnection {
    provider: Arc<Provider<Http>>,
    chain: Chain,
}

impl BlockchainConnection {
    pub async fn new(rpc_url: &str, chain: Chain) -> Result<Self, BlockchainError> {
        let provider = Provider::<Http>::try_from(rpc_url)
            .map_err(|e| BlockchainError::NetworkError(e.to_string()))?;
        
        // Verify chain ID
        let chain_id = provider.get_chainid().await?.as_u64();
        match chain {
            Chain::PolygonMumbai if chain_id == 80001 => (),
            Chain::Polygon if chain_id == 137 => (),
            _ => return Err(BlockchainError::InvalidChainId(chain_id)),
        }

        Ok(Self {
            provider: Arc::new(provider),
            chain,
        })
    }

    pub fn provider(&self) -> Arc<Provider<Http>> {
        self.provider.clone()
    }

    pub fn chain(&self) -> Chain {
        self.chain
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::test;

    #[test]
    async fn test_local_provider() {
        let connection = BlockchainConnection::new(
            "http://localhost:8545",
            Chain::PolygonMumbai
        ).await.unwrap();
        
        let provider = connection.provider();
        let block = provider.get_block_number().await.unwrap();
        assert!(block.as_u64() > 0, "Should connect to local testnet");
    }

    #[test]
    async fn test_invalid_chain_id() {
        let result = BlockchainConnection::new(
            "http://localhost:8545",
            Chain::Mainnet // Wrong chain
        ).await;
        
        assert!(matches!(result, Err(BlockchainError::InvalidChainId(_))));
    }

    #[test]
    async fn test_invalid_url() {
        let result = BlockchainConnection::new(
            "http://invalid-url",
            Chain::PolygonMumbai
        ).await;
        
        assert!(matches!(result, Err(BlockchainError::NetworkError(_))));
    }

    #[test]
    async fn test_chain_accessor() {
        let connection = BlockchainConnection::new(
            "http://localhost:8545",
            Chain::PolygonMumbai
        ).await.unwrap();
        
        assert_eq!(connection.chain(), Chain::PolygonMumbai);
    }
}
