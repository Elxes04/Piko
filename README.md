# 🧩 Piko

![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Platform](https://img.shields.io/badge/platform-Linux%20%7C%20macOS-green)
![Status](https://img.shields.io/badge/status-Stable-green)
![Rust](https://img.shields.io/badge/Rust-1.75%2B-orange)
[![Docs](https://img.shields.io/badge/docs-online-success)](https://piko.readthedocs.io)

[![Version](https://img.shields.io/badge/version-1.0.0-blue.svg)](https://github.com/Elxes04/Piko/releases)
[![Issues](https://img.shields.io/github/issues/Elxes04/piko)](https://github.com/Elxes04/piko/issues)
[![Pull Requests](https://img.shields.io/github/issues-pr/Elxes04/piko)](https://github.com/Elxes04/piko/pulls)
[![Last Commit](https://img.shields.io/github/last-commit/Elxes04/piko)](https://github.com/Elxes04/piko/commits/main)

> A minimal, customizable system information tool written in Rust — inspired by Neofetch.

📚 **Documentation**: [Piko's Docs](https://piko.readthedocs.io)

---

## ✨ Overview

**Piko** is a lightweight and extensible command-line tool that gathers and displays system information in a customizable format. Inspired by tools like **Neofetch**, it offers a clean and flexible way to view details about your system — from OS to CPU, memory, and more.

## 🚀 Features

- 📦 **Operating System details** - OS name, version, and architecture
- 🧠 **CPU specifications** - Model, cores, and performance info
- 💾 **Memory usage** - RAM usage with percentage and total capacity
- 🗃️ **Disk usage** - Storage information and space utilization
- ⏰ **System uptime** - How long the system has been running
- 🖥️ **GPU information** - Graphics card details and drivers
- 🎛️ **Customizable output layout** - Flexible display options
- 🎨 **Multiple color schemes** - Pre-made themes and custom colors
- 🐧 **Cross-platform** - Linux & macOS support
- ⚡ **Fast and lightweight** - Written in Rust for optimal performance

## 🧱 Project Structure

```text
piko/
├── src/
│   ├── main.rs           # Entry point and CLI handling
│   ├── system_info.rs    # System information collection
│   ├── output.rs         # Output formatting and display
│   └── config.rs         # Configuration file parsing
├── config/
│   ├── default_config.toml    # Default Dracula-inspired theme
│   ├── pastel_config.toml     # Soft pastel color scheme
│   ├── dark_config.toml       # Dark professional theme
│   ├── COLOR_SCHEMES.md       # Color scheme documentation
│   └── README.md              # Configuration guide
├── docs/                 # Documentation files
├── Cargo.toml           # Build & dependency configuration
└── README.md            # Project documentation
```

## 🛠 Installation

### From Source

Ensure you have [Rust](https://rustup.rs/) installed:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Clone and build the project:

```bash
git clone https://github.com/Elxes04/piko.git
cd piko
cargo build --release
```

### Manual Installation

To install manually, copy the binary and configuration file:

```bash
sudo cp target/release/piko /usr/local/bin/
sudo mkdir -p /usr/share/piko
sudo cp config/default_config.toml /usr/share/piko/
```

## ⚙️ Usage

### Basic Usage

After building, run the binary:

```bash
./target/release/piko
```

### Configuration Options

Piko will automatically load the default configuration from:

- **Package installation**: `/usr/share/piko/default_config.toml`
- **macOS**: `~/.config/piko/default_config.toml`
- **Custom path**: Use `--config /path/to/config.toml`

The configuration file is included during installation and no longer needs to be downloaded from GitHub.

### Command Line Options

```bash
# Use default configuration
piko

# Use custom configuration file
piko --config /path/to/config.toml

# Use pre-made color schemes
piko --config config/pastel_config.toml
piko --config config/dark_config.toml
```

## 🧩 Configuration

Piko uses a TOML configuration file for customization. The default configuration is automatically installed with Piko.

### Configuration Locations

- **Package installation**: `/usr/share/piko/default_config.toml`
- **macOS**: `~/.config/piko/default_config.toml`
- **Custom path**: Use `--config /path/to/config.toml`

### Creating Custom Configurations

You can create your own configuration file by copying the default one:

```bash
cp /usr/share/piko/default_config.toml ~/.config/piko/my_config.toml
piko --config ~/.config/piko/my_config.toml
```

### Pre-made Color Schemes

Piko comes with several pre-made color schemes in the `config/` directory:

- **`default_config.toml`** - Modern Dracula-inspired palette (default)
- **`pastel_config.toml`** - Soft pastel colors for a gentle look
- **`dark_config.toml`** - Dark theme with elegant colors

To use a different color scheme:

```bash
piko --config config/pastel_config.toml
piko --config config/dark_config.toml
```

### Configuration Options

The configuration file allows you to customize:

- **Colors**: Hex color codes for each information type
- **Symbols**: Emoji or text symbols for each field
- **Layout**: Order and visibility of information fields
- **Display options**: Font size, alignment, and formatting

For detailed configuration documentation, see:

📄 [Documentation on Read the Docs](https://piko.readthedocs.io)

Or check the in-repo `config/COLOR_SCHEMES.md` for color scheme details.

## 🤝 Contributing

Got an idea or improvement? Contributions are welcome!  
Feel free to open an [issue](https://github.com/Elxes04/piko/issues) or submit a pull request 🚀

### Development Setup

1. Fork the repository
2. Create a feature branch: `git checkout -b feature-name`
3. Make your changes
4. Test your changes: `cargo test`
5. Commit your changes: `git commit -m 'Add feature'`
6. Push to the branch: `git push origin feature-name`
7. Submit a pull request



## 📄 License

Licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
