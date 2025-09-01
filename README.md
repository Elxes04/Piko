# Piko 🐧

A highly customizable system information tool written in Rust — inspired by Neofetch

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Crates.io](https://img.shields.io/crates/v/piko)](https://crates.io/crates/piko)
[![Build Status](https://github.com/Elxes04/piko/workflows/CI/badge.svg)](https://github.com/Elxes04/piko/actions)
[![Version](https://img.shields.io/badge/version-1.1.0-blue.svg)](https://github.com/Elxes04/piko/releases)

## Features ✨

- **Automatic Distribution Detection**: Automatically detects and displays your Linux distribution logo
- **Highly Customizable**: Extensive configuration options for colors, symbols, layout, and more
- **Multiple Output Formats**: Support for normal, JSON, and YAML output
- **Logo Support**: Built-in ASCII art logos for popular Linux distributions
- **Flexible Layout**: Configurable logo position, size, and style
- **Theme Support**: Multiple color themes and custom color schemes
- **Export/Import**: Save and load your custom configurations
- **Border Options**: Add decorative borders around your output

## Quick Start 🚀

```bash
# Show system information with default settings
piko

# Show only the distribution logo
piko --logo-only

# List all available distribution logos
piko --list-logos

# Get help
piko --help
```

## Documentation 📚

For detailed information about installation, configuration, and advanced features, please see our documentation:

- **[Installation Guide](docs/installation.rst)** - How to install Piko
- **[Configuration Guide](docs/configuration.rst)** - Customizing Piko
- **[Usage Guide](docs/usage.rst)** - Command line options and examples
- **[Advanced Features](docs/ADVANCED_FEATURES.md)** - Advanced customization and features

## Supported Distributions 🐧

Piko includes built-in ASCII art logos for:

- **Arch Linux** - The iconic Arch logo
- **Ubuntu** - Ubuntu's distinctive logo
- **Debian** - Debian's classic logo
- **Fedora** - Fedora's modern logo
- **Manjaro** - Manjaro's sleek logo
- **openSUSE** - openSUSE's professional logo
- **CentOS** - CentOS's enterprise logo
- **Alpine** - Alpine's minimal logo
- **Gentoo** - Gentoo's penguin logo
- **Default** - Generic Linux logo

## Contributing 🤝

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

## Changelog 📝

See [CHANGELOG.md](CHANGELOG.md) for a list of changes and version history.

### Adding New Distribution Logos

To add a new distribution logo:

1. Add the ASCII art to `src/distro_logo.rs`
2. Update the detection logic if needed
3. Test with your distribution

## License 📄

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments 🙏

- Inspired by [Neofetch](https://github.com/dylanaraps/neofetch)
- Built with [Rust](https://www.rust-lang.org/)
- Uses [sysinfo](https://github.com/GuillaumeGomez/sysinfo) for system information
- Styled with [colored](https://github.com/mackwic/colored)

---

**Piko** - Because every penguin deserves a beautiful system information display! 🐧✨
