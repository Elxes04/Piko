# Piko Documentation

Piko is a lightweight system information tool written in Rust, inspired by Neofetch.

```{toctree}
:maxdepth: 2
:caption: Table of Contents

installation
configuration
usage
```

## Overview

Piko queries system information directly from the kernel interfaces and displays formatted output alongside distribution ASCII art.

Features:

- Fast startup (<10ms) using direct procfs/sysfs parsers
- Zero-config execution with embedded defaults
- 10 built-in color themes
- 230+ distribution ASCII logos with authentic Neofetch color palettes and small variants
- Detailed system metrics: OS, Host/Model, Kernel, Uptime, Packages (11 package managers), Shell, DE, WM, Terminal, CPU, GPU, Memory, Disks, and Battery
- Terminal color blocks palette
- Border formatting (single, double, rounded, bold)
- Normal ANSI, JSON, and YAML output formats

## Sections

- [Installation](installation.rst) - Building from source, Cargo, and package managers
- [Configuration](configuration.rst) - Customizing info fields, themes, borders, and colors
- [Usage](usage.rst) - Command-line flags, options, and practical examples