use chrono::{DateTime, Utc};
use ethers::types::U256;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::time::{interval, Duration};
use log;

#[derive(Debug, Clone)]
pub struct Trade {
    pub timestamp: DateTime<Utc>,
    pub profit_loss: U256,
}

#[derive(Debug)]
pub struct RiskEngine {
    daily_loss_limit: U256,
    trades: Arc<RwLock<Vec<Trade>>>,
}

impl RiskEngine {
    pub fn new(daily_loss_limit: U256) -> Self {
        Self {
            daily_loss_limit,
            trades: Arc::new(RwLock::new(Vec::new())),
        }
    }

    pub async fn validate_trade(&self, profit_loss: U256) -> bool {
        let now = Utc::now();
        let trades = self.trades.read().await;
        
        // Calculate total losses for today
        let today_losses = trades
            .iter()
            .filter(|trade| {
                trade.timestamp.date_naive() == now.date_naive() && 
                trade.profit_loss < U256::zero()
            })
            .fold(U256::zero(), |acc, trade| acc + trade.profit_loss);

        // Check if new trade would exceed daily loss limit
        if today_losses + profit_loss > self.daily_loss_limit {
            return false;
        }

        true
    }

    pub async fn record_trade(&self, profit_loss: U256) {
        let trade = Trade {
            timestamp: Utc::now(),
            profit_loss,
        };

        let mut trades = self.trades.write().await;
        trades.push(trade);
    }
}

#[derive(Debug)]
pub struct CircuitBreaker {
    risk_engine: Arc<RiskEngine>,
    check_interval: Duration,
}

impl CircuitBreaker {
    pub fn new(daily_loss_limit: U256, check_interval: Duration) -> Self {
        Self {
            risk_engine: Arc::new(RiskEngine::new(daily_loss_limit)),
            check_interval,
        }
    }

    pub async fn check_trade(&self, profit_loss: U256) -> bool {
        self.risk_engine.validate_trade(profit_loss).await
    }

    pub async fn record_trade(&self, profit_loss: U256) {
        self.risk_engine.record_trade(profit_loss).await;
    }

    pub async fn run(&mut self) {
        let mut interval = interval(self.check_interval);
        
        loop {
            interval.tick().await;
            
            // Check daily losses against threshold using validate_trade
            if let false = self.risk_engine.validate_trade(U256::zero()).await {
                log::error!("Daily loss limit exceeded! Shutting down...");
                std::process::exit(1);
            }
        }
    }
}
