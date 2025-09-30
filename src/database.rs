use sqlx::{SqlitePool, Row};
use anyhow::Result;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArbitrageOpportunity {
    pub id: Option<i64>,
    pub timestamp: DateTime<Utc>,
    pub token_pair: String,
    pub buy_dex: String,
    pub sell_dex: String,
    pub buy_price: Decimal,
    pub sell_price: Decimal,
    pub price_difference: Decimal,
    pub profit_usd: Decimal,
    pub profit_percentage: Decimal,
    pub trade_amount: Decimal,
    pub gas_cost_usd: Decimal,
}

pub struct Database {
    pool: SqlitePool,
}

impl Database {
    pub async fn new(database_url: &str) -> Result<Self> {
        let pool = SqlitePool::connect(database_url).await?;
        Ok(Self { pool })
    }
    
    pub async fn migrate(&self) -> Result<()> {
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS arbitrage_opportunities (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                timestamp DATETIME NOT NULL,
                token_pair TEXT NOT NULL,
                buy_dex TEXT NOT NULL,
                sell_dex TEXT NOT NULL,
                buy_price DECIMAL(20, 8) NOT NULL,
                sell_price DECIMAL(20, 8) NOT NULL,
                price_difference DECIMAL(20, 8) NOT NULL,
                profit_usd DECIMAL(20, 8) NOT NULL,
                profit_percentage DECIMAL(10, 4) NOT NULL,
                trade_amount DECIMAL(20, 8) NOT NULL,
                gas_cost_usd DECIMAL(20, 8) NOT NULL
            )
            "#,
        )
        .execute(&self.pool)
        .await?;
        
        sqlx::query(
            r#"
            CREATE INDEX IF NOT EXISTS idx_timestamp ON arbitrage_opportunities(timestamp);
            CREATE INDEX IF NOT EXISTS idx_token_pair ON arbitrage_opportunities(token_pair);
            CREATE INDEX IF NOT EXISTS idx_profit_usd ON arbitrage_opportunities(profit_usd);
            "#,
        )
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
    
    pub async fn save_opportunity(&self, opportunity: &ArbitrageOpportunity) -> Result<i64> {
        let result = sqlx::query(
            r#"
            INSERT INTO arbitrage_opportunities (
                timestamp, token_pair, buy_dex, sell_dex, buy_price, sell_price,
                price_difference, profit_usd, profit_percentage, trade_amount, gas_cost_usd
            )
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&opportunity.timestamp)
        .bind(&opportunity.token_pair)
        .bind(&opportunity.buy_dex)
        .bind(&opportunity.sell_dex)
        .bind(&opportunity.buy_price)
        .bind(&opportunity.sell_price)
        .bind(&opportunity.price_difference)
        .bind(&opportunity.profit_usd)
        .bind(&opportunity.profit_percentage)
        .bind(&opportunity.trade_amount)
        .bind(&opportunity.gas_cost_usd)
        .execute(&self.pool)
        .await?;
        
        Ok(result.last_insert_rowid())
    }
    
    pub async fn get_recent_opportunities(&self, limit: i64) -> Result<Vec<ArbitrageOpportunity>> {
        let rows = sqlx::query(
            r#"
            SELECT * FROM arbitrage_opportunities
            ORDER BY timestamp DESC
            LIMIT ?
            "#,
        )
        .bind(limit)
        .fetch_all(&self.pool)
        .await?;
        
        let mut opportunities = Vec::new();
        for row in rows {
            opportunities.push(ArbitrageOpportunity {
                id: Some(row.get("id")),
                timestamp: row.get("timestamp"),
                token_pair: row.get("token_pair"),
                buy_dex: row.get("buy_dex"),
                sell_dex: row.get("sell_dex"),
                buy_price: row.get("buy_price"),
                sell_price: row.get("sell_price"),
                price_difference: row.get("price_difference"),
                profit_usd: row.get("profit_usd"),
                profit_percentage: row.get("profit_percentage"),
                trade_amount: row.get("trade_amount"),
                gas_cost_usd: row.get("gas_cost_usd"),
            });
        }
        
        Ok(opportunities)
    }
}