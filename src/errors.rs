use thiserror::Error;

#[derive(Error, Debug)]
pub enum ArbitrageError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    
    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),
    
    #[error("Ethereum error: {0}")]
    Ethereum(#[from] ethers::providers::ProviderError),
    
    #[error("Contract error: {0}")]
    Contract(#[from] ethers::contract::ContractError<ethers::providers::Provider<ethers::providers::Http>>),
    
    #[error("Configuration error: {0}")]
    Config(String),
    
    #[error("Price fetch error: {0}")]
    PriceFetch(String),
    
    #[error("Calculation error: {0}")]
    Calculation(String),
    
    #[error("Invalid token pair: {from} -> {to}")]
    InvalidTokenPair { from: String, to: String },
}

pub type Result<T> = std::result::Result<T, ArbitrageError>;