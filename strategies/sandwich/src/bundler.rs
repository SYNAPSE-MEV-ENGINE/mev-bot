// This module handles the bundling of transactions for MEV strategies.
// It includes functionalities for constructing and submitting transaction bundles.

use ethers::{
    prelude::*,
    types::{Transaction, U256, transaction::eip2718::TypedTransaction},
};
use mev_rs::relay::Relay;
use mev_rs::{BlobBundle, BlobTransaction, SigningWallet};
use parking_lot::Mutex;
use std::{sync::Arc, time::Duration};

// Constants for gas buffer percentage and maximum gas price deviation.
const GAS_BUFFER_PERCENT: u64 = 10;
const MAX_GAS_PRICE_DEVIATION: f64 = 0.25;
const POLYGON_FLASHBOTS_RELAY: &str = "https://polygon-relay.flashbots.net";

// Error types for bundling operations.
#[derive(Debug, thiserror::Error)]
pub enum BundleError {
    #[error("Signing error: {0}")]
    SigningError(#[from] ethers::signers::WalletError),
    #[error("Relay error: {0}")]
    RelayError(String),
}

// Structure representing gas parameters for transactions.
#[derive(Clone, Debug)]
struct GasParameters {
    base_fee: U256,
    priority_fee: U256,
    gas_limit: U256,
}

// The BundleConstructor struct is responsible for creating and managing transaction bundles.
pub struct BundleConstructor {
    relay: Relay,
    wallet: LocalWallet,
    provider: Provider<Http>,
    gas_cache: Arc<Mutex<GasParameters>>, // Caches gas parameters for efficiency.
}

impl BundleConstructor {
    // Initializes a new BundleConstructor instance with the provided RPC URL.
    pub async fn new(rpc_url: &str) -> Result<Self, Box<dyn std::error::Error>> {
        // Create a new relay and provider for transaction management.
        let relay = Relay::new(rpc_url);
        let provider = Provider::<Http>::try_from(rpc_url)?;
        let wallet = LocalWallet::new(&mut rand::thread_rng());

        // Fetch gas parameters for transaction optimization.
        let gas_params = Self::fetch_gas_parameters(&provider).await?;

        Ok(Self {
            relay,
            wallet,
            provider,
            gas_cache: Arc::new(Mutex::new(gas_params)),
        })
    }

    // Function to fetch gas parameters from the provider.
    async fn fetch_gas_parameters(provider: &Provider<Http>) -> Result<GasParameters, ProviderError> {
        // Get the latest block from the provider.
        let block = provider.get_block(BlockNumber::Latest).await?.unwrap();
        let base_fee = block.base_fee_per_gas.unwrap_or_default();
        let gas_limit = block.gas_limit;

        // Estimate EIP-1559 fees for priority fee calculation.
        let priority_fee = provider.estimate_eip1559_fees(None).await?.max_priority_fee_per_gas;

        Ok(GasParameters {
            base_fee,
            priority_fee,
            gas_limit,
        })
    }

    // Function to calculate dynamic gas price based on cached gas parameters.
    fn calculate_dynamic_gas(&self) -> U256 {
        let cache = self.gas_cache.lock();
        let max_fee = cache.base_fee * (100 + GAS_BUFFER_PERCENT) / 100;
        max_fee + cache.priority_fee
    }

    // Function to build sandwich transactions.
    // This function constructs the necessary transactions for executing a sandwich attack.
    pub async fn build_sandwich_txs(&self, amount: U256) -> (TransactionRequest, TransactionRequest) {
        // Get the current nonce for the wallet.
        let nonce = self.provider.get_transaction_count(self.wallet.address(), None)
            .await
            .unwrap();

        // Calculate dynamic gas price and gas limit for transactions.
        let gas_price = self.calculate_dynamic_gas();
        let gas_limit = self.gas_cache.lock().gas_limit * 105 / 100;

        // Construct the frontrun and backrun transactions.
        let frontrun = TransactionRequest::new()
            .to(self.pool_address)
            .data(encode_swap(amount))
            .nonce(nonce)
            .gas_price(gas_price)
            .gas(gas_limit);

        let backrun = TransactionRequest::new()
            .to(self.pool_address)
            .data(encode_swap(amount))
            .nonce(nonce + 1)
            .gas_price(gas_price * 90 / 100)
            .gas(gas_limit);

        (frontrun, backrun)
    }
}

// Structure representing a MEV bundle.
pub struct MevBundle {
    pub transactions: Vec<TypedTransaction>,
    pub block_number: U256,
    pub min_timestamp: u64,
    pub max_timestamp: u64,
}

impl MevBundle {
    // Function to submit the transaction bundle to the relay.
    // This function takes a wallet for signing the bundle.
    pub async fn submit_to_relay(
        self,
        wallet: &LocalWallet,
    ) -> Result<(), BundleError> {
        // Create a new relay instance for submitting the bundle.
        let relay = Relay::new(POLYGON_FLASHBOTS_RELAY);
        let signer = SigningWallet::from(wallet.clone());

        // Convert transactions to blob transactions for relay submission.
        let blob_txs = self.transactions
            .into_iter()
            .map(|tx| BlobTransaction::from(tx.rlp()))
            .collect();

        // Create a blob bundle for submission.
        let blob_bundle = BlobBundle {
            transactions: blob_txs,
            block_number: self.block_number,
            min_timestamp: self.min_timestamp,
            max_timestamp: self.max_timestamp,
        };

        // Submit the bundle to the relay.
        relay.send_bundle(&signer, blob_bundle)
            .await
            .map_err(|e| BundleError::RelayError(e.to_string()))?;
        Ok(())
    }
}
