use super::{DexPriceFetcher, PriceQuote, TokenPair, get_uniswap_v2_abi};
use crate::config::DexConfig;
use crate::errors::{ArbitrageError, Result};
use ethers::{
    providers::{Provider, Http},
    types::{Address, U256},
    contract::Contract,
};
use std::str::FromStr;
use std::sync::Arc;
use rust_decimal::Decimal;
use chrono::Utc;

pub struct QuickSwapFetcher {
    router_contract: Contract<Arc<Provider<Http>>>,
    config: DexConfig,
}

impl QuickSwapFetcher {
    pub async fn new(
        provider: Arc<Provider<Http>>,
        config: DexConfig,
    ) -> Result<Self> {
        let router_address = Address::from_str(&config.router_address)
            .map_err(|e| ArbitrageError::Config(format!("Invalid router address: {}", e)))?;
        
        let abi = get_uniswap_v2_abi();
        let router_contract = Contract::new(router_address, abi, provider);
        
        Ok(Self {
            router_contract,
            config,
        })
    }
    
    async fn get_amounts_out(&self, amount_in: U256, path: Vec<Address>) -> Result<Vec<U256>> {
        let amounts: Vec<U256> = self
            .router_contract
            .method::<_, Vec<U256>>("getAmountsOut", (amount_in, path))?
            .call()
            .await?;
        
        Ok(amounts)
    }
}

impl DexPriceFetcher for QuickSwapFetcher {
    async fn get_price(&self, token_pair: &TokenPair) -> Result<PriceQuote> {
        let token0_address = Address::from_str(&token_pair.token0.address)
            .map_err(|e| ArbitrageError::Config(format!("Invalid token0 address: {}", e)))?;
        let token1_address = Address::from_str(&token_pair.token1.address)
            .map_err(|e| ArbitrageError::Config(format!("Invalid token1 address: {}", e)))?;
        
        // Use 1 unit of token0 for price calculation
        let amount_in = U256::from(10u128.pow(token_pair.token0.decimals as u32));
        let path = vec![token0_address, token1_address];
        
        let amounts = self.get_amounts_out(amount_in, path).await?;
        
        if amounts.len() < 2 {
            return Err(ArbitrageError::PriceFetch(
                "Invalid amounts returned from router".to_string(),
            ));
        }
        
        let amount_out = amounts[1];
        
        // Calculate price: token1 per token0
        let price_raw = amount_out.as_u128() as f64 / (10u128.pow(token_pair.token1.decimals as u32) as f64);
        let price = Decimal::try_from(price_raw)
            .map_err(|e| ArbitrageError::Calculation(format!("Price conversion error: {}", e)))?;
        
        Ok(PriceQuote {
            dex_name: self.config.name.clone(),
            token_pair: format!("{}/{}", token_pair.token0.symbol, token_pair.token1.symbol),
            price,
            liquidity: Decimal::ZERO, // TODO: Implement liquidity calculation
            timestamp: Utc::now(),
        })
    }
    
    fn get_name(&self) -> &str {
        &self.config.name
    }
}