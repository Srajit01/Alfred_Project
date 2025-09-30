# Polygon Arbitrage Opportunity Detector Bot

A Rust-based bot that detects arbitrage opportunities between different DEXes on the Polygon network.

## Features

- **Multi-DEX Price Fetching**: Supports Uniswap V2, QuickSwap, and SushiSwap
- **Real-time Monitoring**: Continuously monitors price differences across DEXes
- **Profit Calculation**: Calculates potential profits including gas costs
- **Database Logging**: Stores opportunities in SQLite database
- **Configurable Parameters**: Easy configuration via TOML files
- **Comprehensive Logging**: Detailed logging for monitoring and debugging

## Architecture

### Core Components

1. **Price Fetchers** (`src/dex/`): Modular DEX interface implementations
2. **Arbitrage Detector** (`src/arbitrage.rs`): Core logic for opportunity detection
3. **Database Layer** (`src/database.rs`): SQLite integration for data persistence
4. **Configuration** (`src/config.rs`): TOML-based configuration management
5. **Error Handling** (`src/errors.rs`): Comprehensive error types

### Database Schema

```sql
CREATE TABLE arbitrage_opportunities (
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
);
```

## Installation & Setup

### Prerequisites

- Rust 1.70+
- Polygon RPC endpoint (Infura, Alchemy, or public RPC)

### Installation

1. Clone the repository:
```bash
git clone <repository-url>
cd polygon-arbitrage-bot
```

2. Install dependencies:
```bash
cargo build
```

3. Configure the application:
```bash
cp config.toml.example config.toml
# Edit config.toml with your settings
```

### Configuration

Edit `config.toml` to customize:

- **RPC Settings**: Update `polygon.rpc_url` with your endpoint
- **DEX Selection**: Enable/disable specific DEXes
- **Token Addresses**: Configure token contracts
- **Profit Thresholds**: Set minimum profit requirements
- **Trade Parameters**: Adjust trade sizes and gas estimates

### Running the Bot

```bash
# Run with default config
cargo run

# Run with custom config
cargo run -- --config custom_config.toml

# Dry run mode (no database writes)
cargo run -- --dry-run
```

## Supported DEXes

### Currently Implemented
- **Uniswap V2** on Polygon
- **QuickSwap** (Uniswap V2 fork)

### Easy to Add
- **SushiSwap** (included but disabled by default)
- **Any Uniswap V2 compatible DEX**

## Token Pairs

Currently supports:
- **WETH/USDC**
- **WBTC/USDC**

Additional pairs can be easily added by extending the configuration and detection logic.

## Key Metrics

The bot tracks several important metrics:

- **Price Difference**: Absolute price difference between DEXes
- **Profit USD**: Net profit after gas costs
- **Profit Percentage**: Return on investment percentage
- **Gas Cost**: Estimated transaction costs
- **Liquidity**: Available liquidity (when implemented)

## Risk Considerations

⚠️ **Important**: This is a simulation tool for educational purposes. Real arbitrage involves:

- **Slippage**: Price impact during execution
- **MEV Competition**: Other bots competing for opportunities
- **Gas Price Fluctuations**: Network congestion affects costs
- **Smart Contract Risk**: DEX contract vulnerabilities
- **Impermanent Loss**: Price changes during execution

## Development

### Adding New DEXes

1. Implement the `DexPriceFetcher` trait
2. Add configuration in `config.toml`
3. Register in `DexManager::new()`

Example:
```rust
pub struct NewDexFetcher {
    // Implementation
}

impl DexPriceFetcher for NewDexFetcher {
    async fn get_price(&self, token_pair: &TokenPair) -> Result<PriceQuote> {
        // Price fetching logic
    }
    
    fn get_name(&self) -> &str {
        "New DEX"
    }
}
```

### Adding New Token Pairs

1. Add token configurations to `config.toml`
2. Extend `check_arbitrage_opportunities()` in `ArbitrageDetector`

### Testing

```bash
# Run unit tests
cargo test

# Run with logging
RUST_LOG=debug cargo test -- --nocapture
```

## Monitoring

The bot provides comprehensive logging:

- **INFO**: Opportunity discoveries and system status
- **WARN**: Non-critical issues (failed price fetches)
- **ERROR**: Critical errors requiring attention

## Performance Considerations

- **RPC Rate Limits**: Configure appropriate check intervals
- **Database Growth**: Implement cleanup for old records
- **Memory Usage**: Monitor for long-running deployments
- **Network Latency**: Consider geographic proximity to RPCs

## Legal Notice

This software is for educational and research purposes only. Cryptocurrency trading involves substantial risk and may not be suitable for all investors. The authors are not responsible for any financial losses incurred through the use of this software.

## License

MIT License - see LICENSE file for details.