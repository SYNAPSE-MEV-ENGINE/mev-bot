use ethers::types::U256;
use mev_risk::RiskEngine;
use tokio::time::{interval, Duration};

pub struct CircuitBreaker {
    risk_engine: RiskEngine,
    check_interval: Duration,
}

impl CircuitBreaker {
    pub fn new(risk_engine: RiskEngine) -> Self {
        Self {
            risk_engine,
            check_interval: Duration::from_secs(60),
        }
    }

    pub async fn run(&mut self) {
        let mut interval = interval(self.check_interval);
        
        loop {
            interval.tick().await;
            
            if self.risk_engine.daily_loss_exceeded() {
                log::error!("Daily loss limit exceeded! Shutting down...");
                std::process::exit(1);
            }
        }
    }
}
