#!/bin/bash

# Piko Installation Script
# This script installs Piko system information tool

set -e

echo "🐧 Installing Piko - System Information Tool"
echo "=============================================="

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo "❌ Rust is not installed. Please install Rust first:"
    echo "   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    exit 1
fi

echo "✅ Rust is installed"

# Build the project
echo "🔨 Building Piko..."
cargo build --release

# Create installation directories
echo "📁 Creating installation directories..."
sudo mkdir -p /usr/local/bin
sudo mkdir -p /etc/piko

# Copy binary
echo "📦 Installing Piko binary..."
sudo cp target/release/piko /usr/local/bin/

# Copy configuration files
echo "⚙️ Installing configuration files..."
sudo cp config/default_config.toml /etc/piko/
sudo cp config/compact_config.toml /etc/piko/
sudo cp config/border_config.toml /etc/piko/

# Set permissions
echo "🔐 Setting permissions..."
sudo chmod +x /usr/local/bin/piko

echo ""
echo "✅ Installation completed successfully!"
echo ""
echo "Usage examples:"
echo "  piko                    # Show system information with default settings"
echo "  piko --logo-only        # Show only the distribution logo"
echo "  piko --logo-size small  # Use smaller logo"
echo "  piko --border           # Add border around output"
echo "  piko --format json      # Output in JSON format"
echo "  piko --help             # Show all available options"
echo ""
echo "Configuration files are available at:"
echo "  /etc/piko/default_config.toml"
echo "  /etc/piko/compact_config.toml"
echo "  /etc/piko/border_config.toml"
echo ""
echo "🐧 Enjoy using Piko!"
