#!/bin/bash

# Polygon Node Setup Script
# This script helps configure the bot for different Polygon RPC providers

echo "🔗 Polygon Arbitrage Bot - RPC Setup"
echo "===================================="

# Function to test RPC endpoint
test_rpc() {
    local rpc_url=$1
    echo "Testing RPC endpoint: $rpc_url"
    
    response=$(curl -s -X POST \
        -H "Content-Type: application/json" \
        --data '{"jsonrpc":"2.0","method":"eth_chainId","params":[],"id":1}' \
        "$rpc_url")
    
    if echo "$response" | grep -q "0x89"; then
        echo "✅ RPC endpoint is working (Polygon Mainnet)"
        return 0
    else
        echo "❌ RPC endpoint test failed"
        return 1
    fi
}

echo "Available RPC options:"
echo "1. Public Polygon RPC (Free, rate limited)"
echo "2. Infura (Requires API key)"
echo "3. Alchemy (Requires API key)"
echo "4. Custom RPC endpoint"

read -p "Select an option (1-4): " choice

case $choice in
    1)
        rpc_url="https://polygon-rpc.com"
        echo "Using public Polygon RPC..."
        ;;
    2)
        read -p "Enter your Infura project ID: " infura_id
        rpc_url="https://polygon-mainnet.infura.io/v3/$infura_id"
        ;;
    3)
        read -p "Enter your Alchemy API key: " alchemy_key
        rpc_url="https://polygon-mainnet.g.alchemy.com/v2/$alchemy_key"
        ;;
    4)
        read -p "Enter your custom RPC URL: " rpc_url
        ;;
    *)
        echo "Invalid option. Using public RPC."
        rpc_url="https://polygon-rpc.com"
        ;;
esac

# Test the RPC endpoint
if test_rpc "$rpc_url"; then
    # Update config.toml
    if [ -f "config.toml" ]; then
        # Create backup
        cp config.toml config.toml.backup
        
        # Update RPC URL in config
        sed -i.bak "s|rpc_url = \".*\"|rpc_url = \"$rpc_url\"|g" config.toml
        
        echo "✅ Configuration updated successfully!"
        echo "📋 RPC URL: $rpc_url"
        echo ""
        echo "Next steps:"
        echo "1. Review and adjust other settings in config.toml"
        echo "2. Run the bot: cargo run"
    else
        echo "❌ config.toml not found. Please copy from config.toml.example"
    fi
else
    echo "❌ RPC endpoint test failed. Please check your configuration."
fi