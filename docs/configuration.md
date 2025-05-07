# Configuration Options for Piko

Piko allows users to customize its behavior through a configuration file. This document outlines the available configuration options and how to set them.

## Configuration File Location

The default configuration file is located at:

- **macOS**: `~/.config/piko/default_config.toml`
- **Linux**: `/etc/piko/default_config.toml`

If the configuration file does not exist, Piko will attempt to download a default configuration file automatically.

## Configuration Format

The configuration file is written in TOML format. Below is an example of the structure of the configuration file:

```toml
# Example configuration file for Piko

[general]
# General settings for Piko
option1 = "value1"
option2 = "value2"

[system]
# System-related settings
show_os_info = true
show_hardware_info = false

[output]
# Output settings
output_format = "pretty"  # Options: "pretty", "json", "yaml"
```

## Configuration Options

### General Settings

- `option1`: Description of option1.
- `option2`: Description of option2.

### System Settings

- `show_os_info`: A boolean value that determines whether to display OS information.
- `show_hardware_info`: A boolean value that determines whether to display hardware information.

### Output Settings

- `output_format`: Specifies the format of the output. Options include:
  - `pretty`: Human-readable format.
  - `json`: JSON format.
  - `yaml`: YAML format.

## Customizing Configuration

To customize Piko's behavior, edit the configuration file according to your preferences. After making changes, save the file and restart Piko to apply the new settings.

## Troubleshooting

If you encounter issues with the configuration file:

- Ensure the file is in the correct TOML format.
- Check for typos in option names.
- Refer to the default configuration file for guidance on available options.