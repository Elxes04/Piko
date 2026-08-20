#!/bin/bash

# Piko Installation Script
# This script builds and installs Piko

set -e

echo "Installing Piko..."
echo "=================="

# Detect environment
if [ -n "$TERMUX_VERSION" ] || [[ "$PREFIX" == *"/data/data/com.termux"* ]]; then
    echo "Detected Termux environment"
    IS_TERMUX=true
    USE_SUDO=false
elif command -v sudo &> /dev/null; then
    echo "Sudo available - installing system-wide"
    IS_TERMUX=false
    USE_SUDO=true
else
    echo "Sudo not available - installing to user directory"
    IS_TERMUX=false
    USE_SUDO=false
fi

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo "Error: Rust/Cargo is not installed. Please install Rust first:"
    if [ "$IS_TERMUX" = true ]; then
        echo "   pkg install rust"
    else
        echo "   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    fi
    exit 1
fi

echo "Rust toolchain detected."

# Build the project
echo "Building Piko (release mode)..."
cargo build --release

# Set installation paths based on environment
if [ "$IS_TERMUX" = true ]; then
    BIN_DIR="$HOME/.local/bin"
    CONFIG_DIR="$HOME/.config/piko"
else
    if [ "$USE_SUDO" = true ]; then
        BIN_DIR="/usr/local/bin"
        CONFIG_DIR="/etc/piko"
    else
        BIN_DIR="$HOME/.local/bin"
        CONFIG_DIR="$HOME/.config/piko"
    fi
fi

# Create installation directories
echo "Creating directories..."
if [ "$USE_SUDO" = true ]; then
    sudo mkdir -p "$BIN_DIR"
    sudo mkdir -p "$CONFIG_DIR"
else
    mkdir -p "$BIN_DIR"
    mkdir -p "$CONFIG_DIR"
fi

# Copy binary
echo "Installing binary to $BIN_DIR/piko..."
if [ "$USE_SUDO" = true ]; then
    sudo cp target/release/piko "$BIN_DIR/"
    sudo chmod +x "$BIN_DIR/piko"
else
    cp target/release/piko "$BIN_DIR/"
    chmod +x "$BIN_DIR/piko"
fi

# Copy configuration files
echo "Installing sample configuration files..."
if [ "$USE_SUDO" = true ]; then
    sudo cp config/default_config.toml "$CONFIG_DIR/"
    sudo cp config/compact_config.toml "$CONFIG_DIR/"
    sudo cp config/border_config.toml "$CONFIG_DIR/"
    sudo cp config/termux_config.toml "$CONFIG_DIR/"
else
    cp config/default_config.toml "$CONFIG_DIR/"
    cp config/compact_config.toml "$CONFIG_DIR/"
    cp config/border_config.toml "$CONFIG_DIR/"
    cp config/termux_config.toml "$CONFIG_DIR/"
fi

# Add to PATH if not already there
if [ "$IS_TERMUX" = true ] || [ "$USE_SUDO" = false ]; then
    if ! grep -q "$BIN_DIR" "$HOME/.bashrc" 2>/dev/null; then
        echo "" >> "$HOME/.bashrc"
        echo "# Add Piko to PATH" >> "$HOME/.bashrc"
        echo "export PATH=\"\$PATH:$BIN_DIR\"" >> "$HOME/.bashrc"
        echo "Added $BIN_DIR to PATH in ~/.bashrc"
    fi
fi

echo ""
echo "Installation complete!"
echo ""
echo "Usage:"
echo "  piko                    # Run with defaults"
echo "  piko --theme nord       # Choose a theme"
echo "  piko --border           # Wrap in a border"
echo "  piko --format json      # Output JSON"
echo "  piko --help             # View all options"
echo ""
