use ethers::{
    providers::{Provider, Http},
    types::{Address, U256},
    contract::Contract,
    abi::Abi,
};
use anyhow::Result;
use std::str::FromStr;
use std::sync::Arc;
use rust_decimal::Decimal;

pub mod uniswap;
pub mod quickswap;

use crate::config::{Config, DexConfig, TokenConfig};
use crate::errors::{ArbitrageError, Result as ArbitrageResult};

#[derive(Debug, Clone)]
pub struct TokenPair {
    pub token0: TokenConfig,
    pub token1: TokenConfig,
}

#[derive(Debug, Clone)]
pub struct PriceQuote {
    pub dex_name: String,
    pub token_pair: String,
    pub price: Decimal, // token1 per token0
    pub liquidity: Decimal,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

pub trait DexPriceFetcher: Send + Sync {
    async fn get_price(&self, token_pair: &TokenPair) -> ArbitrageResult<PriceQuote>;
    fn get_name(&self) -> &str;
}

pub struct DexManager {
    fetchers: Vec<Box<dyn DexPriceFetcher>>,
}

impl DexManager {
    pub async fn new(config: Config) -> Result<Self> {
        let provider = Arc::new(Provider::<Http>::try_from(&config.polygon.rpc_url)?);
        let mut fetchers: Vec<Box<dyn DexPriceFetcher>> = Vec::new();
        
        if config.dexes.uniswap_v2.enabled {
            let fetcher = uniswap::UniswapV2Fetcher::new(
                provider.clone(),
                config.dexes.uniswap_v2,
            ).await?;
            fetchers.push(Box::new(fetcher));
        }
        
        if config.dexes.quickswap.enabled {
            let fetcher = quickswap::QuickSwapFetcher::new(
                provider.clone(),
                config.dexes.quickswap,
            ).await?;
            fetchers.push(Box::new(fetcher));
        }
        
        Ok(Self { fetchers })
    }
    
    pub async fn get_all_prices(&self, token_pair: &TokenPair) -> Vec<PriceQuote> {
        let mut prices = Vec::new();
        
        for fetcher in &self.fetchers {
            match fetcher.get_price(token_pair).await {
                Ok(quote) => prices.push(quote),
                Err(e) => {
                    tracing::warn!("Failed to get price from {}: {}", fetcher.get_name(), e);
                }
            }
        }
        
        prices
    }
}

// Uniswap V2 Router ABI (simplified)
pub fn get_uniswap_v2_abi() -> Abi {
    serde_json::from_str(r#"[
        {
            "inputs": [
                {"internalType": "uint256", "name": "amountIn", "type": "uint256"},
                {"internalType": "address[]", "name": "path", "type": "address[]"}
            ],
            "name": "getAmountsOut",
            "outputs": [
                {"internalType": "uint256[]", "name": "amounts", "type": "uint256[]"}
            ],
            "stateMutability": "view",
            "type": "function"
        }
    ]"#).unwrap()
}