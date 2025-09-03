#!/bin/bash

# Piko Installation Script
# This script installs Piko system information tool

set -e

echo "🐧 Installing Piko - System Information Tool"
echo "=============================================="

# Detect environment
if [ -n "$TERMUX_VERSION" ] || [[ "$PREFIX" == *"/data/data/com.termux"* ]]; then
    echo "📱 Detected Termux environment"
    IS_TERMUX=true
    USE_SUDO=false
elif command -v sudo &> /dev/null; then
    echo "🔐 Sudo available - will use for system-wide installation"
    IS_TERMUX=false
    USE_SUDO=true
else
    echo "⚠️  Sudo not available - will install to user directory"
    IS_TERMUX=false
    USE_SUDO=false
fi

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo "❌ Rust is not installed. Please install Rust first:"
    if [ "$IS_TERMUX" = true ]; then
        echo "   pkg install rust"
    else
        echo "   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    fi
    exit 1
fi

echo "✅ Rust is installed"

# Build the project
echo "🔨 Building Piko..."
cargo build --release

# Set installation paths based on environment
if [ "$IS_TERMUX" = true ]; then
    # Termux: install to user directory
    BIN_DIR="$HOME/.local/bin"
    CONFIG_DIR="$HOME/.config/piko"
    echo "📁 Installing to Termux user directory"
else
    if [ "$USE_SUDO" = true ]; then
        # System-wide installation with sudo
        BIN_DIR="/usr/local/bin"
        CONFIG_DIR="/etc/piko"
        echo "📁 Installing system-wide"
    else
        # User directory installation without sudo
        BIN_DIR="$HOME/.local/bin"
        CONFIG_DIR="$HOME/.config/piko"
        echo "📁 Installing to user directory"
    fi
fi

# Create installation directories
echo "📁 Creating installation directories..."
if [ "$USE_SUDO" = true ]; then
    sudo mkdir -p "$BIN_DIR"
    sudo mkdir -p "$CONFIG_DIR"
else
    mkdir -p "$BIN_DIR"
    mkdir -p "$CONFIG_DIR"
fi

# Copy binary
echo "📦 Installing Piko binary..."
if [ "$USE_SUDO" = true ]; then
    sudo cp target/release/piko "$BIN_DIR/"
    sudo chmod +x "$BIN_DIR/piko"
else
    cp target/release/piko "$BIN_DIR/"
    chmod +x "$BIN_DIR/piko"
fi

# Copy configuration files
echo "⚙️ Installing configuration files..."
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
if [ "$IS_TERMUX" = true ]; then
    # For Termux, add to .bashrc if not present
    if ! grep -q "$BIN_DIR" "$HOME/.bashrc" 2>/dev/null; then
        echo "" >> "$HOME/.bashrc"
        echo "# Add Piko to PATH" >> "$HOME/.bashrc"
        echo "export PATH=\"\$PATH:$BIN_DIR\"" >> "$HOME/.bashrc"
        echo "📝 Added Piko to PATH in ~/.bashrc"
        echo "   Please restart your terminal or run: source ~/.bashrc"
    fi
elif [ "$USE_SUDO" = false ]; then
    # For user installation, add to .bashrc if not present
    if ! grep -q "$BIN_DIR" "$HOME/.bashrc" 2>/dev/null; then
        echo "" >> "$HOME/.bashrc"
        echo "# Add Piko to PATH" >> "$HOME/.bashrc"
        echo "export PATH=\"\$PATH:$BIN_DIR\"" >> "$HOME/.bashrc"
        echo "📝 Added Piko to PATH in ~/.bashrc"
        echo "   Please restart your terminal or run: source ~/.bashrc"
    fi
fi

echo ""
echo "✅ Installation completed successfully!"
echo ""

if [ "$IS_TERMUX" = true ]; then
    echo "📱 Termux Installation Details:"
    echo "   Binary: $BIN_DIR/piko"
    echo "   Config: $CONFIG_DIR/"
    echo "   PATH updated in ~/.bashrc"
    echo "   Termux config: $CONFIG_DIR/termux_config.toml"
elif [ "$USE_SUDO" = false ]; then
    echo "👤 User Installation Details:"
    echo "   Binary: $BIN_DIR/piko"
    echo "   Config: $CONFIG_DIR/"
    echo "   PATH updated in ~/.bashrc"
else
    echo "🔐 System Installation Details:"
    echo "   Binary: $BIN_DIR/piko"
    echo "   Config: $CONFIG_DIR/"
fi

echo ""
echo "Usage examples:"
echo "  piko                    # Show system information with default settings"
echo "  piko --logo-only        # Show only the distribution logo"
echo "  piko --logo-size small  # Use smaller logo"
echo "  piko --border           # Add border around output"
echo "  piko --format json      # Output in JSON format"
echo "  piko --help             # Show all available options"
echo ""

if [ "$IS_TERMUX" = true ]; then
    echo "🐧 Enjoy using Piko on Termux!"
    echo "   Note: Some system information may be limited due to Android restrictions"
else
    echo "🐧 Enjoy using Piko!"
fi
