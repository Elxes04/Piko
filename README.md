# 🧩 Piko

![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Platform](https://img.shields.io/badge/platform-Linux%20%7C%20macOS-green)
![Status](https://img.shields.io/badge/status-WIP-yellow)
![Rust](https://img.shields.io/badge/Rust-1.75%2B-orange)
[![Docs](https://img.shields.io/badge/docs-online-success)](https://piko.readthedocs.io)
[![CI](https://github.com/Elxes04/piko/actions/workflows/rust.yml/badge.svg)](https://github.com/Elxes04/piko/actions/workflows/rust.yml)
[![Crates.io](https://img.shields.io/crates/v/piko.svg)](https://crates.io/crates/piko)
[![Issues](https://img.shields.io/github/issues/Elxes04/piko)](https://github.com/Elxes04/piko/issues)
[![Pull Requests](https://img.shields.io/github/issues-pr/Elxes04/piko)](https://github.com/Elxes04/piko/pulls)
[![Last Commit](https://img.shields.io/github/last-commit/Elxes04/piko)](https://github.com/Elxes04/piko/commits/main)


> A minimal, customizable system information tool written in Rust — inspired by Neofetch.

📚 **Documentation**: [Piko's Docs](https://piko.readthedocs.io)

---

## ✨ Overview

**Piko** is a lightweight and extensible command-line tool that gathers and displays system information in a customizable format. Inspired by tools like **Neofetch**, it offers a clean and flexible way to view details about your system — from OS to CPU, memory, and more.

## 🚀 Features

- 📦 Operating System details  
- 🧠 CPU specifications  
- 💾 Memory usage  
- 🗃️ Disk usage  
- 🌐 Network info *(WIP)*  
- 🎛️ Customizable output layout *(WIP)*  
- 🐧 Cross-platform: Linux & macOS  

## 🧱 Project Structure

```text
piko/
├── src/
│   ├── main.rs           # Entry point
│   ├── system_info.rs    # Collects system info
│   ├── output.rs         # Output formatting
│   └── config.rs         # Config file parsing
├── config/
│   ├── default_config.toml  # Default layout config
│   └── README.md            # Configuration guide
├── Cargo.toml            # Build & dependency config
└── README.md             # Project documentation

```

## 🛠 Installation

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

## ⚙️ Usage

After building, run the binary:

```bash
./target/release/piko
```

It will load your layout from:

```text
config/default_config.toml
```

and display system info accordingly.

## 🧩 Configuration

Customize your output by editing:

```text
config/default_config.toml
```

For a full list of configuration options and examples, check out:

📄 [Documentation on Read the Docs](https://piko.readthedocs.io)

Or see the in-repo config/README.md

## 🤝 Contributing

Got an idea or improvement? Contributions are welcome!  
Feel free to open an [issue](https://github.com/Elxes04/piko/issues) or submit a pull request 🚀

## 📄 License

Licensed under the MIT License.
