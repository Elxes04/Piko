# Usage of Piko

Piko is a Neofetch-like tool that provides system information in a user-friendly format. This document outlines how to use the Piko program effectively.

## Command-Line Options

Piko can be run with various command-line options. The following options are available:

- `--config <path>`: Specify a custom path to a configuration file. If not provided, Piko will use the default configuration file located at `~/.config/piko/default_config.toml` on macOS or `/etc/piko/default_config.toml` on other systems.

## Running Piko

To run Piko, simply execute the following command in your terminal:

```bash
piko
```

If you want to use a custom configuration file, you can do so by specifying the `--config` option:

```bash
piko --config /path/to/your/config.toml
```

## Examples

### Basic Usage

To display system information using the default configuration, run:

```bash
piko
```

### Using a Custom Configuration

If you have a custom configuration file, you can run:

```bash
piko --config /path/to/custom_config.toml
```

## Features

Piko provides various features, including:

- Displaying system information such as OS, kernel version, and hardware specifications.
- Customizable output format through configuration files.
- Easy installation and setup process.

For more detailed information on configuration options, please refer to the [Configuration](configuration.md) document.