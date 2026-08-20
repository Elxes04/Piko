Configuration Guide
===================

Piko uses TOML for configuration. It runs with zero configuration by default using embedded settings, but can be fully customized with a local configuration file.

Configuration Resolution Order
------------------------------

Piko resolves configuration files in the following order:

1. Path supplied to the ``--config <path>`` command-line flag
2. Path defined in the ``PIKO_CONFIG`` environment variable
3. User configuration file: ``~/.config/piko/config.toml``
4. System-wide configuration file: ``/etc/piko/config.toml``
5. Built-in embedded default configuration

Generating a Default Configuration
----------------------------------

You can generate a starter configuration file at ``~/.config/piko/config.toml``:

.. code-block:: bash

    piko --gen-config

Or print the embedded default configuration to stdout:

.. code-block:: bash

    piko --print-config

Configuration File Structure
----------------------------

.. code-block:: toml

    layout = "default"

    [output]
    # List of fields to display, in order
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

    # Logo settings
    show_logo = true
    logo_position = "left"          # "left", "right", "top", "bottom"
    logo_size = "medium"            # "small", "medium", "large"
    logo_style = "ascii"            # "ascii", "unicode", "minimal"

    # Header and palette settings
    show_title = true               # user@hostname header
    show_title_separator = true     # underline delimiter
    show_color_blocks = true        # color palette blocks
    color_blocks_style = "circle"   # "circle" (●) or "block" (█)

    [logo]
    enabled = true
    distro = "auto"                 # "auto" or specific distro name (e.g. "arch", "nixos")
    padding = 2
    alignment = "left"

    [theme]
    name = "catppuccin"
    primary_color = "#CBA6F7"
    secondary_color = "#89B4FA"
    accent_color = "#A6E3A1"
    text_color = "#CDD6F4"

    [display]
    border = false
    border_style = "rounded"        # "single", "double", "rounded", "bold", "none"
    border_color = "#6C7086"
    padding = 1
    margin = 0
    alignment = "left"
    compact = false
    show_separators = true
    separator_symbol = ":"

    [colors]
    OS = "#CBA6F7"
    Host = "#89B4FA"
    Kernel = "#F38BA8"
    Uptime = "#FAB387"
    Packages = "#F9E2AF"
    Shell = "#A6E3A1"
    DE = "#94E2D5"
    WM = "#89DCEB"
    Terminal = "#74C7EC"
    CPU = "#89B4FA"
    GPU = "#B4BEFE"
    Memory = "#A6E3A1"
    Disk = "#F38BA8"
    Battery = "#A6E3A1"

    [symbols]
    OS = ""
    Host = "󰌢"
    Kernel = ""
    Uptime = "󰅐"
    Packages = "󰏖"
    Shell = ""
    DE = "󰧨"
    WM = ""
    Terminal = ""
    CPU = ""
    GPU = "󰢮"
    Memory = "󰍛"
    Disk = "󰋊"
    Battery = "󰁹"

Supported Information Keys
--------------------------

The ``info_keys`` array supports any combination of the following identifiers:

- ``"OS"``: Distribution name, version release, and architecture
- ``"Host"``: Hardware / laptop model (e.g. ThinkPad X380 Yoga)
- ``"Kernel"``: Operating system kernel release
- ``"Uptime"``: System uptime formatted in days, hours, and minutes
- ``"Packages"``: Count of installed packages across detected package managers
- ``"Shell"``: Active shell name and version
- ``"DE"``: Desktop Environment (e.g. GNOME, KDE Plasma, XFCE)
- ``"WM"``: Window Manager or Wayland compositor (e.g. niri, Hyprland, Sway, i3)
- ``"Terminal"``: Detected terminal emulator
- ``"CPU"``: Processor model, core count, and frequency
- ``"GPU"``: Graphics card / controller
- ``"Memory"``: Used and total RAM with usage percentage
- ``"Disk"``: Deduplicated physical mount points and usage
- ``"Battery"``: Battery charge percentage and charging status

Built-in Themes
---------------

You can select a built-in theme by name in your config or via ``piko --theme <name>``:

- ``catppuccin`` (Mocha)
- ``catppuccin-latte``
- ``dracula``
- ``nord``
- ``gruvbox``
- ``tokyo-night``
- ``one-dark``
- ``solarized``
- ``rose-pine``
- ``synthwave``

Border Styles
-------------

The ``display.border_style`` option supports:

- ``"single"``: Clean single-line box drawing (┌─┐)
- ``"double"``: Double-line box drawing (╔═╗)
- ``"rounded"``: Smooth rounded corners (╭─╮)
- ``"bold"``: Heavy-line box drawing (┏━┓)
- ``"none"``: Space boundary without box characters

Exporting and Importing
-----------------------

Export the active configuration (including any CLI overrides applied):

.. code-block:: bash

    piko --theme nord --border --export-config ~/nord_theme.toml

Load a custom configuration file:

.. code-block:: bash

    piko --config ~/nord_theme.toml

