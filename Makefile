.PHONY: build run test clean setup lint

# Build the project
build:
	cargo build --release

# Run the bot with default config
run:
	cargo run --release

# Run tests
test:
	cargo test

# Clean build artifacts
clean:
	cargo clean
	rm -f *.db *.db-shm *.db-wal

# Setup development environment
setup:
	rustup update
	cargo install cargo-watch
	cp config.toml.example config.toml

# Lint the code
lint:
	cargo clippy -- -D warnings
	cargo fmt --check

# Format the code
fmt:
	cargo fmt

# Run with file watching for development
dev:
	cargo watch -x run

# Run tests with file watching
test-watch:
	cargo watch -x test

# Generate documentation
docs:
	cargo doc --open

# Check for security vulnerabilities
audit:
	cargo install cargo-audit
	cargo audit

# Run the bot in dry-run mode
dry-run:
	cargo run -- --dry-run

# View recent opportunities from database
view-opportunities:
	sqlite3 arbitrage.db "SELECT * FROM arbitrage_opportunities ORDER BY timestamp DESC LIMIT 10;"

# Database statistics
db-stats:
	sqlite3 arbitrage.db "SELECT COUNT(*) as total_opportunities, AVG(profit_usd) as avg_profit, MAX(profit_usd) as max_profit FROM arbitrage_opportunities;"