use crate::config::Config;
use crate::database::{Database, ArbitrageOpportunity};
use crate::dex::{DexManager, TokenPair, PriceQuote};
use crate::errors::{ArbitrageError, Result};
use chrono::Utc;
use rust_decimal::Decimal;
use tracing::{info, warn, error};

pub struct ArbitrageDetector {
    config: Config,
    database: Database,
    dex_manager: DexManager,
}

impl ArbitrageDetector {
    pub async fn new(config: Config, database: Database) -> Result<Self> {
        let dex_manager = DexManager::new(config.clone()).await
            .map_err(|e| ArbitrageError::Config(format!("Failed to initialize DEX manager: {}", e)))?;
        
        Ok(Self {
            config,
            database,
            dex_manager,
        })
    }
    
    pub async fn check_arbitrage_opportunities(&mut self) -> Result<Vec<ArbitrageOpportunity>> {
        let mut opportunities = Vec::new();
        
        // Check WETH/USDC pair
        let weth_usdc_pair = TokenPair {
            token0: self.config.tokens.weth.clone(),
            token1: self.config.tokens.usdc.clone(),
        };
        
        if let Some(opportunity) = self.check_token_pair(&weth_usdc_pair).await? {
            opportunities.push(opportunity);
        }
        
        // Check WBTC/USDC pair
        let wbtc_usdc_pair = TokenPair {
            token0: self.config.tokens.wbtc.clone(),
            token1: self.config.tokens.usdc.clone(),
        };
        
        if let Some(opportunity) = self.check_token_pair(&wbtc_usdc_pair).await? {
            opportunities.push(opportunity);
        }
        
        Ok(opportunities)
    }
    
    async fn check_token_pair(&self, token_pair: &TokenPair) -> Result<Option<ArbitrageOpportunity>> {
        let prices = self.dex_manager.get_all_prices(token_pair).await;
        
        if prices.len() < 2 {
            warn!("Not enough price quotes for {}/{}", token_pair.token0.symbol, token_pair.token1.symbol);
            return Ok(None);
        }
        
        // Find best buy and sell prices
        let mut best_buy = &prices[0];
        let mut best_sell = &prices[0];
        
        for price in &prices {
            if price.price < best_buy.price {
                best_buy = price;
            }
            if price.price > best_sell.price {
                best_sell = price;
            }
        }
        
        // Skip if buying and selling on the same DEX
        if best_buy.dex_name == best_sell.dex_name {
            return Ok(None);
        }
        
        let opportunity = self.calculate_arbitrage_profit(best_buy, best_sell, token_pair)?;
        
        // Check if opportunity meets minimum requirements
        if opportunity.profit_usd >= Decimal::try_from(self.config.arbitrage.min_profit_usd).unwrap() &&
           opportunity.profit_percentage >= Decimal::try_from(self.config.arbitrage.min_profit_percentage).unwrap() {
            
            info!("Arbitrage opportunity found: {}", serde_json::to_string(&opportunity).unwrap_or_default());
            
            // Save to database
            self.database.save_opportunity(&opportunity).await
                .map_err(|e| ArbitrageError::Database(e))?;
            
            return Ok(Some(opportunity));
        }
        
        Ok(None)
    }
    
    fn calculate_arbitrage_profit(
        &self,
        buy_quote: &PriceQuote,
        sell_quote: &PriceQuote,
        token_pair: &TokenPair,
    ) -> Result<ArbitrageOpportunity> {
        let trade_amount_usd = Decimal::try_from(self.config.arbitrage.trade_amount_usd)
            .map_err(|e| ArbitrageError::Calculation(format!("Invalid trade amount: {}", e)))?;
        
        // Calculate trade amount in base token
        let trade_amount_tokens = trade_amount_usd / buy_quote.price;
        
        // Calculate costs and revenues
        let buy_cost = trade_amount_tokens * buy_quote.price;
        let sell_revenue = trade_amount_tokens * sell_quote.price;
        
        // Calculate gas cost (simplified)
        let gas_cost_usd = Decimal::try_from(
            self.config.polygon.gas_price_gwei * (self.config.polygon.gas_limit as f64) * 1e-9 * 2.0 // Assume 2 transactions
        ).map_err(|e| ArbitrageError::Calculation(format!("Gas cost calculation error: {}", e)))?;
        
        // Calculate profit
        let gross_profit = sell_revenue - buy_cost;
        let net_profit = gross_profit - gas_cost_usd;
        let profit_percentage = if buy_cost > Decimal::ZERO {
            (net_profit / buy_cost) * Decimal::from(100)
        } else {
            Decimal::ZERO
        };
        
        let price_difference = sell_quote.price - buy_quote.price;
        
        Ok(ArbitrageOpportunity {
            id: None,
            timestamp: Utc::now(),
            token_pair: format!("{}/{}", token_pair.token0.symbol, token_pair.token1.symbol),
            buy_dex: buy_quote.dex_name.clone(),
            sell_dex: sell_quote.dex_name.clone(),
            buy_price: buy_quote.price,
            sell_price: sell_quote.price,
            price_difference,
            profit_usd: net_profit,
            profit_percentage,
            trade_amount: trade_amount_usd,
            gas_cost_usd,
        })
    }
}