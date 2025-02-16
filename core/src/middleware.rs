use ethers::{
    middleware::{
        nonce_manager::NonceManagerMiddleware,
        SignerMiddleware,
    },
    prelude::*,
    providers::{Http, Provider},
    signers::LocalWallet,
};
use std::sync::Arc;

/// FlashBotMiddleware represents our custom middleware stack for MEV transactions
/// It includes:
/// - NonceManager: For handling nonce management across concurrent transactions
/// - SignerMiddleware: For transaction signing
pub type FlashBotMiddleware = SignerMiddleware<
    NonceManagerMiddleware<Provider<Http>>,
    LocalWallet
>;

/// Creates a middleware stack optimized for MEV transactions
pub async fn create_middleware_stack(
    provider: Arc<Provider<Http>>,
    wallet: LocalWallet,
) -> Result<Arc<FlashBotMiddleware>, Box<dyn std::error::Error>> {
    // Set up nonce manager
    let nonce_manager = NonceManagerMiddleware::new(
        Arc::try_unwrap(provider)
            .unwrap_or_else(|arc| (*arc).clone()),
        wallet.address(),
    );

    // Set up signer
    Ok(Arc::new(SignerMiddleware::new(
        nonce_manager,
        wallet,
    )))
}

/// Creates a dedicated signer for Flashbots bundles
pub async fn create_flashbots_bundle_signer(
    provider: Arc<Provider<Http>>,
    bundle_signer: LocalWallet,
) -> Result<Arc<SignerMiddleware<Provider<Http>, LocalWallet>>, Box<dyn std::error::Error>> {
    Ok(Arc::new(SignerMiddleware::new(
        Arc::try_unwrap(provider)
            .unwrap_or_else(|arc| (*arc).clone()),
        bundle_signer,
    )))
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::thread_rng;
    use tokio::test;

    #[test]
    async fn test_middleware_stack() -> Result<(), Box<dyn std::error::Error>> {
        let provider = Provider::<Http>::try_from("http://localhost:8545")?;
        let wallet = LocalWallet::new(&mut thread_rng());
        
        let middleware = create_middleware_stack(
            Arc::new(provider),
            wallet,
        ).await?;
        
        assert!(middleware.is_signer().await, "Middleware should be a signer");
        Ok(())
    }
}
