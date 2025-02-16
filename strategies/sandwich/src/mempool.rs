use ethers::{providers::Middleware, types::Transaction};
use parking_lot::Mutex;
use std::{sync::Arc, time::Duration};

pub struct MempoolWatcher<M: Middleware> {
    provider: Arc<M>,
    pending_txs: Arc<Mutex<Vec<Transaction>>>,
}

impl<M: Middleware> MempoolWatcher<M> {
    pub fn new(provider: Arc<M>) -> Self {
        Self {
            provider,
            pending_txs: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub async fn start(&self) {
        let provider = self.provider.clone();
        let pending_txs = self.pending_txs.clone();
        
        tokio::spawn(async move {
            loop {
                if let Ok(txs) = provider.pending_transactions().await {
                    let mut guard = pending_txs.lock();
                    *guard = txs.into_iter()
                        .filter(|tx| tx.to == Some("0x1b02dA8Cb0d097eB8D57A175b88c7D8b47997506".parse().unwrap()))
                        .collect();
                }
                tokio::time::sleep(Duration::from_millis(50)).await;
            }
        });
    }
}
