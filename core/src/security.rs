use secstr::SecUtf8;
use std::env;

#[derive(Debug, thiserror::Error)]
pub enum SecurityError {{
    #[error("Environment variable error: {{0}}")]
    EnvVar(#[from] env::VarError),
    #[error("Invalid private key format")]
    InvalidKey,
}}

pub struct SecureVault {{
    rpc_url: SecUtf8,
    private_key: SecUtf8,
    flashbots_secret: SecUtf8,
}}

impl SecureVault {{
    pub fn from_env() -> Result<Self, SecurityError> {{
        Ok(Self {{
            rpc_url: SecUtf8::from(env::var("RPC_URL")?),
            private_key: SecUtf8::from(env::var("PRIVATE_KEY")?),
            flashbots_secret: SecUtf8::from(env::var("FLASHBOTS_SECRET")?),
        }})
    }}

    pub fn get_signer(&self) -> Result<ethers::signers::LocalWallet, SecurityError> {{
        self.private_key
            .unsecure()
            .parse::<ethers::signers::LocalWallet>()
            .map_err(|_| SecurityError::InvalidKey)
    }}
}}
