// This module monitors the mempool for pending transactions.
// It provides functionality to receive and process transactions from the mempool.

use ethers::{
    providers::{Provider, Ws, Middleware}, // Import Middleware trait for mempool operations.
    types::Transaction,
};
use thiserror::Error;
use tokio::sync::mpsc;
use futures_util::stream::StreamExt; 

// Error types for mempool operations.
#[derive(Error, Debug)]
pub enum MempoolError {
    #[error("WebSocket connection failed: {0}")]
    ConnectionFailure(String),
    #[error("Transaction processing error")]
    ProcessingError,
}

// MempoolWatcher struct watches for pending transactions in the mempool.
pub struct MempoolWatcher {
    pub tx_receiver: mpsc::Receiver<Transaction>, 
}

impl MempoolWatcher {
    // Initializes a new MempoolWatcher instance with the provided WebSocket URL.
    pub async fn new(ws_url: &str) -> Result<Self, MempoolError> {
        // Create a provider for connecting to the mempool.
        let provider = Provider::<Ws>::connect(ws_url)
            .await
            .map_err(|e| MempoolError::ConnectionFailure(e.to_string()))?;

        // Create a channel for receiving transactions.
        let (tx_sender, tx_receiver) = mpsc::channel(100);

        // Spawn a task to monitor pending transactions.
        tokio::spawn(async move {
            let mut stream = provider.subscribe_pending_txs().await.unwrap();
            while let Some(tx_hash) = stream.next().await {
                if let Ok(Some(tx)) = provider.get_transaction(tx_hash).await {
                    let _ = tx_sender.send(tx).await; 
                }
            }
        });

        Ok(Self { tx_receiver })
    }
}

// Function to initialize the mempool module.
pub fn init_mempool() {
    println!("Mempool module initialized");
}
