# piko

A fast, lightweight system information tool written in Rust, inspired by Neofetch.

[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![Version](https://img.shields.io/badge/version-1.2.0-blue.svg)](https://github.com/Elxes04/piko/releases)

Piko gathers and displays system metrics alongside distribution ASCII art. It is built to start instantly (sub-10ms), requires zero configuration to run, and supports custom layouts and color palettes.

## Features

- Fast execution (<10ms) using direct procfs/sysfs reads
- Zero configuration required; embedded defaults work out of the box
- 10 built-in color themes (Catppuccin, Nord, Dracula, Gruvbox, Tokyo Night, One Dark, Solarized, Rose Pine, Synthwave)
- 230+ distribution ASCII logos imported from Neofetch with authentic multi-color palettes and small variants
- Comprehensive system info: OS, Host/Model, Kernel, Uptime, Packages (11 package managers), Shell, DE, WM, Terminal, CPU, GPU (with udev / vulkan hardware detection), Memory, Disks, and Battery
- Terminal color blocks palette
- Border formatting (single, double, rounded, bold)
- Formats: Terminal ANSI, JSON, and YAML

## Installation

### From Source

```bash
git clone https://github.com/Elxes04/piko.git
cd piko
cargo build --release
sudo cp target/release/piko /usr/local/bin/
```

Or run the included installer:

```bash
./install.sh
```

### From Cargo

```bash
cargo install --path .
```

## Usage

Run `piko` without arguments to display system info using your detected distribution logo:

```bash
piko
```

### CLI Flags

```text
Options:
  -c, --config <CONFIG>                Path to custom TOML config file
  -t, --theme <THEME>                  Color theme name (e.g. catppuccin, nord, dracula, gruvbox, tokyo-night)
      --ascii-distro <DISTRO>          Override distribution logo (e.g. arch, nixos, debian, fedora)
      --logo-only                      Display only the distro logo
      --list-logos                     List all available distribution logos
      --list-themes                    List all available themes
  -f, --format <FORMAT>                Output format: normal, json, yaml [default: normal]
      --logo-position <POSITION>       Logo position: left, right, top, bottom
      --logo-size <SIZE>               Logo size: small, medium, large
      --logo-style <STYLE>             Logo style: ascii, unicode, minimal
  -b, --border                         Draw a border around the output
      --border-style <STYLE>           Border style: single, double, rounded, bold
      --no-separators                  Hide field separators
      --color-blocks <BOOL>            Toggle color blocks palette (true/false)
      --gen-config                     Generate default config in ~/.config/piko/config.toml
      --print-config                   Print default configuration to stdout
      --export-config <PATH>           Export current configuration to file
      --import-config <PATH>           Load and apply configuration from file
  -h, --help                           Print help
  -V, --version                        Print version
```

### Examples

```bash
# Use a specific theme
piko --theme nord

# Use compact logo size with a border
piko --logo-size small --border

# Override the distro logo
piko --distro arch

# Generate a config file to customize
piko --gen-config

# Export machine-readable system info
piko --format json
```

## Configuration

Piko checks for configuration files in the following order:

1. Path supplied to `--config <path>`
2. Path defined in `$PIKO_CONFIG`
3. `$XDG_CONFIG_HOME/piko/config.toml` (or `~/.config/piko/config.toml`)
4. `/etc/piko/config.toml`
5. Embedded default configuration

Generate a starter configuration file at `~/.config/piko/config.toml`:

```bash
piko --gen-config
```

Example configuration snippet:

```toml
layout = "default"

[output]
info_keys = [
    "OS",
    "Host",
    "Kernel",
    "Uptime",
    "Packages",
    "Shell",
    "DE",
    "WM",
    "Terminal",
    "CPU",
    "GPU",
    "Memory",
    "Disk",
    "Battery",
]
show_logo = true
logo_position = "left"
logo_size = "medium"
show_title = true
show_color_blocks = true
color_blocks_style = "circle"

[theme]
name = "catppuccin"

[display]
border = false
border_style = "rounded"
show_separators = true
separator_symbol = ":"
```

## Supported Distributions

Piko includes ASCII art for:

Alpine, Android / Termux, Arch Linux, Artix, CentOS, Debian, EndeavourOS, Fedora, FreeBSD, Gentoo, Kali, Linux Mint, macOS, Manjaro, NixOS, OpenBSD, openSUSE, Pop!_OS, Raspberry Pi, Red Hat, Ubuntu, Void Linux, Windows, and generic Linux (Tux).

To see all available logos at any time:

```bash
piko --list-logos
```

## Documentation

Full documentation is available in the `docs/` directory:

- [Installation Guide](docs/installation.rst)
- [Configuration Guide](docs/configuration.rst)
- [Usage Guide](docs/usage.rst)

## License

MIT License. See [LICENSE](LICENSE) for details.


