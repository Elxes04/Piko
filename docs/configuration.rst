Configuration Guide
==================

Piko is highly customizable through TOML configuration files. This guide covers all configuration options and how to use them effectively.

Configuration File Locations
---------------------------

Piko looks for configuration files in the following order:

1. **Custom path** (specified with `--config`)
2. **System configuration** (`/etc/piko/default_config.toml`)
3. **User configuration** (`~/.config/piko/default_config.toml` on macOS)

Basic Configuration Structure
----------------------------

A Piko configuration file has the following structure:

.. code-block:: toml

    # Main layout configuration
    layout = "default"
    
    [output]
    # Output display settings
    
    [logo]
    # Logo configuration
    
    [theme]
    # Theme and color settings
    
    [display]
    # Display and formatting options
    
    [colors]
    # Individual color overrides
    
    [symbols]
    # Custom symbols for information fields

Output Configuration
-------------------

The `[output]` section controls how information is displayed:

.. code-block:: toml

    [output]
    # Logo settings
    show_logo = true
    logo_position = "left"      # left, right, top, bottom
    logo_size = "medium"        # small, medium, large
    logo_style = "ascii"        # ascii, unicode, minimal
    
    # Information fields to display
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
        "Disk"
    ]
    
    # Show/hide specific fields
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

Logo Configuration
-----------------

The `[logo]` section controls the distribution logo display:

.. code-block:: toml

    [logo]
    enabled = true
    distro = "auto"             # auto, arch, ubuntu, debian, fedora, etc.
    padding = 2                  # Space between logo and info
    alignment = "left"           # left, center, right
    
    # Custom ASCII art (optional)
    custom_ascii = [
        "  _____",
        " /  ___|",
        "|  |  __",
        "|  | |_ |",
        "|  |__| |",
        " \\_____|"
    ]
    
    # Custom colors for ASCII art (optional)
    custom_colors = ["#D70A53"]

Theme Configuration
------------------

The `[theme]` section defines color schemes:

.. code-block:: toml

    [theme]
    name = "default"             # default, dracula, nord, gruvbox, custom
    primary_color = "#FF79C6"    # Main accent color
    secondary_color = "#50FA7B"  # Secondary accent color
    accent_color = "#BD93F9"     # Highlight color
    background_color = ""        # Background color (empty for transparent)
    text_color = "#F8F8F2"      # Default text color

Display Configuration
--------------------

The `[display]` section controls visual formatting:

.. code-block:: toml

    [display]
    # Border settings
    border = false
    border_style = "single"      # single, double, rounded, none
    border_color = "#6272A4"
    
    # Spacing
    padding = 1                  # Internal padding
    margin = 0                   # External margin
    
    # Layout
    alignment = "left"           # left, center, right
    show_separators = true       # Show separators between info lines
    separator_style = "dash"     # dash, equals, dots, none

Color Configuration
------------------

The `[colors]` section allows fine-grained color control:

.. code-block:: toml

    [colors]
    # System information colors
    OS = "#FF79C6"               # Pink
    Username = "#50FA7B"         # Green
    Hostname = "#F1FA8C"         # Yellow
    "Desktop Environment" = "#BD93F9"  # Purple
    Memory = "#8BE9FD"           # Cyan
    Disk = "#FF5555"             # Red
    "CPU Model" = "#6272A4"      # Blue-grey
    "GPU Model" = "#FFB86C"      # Orange
    "Kernel Version" = "#44475A" # Dark grey
    "Display Server" = "#A4FFFF" # Light cyan
    Uptime = "#FF92DF"           # Light pink

Symbol Configuration
-------------------

The `[symbols]` section defines custom symbols for information fields:

.. code-block:: toml

    [symbols]
    OS = "🖥️"
    Username = "👤"
    Hostname = "🏠"
    "Desktop Environment" = "🌠"
    Memory = "💾"
    Disk = "📂"
    "CPU Model" = "🧪"
    "GPU Model" = "🖨️"
    "Kernel Version" = "💻"
    "Display Server" = "🖱️"
    Uptime = "⏰"

Predefined Themes
-----------------

Piko comes with several built-in themes:

**Default Theme (Dracula-inspired):**
- Primary: Pink (#FF79C6)
- Secondary: Green (#50FA7B)
- Accent: Purple (#BD93F9)

**Compact Theme:**
- Primary: Teal (#4ECDC4)
- Secondary: Blue (#45B7D1)
- Accent: Green (#96CEB4)

**Border Theme:**
- Primary: Red (#FF6B6B)
- Secondary: Teal (#4ECDC4)
- Accent: Blue (#45B7D1)

Creating Custom Themes
---------------------

To create a custom theme:

1. **Copy an existing theme file:**
   .. code-block:: bash

       cp /etc/piko/default_config.toml ~/my_theme.toml

2. **Modify colors and settings:**
   .. code-block:: toml

       [theme]
       name = "my_custom_theme"
       primary_color = "#FF6B6B"
       secondary_color = "#4ECDC4"
       accent_color = "#45B7D1"

3. **Use your custom theme:**
   .. code-block:: bash

       piko --config ~/my_theme.toml

Configuration Examples
---------------------

**Minimal Configuration:**
.. code-block:: toml

    [output]
    logo_size = "small"
    logo_style = "minimal"
    
    [display]
    show_separators = false
    padding = 0

**Professional Configuration:**
.. code-block:: toml

    [display]
    border = true
    border_style = "double"
    border_color = "#FF6B6B"
    padding = 2
    
    [output]
    logo_size = "large"
    logo_style = "ascii"

**Colorful Configuration:**
.. code-block:: toml

    [theme]
    name = "vibrant"
    primary_color = "#FF6B6B"
    secondary_color = "#4ECDC4"
    accent_color = "#45B7D1"
    
    [colors]
    OS = "#FF6B6B"
    Username = "#4ECDC4"
    Hostname = "#45B7D1"

Environment Variables
--------------------

You can override configuration with environment variables:

.. code-block:: bash

    export PIKO_CONFIG_PATH="/path/to/config.toml"
    export PIKO_LOGO_SIZE="large"
    export PIKO_SHOW_BORDER="true"

Configuration Validation
-----------------------

Piko validates configuration files and will show errors for invalid TOML syntax. Common issues:

- **Inline comments:** TOML doesn't support `#` on the same line as values
- **Missing quotes:** Use quotes for strings with spaces
- **Invalid colors:** Ensure hex colors are valid (e.g., `#FF0000`)

Best Practices
--------------

1. **Backup configurations:** Keep backups of working configurations
2. **Test changes:** Test configuration changes before applying to production
3. **Use version control:** Track configuration changes in git
4. **Document customizations:** Add comments to explain custom settings
5. **Share themes:** Share your custom themes with the community

Troubleshooting
---------------

**Configuration not loading:**
- Check file permissions: `ls -la /etc/piko/`
- Verify TOML syntax: `toml-validate config.toml`
- Check file path: `piko --config /path/to/config.toml`

**Colors not working:**
- Ensure terminal supports true color
- Check hex color format: `#RRGGBB`
- Verify color names are correct

**Logo not displaying:**
- Check `show_logo = true` in output section
- Verify logo is enabled: `[logo] enabled = true`
- Test with `piko --logo-only`

For more advanced configuration options, see the :doc:`advanced_features` guide.
