use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub general: GeneralConfig,
    pub polygon: PolygonConfig,
    pub dexes: DexesConfig,
    pub tokens: TokensConfig,
    pub database: DatabaseConfig,
    pub arbitrage: ArbitrageConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneralConfig {
    pub check_interval: u64, // seconds
    pub log_level: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolygonConfig {
    pub rpc_url: String,
    pub chain_id: u64,
    pub gas_price_gwei: f64,
    pub gas_limit: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DexesConfig {
    pub uniswap_v2: DexConfig,
    pub quickswap: DexConfig,
    pub sushiswap: DexConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DexConfig {
    pub name: String,
    pub router_address: String,
    pub factory_address: String,
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokensConfig {
    pub weth: TokenConfig,
    pub wbtc: TokenConfig,
    pub usdc: TokenConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenConfig {
    pub address: String,
    pub decimals: u8,
    pub symbol: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArbitrageConfig {
    pub min_profit_usd: f64,
    pub min_profit_percentage: f64,
    pub trade_amount_usd: f64,
    pub max_slippage: f64,
}

impl Config {
    pub fn load(path: &str) -> Result<Self> {
        let content = fs::read_to_string(path)?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }
}