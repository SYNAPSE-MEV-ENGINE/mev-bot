use thiserror::Error;

#[derive(Debug, Error)]
pub enum UtilError {
    #[error("Invalid configuration: {0}")]
    Config(String),
}
