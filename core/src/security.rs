use secstr::SecUtf8;
use std::env;

#[derive(Debug, thiserror::Error)]
pub enum SecurityError {
    #[error("Invalid private key: {0}")]
    InvalidPrivateKey(String),
    #[error("Invalid signature: {0}")]
    InvalidSignature(String),
    #[error("Invalid address: {0}")]
    InvalidAddress(String),
    #[error("Unauthorized access: {0}")]
    UnauthorizedAccess(String),
    #[error("Environment variable error: {0}")]
    EnvVarError(#[from] env::VarError),
}

pub struct SecureVault {
    rpc_url: SecUtf8,
    private_key: SecUtf8,
    flashbots_secret: SecUtf8,
}

impl SecureVault {
    pub fn from_env() -> Result<Self, SecurityError> {
        let rpc_url = env::var("RPC_URL")?;
        let private_key = env::var("PRIVATE_KEY")?;
        let flashbots_secret = env::var("FLASHBOTS_SECRET")?;

        Ok(Self {
            rpc_url: SecUtf8::from(rpc_url),
            private_key: SecUtf8::from(private_key),
            flashbots_secret: SecUtf8::from(flashbots_secret),
        })
    }

    pub fn get_signer(&self) -> Result<ethers::signers::LocalWallet, SecurityError> {
        self.private_key
            .unsecure()
            .parse::<ethers::signers::LocalWallet>()
            .map_err(|_| SecurityError::InvalidPrivateKey("Invalid private key".to_string()))
    }

    pub fn rpc_url(&self) -> &SecUtf8 {
        &self.rpc_url
    }

    pub fn flashbots_secret(&self) -> &SecUtf8 {
        &self.flashbots_secret
    }
}
