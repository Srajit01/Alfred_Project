use anyhow::Result;
use clap::Parser;
use std::time::Duration;
use tokio::time;
use tracing::{info, error, warn};

mod config;
mod database;
mod dex;
mod arbitrage;
mod errors;

use config::Config;
use database::Database;
use arbitrage::ArbitrageDetector;

#[derive(Parser)]
#[command(name = "polygon-arbitrage-bot")]
#[command(about = "A Polygon arbitrage opportunity detector bot")]
struct Args {
    #[arg(short, long, default_value = "config.toml")]
    config: String,
    
    #[arg(short, long)]
    dry_run: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();
    
    let args = Args::parse();
    
    // Load configuration
    let config = Config::load(&args.config)?;
    info!("Configuration loaded from {}", args.config);
    
    // Initialize database
    let database = Database::new(&config.database.url).await?;
    database.migrate().await?;
    info!("Database initialized and migrated");
    
    // Initialize arbitrage detector
    let mut detector = ArbitrageDetector::new(config.clone(), database).await?;
    info!("Arbitrage detector initialized");
    
    // Main detection loop
    let mut interval = time::interval(Duration::from_secs(config.general.check_interval));
    
    loop {
        interval.tick().await;
        
        match detector.check_arbitrage_opportunities().await {
            Ok(opportunities) => {
                if opportunities.is_empty() {
                    info!("No arbitrage opportunities found");
                } else {
                    info!("Found {} arbitrage opportunities", opportunities.len());
                    for opportunity in opportunities {
                        info!("Opportunity: {} -> {} | Profit: ${:.2}", 
                            opportunity.buy_dex, 
                            opportunity.sell_dex, 
                            opportunity.profit_usd
                        );
                    }
                }
            }
            Err(e) => {
                error!("Error checking arbitrage opportunities: {}", e);
            }
        }
    }
}