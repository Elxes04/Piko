Configuration Options for Piko
==============================

Piko allows users to customize its behavior through a configuration file. This document outlines the available configuration options and how to set them.

Configuration File Location
---------------------------

The default configuration file is located at:

- **Package installation**: `/usr/share/piko/default_config.toml`
- **macOS**: `~/.config/piko/default_config.toml`
- **Custom path**: Use `--config /path/to/config.toml`

The configuration file is included during installation and no longer needs to be downloaded from GitHub.

Configuration Format
--------------------

The configuration file is written in TOML format. Here's an example of the structure:

.. code-block:: toml

   layout = "default"

   [output]
   show_os = true
   show_cpu = true
   show_memory = true
   show_disk = true
   show_uptime = true
   show_cpu_model = true
   show_gpu_model = true
   show_kernel_version = true
   show_display_server = true
   show_desktop_environment = true
   show_username = true
   show_hostname = true
   show_disks = true

   info_keys = [
       "OS",
       "Kernel Version",
       "Desktop Environment",
       "Display Server",
       "Uptime",
       "CPU Model",
       "Memory",
       "GPU Model",
       "Username",
       "Hostname",
       "Disks"
   ]

   [customization]
   color_scheme = "default"
   font_size = 12
   alignment = "left"

   [colors]
   OS = "#FF79C6"
   Username = "#50FA7B"
   Hostname = "#F1FA8C"
   "Desktop Environment" = "#BD93F9"
   Memory = "#8BE9FD"
   Disks = "#FF5555"
   "CPU Model" = "#6272A4"
   "GPU Model" = "#FFB86C"
   "Kernel Version" = "#44475A"
   "Display Server" = "#A4FFFF"
   Uptime = "#FF92DF"

   [symbols]
   OS = "🖥️"
   Username = "👤"
   Hostname = "📡"
   "Desktop Environment" = "🌠"
   Memory = "💾"
   Disks = "🗃️"
   "CPU Model" = "🧪"
   "GPU Model" = "🖨️"
   "Kernel Version" = "💻"
   "Display Server" = "🖱️"
   Uptime = "⏰"

Configuration Options
---------------------

**Output Settings**

- ``show_os``: Display OS information (boolean)
- ``show_cpu``: Display CPU information (boolean)
- ``show_memory``: Display memory usage (boolean)
- ``show_disk``: Display disk usage (boolean)
- ``show_uptime``: Display system uptime (boolean)
- ``show_cpu_model``: Display CPU model details (boolean)
- ``show_gpu_model``: Display GPU model details (boolean)
- ``show_kernel_version``: Display kernel version (boolean)
- ``show_display_server``: Display display server info (boolean)
- ``show_desktop_environment``: Display desktop environment (boolean)
- ``show_username``: Display username (boolean)
- ``show_hostname``: Display hostname (boolean)
- ``show_disks``: Display disk information (boolean)

**Layout Settings**

- ``info_keys``: Array defining the order of displayed information
- ``layout``: Layout type (currently supports "default")

**Customization Settings**

- ``color_scheme``: Name of the color scheme
- ``font_size``: Font size for output (integer)
- ``alignment``: Text alignment ("left", "center", "right")

**Color Settings**

- ``[colors]``: Section containing hex color codes for each information type
- Use 6-digit hexadecimal codes (#RRGGBB) for all colors

**Symbol Settings**

- ``[symbols]``: Section containing emoji or text symbols for each field
- Supports Unicode emojis and custom text symbols

Pre-made Color Schemes
----------------------

Piko comes with several pre-made color schemes:

- **Default (Dracula-inspired)**: Modern vibrant colors
- **Pastel**: Soft and elegant pastel colors
- **Dark**: Professional dark theme

To use a different color scheme:

.. code-block:: bash

   piko --config config/pastel_config.toml
   piko --config config/dark_config.toml

Customizing Configuration
-------------------------

To customize Piko's behavior:

1. Copy the default configuration file:
   .. code-block:: bash

      cp /usr/share/piko/default_config.toml ~/.config/piko/my_config.toml

2. Edit the configuration file with your preferred settings
3. Use your custom configuration:
   .. code-block:: bash

      piko --config ~/.config/piko/my_config.toml

Troubleshooting
---------------

- Ensure the file is in valid TOML format
- Check for typos in option names
- Verify that color codes are in valid hexadecimal format (#RRGGBB)
- Refer to the default configuration file as a guide
- If the configuration file is not found, ensure it was properly installed
