Usage Guide
===========

This guide covers command-line options, themes, formats, and integration examples for Piko.

Basic Invocation
----------------

Running ``piko`` without arguments detects your current operating system and displays system metrics with standard formatting:

.. code-block:: bash

    piko

Command-Line Options
--------------------

Configuration
~~~~~~~~~~~~~

- ``-c, --config <PATH>``: Load a specific configuration TOML file
- ``--gen-config``: Generate a default configuration file in ``~/.config/piko/config.toml``
- ``--print-config``: Print the default configuration file to stdout
- ``--export-config <PATH>``: Serialize and save the current configuration to a file
- ``--import-config <PATH>``: Load and apply a configuration from a file

Themes and Aesthetics
~~~~~~~~~~~~~~~~~~~~~

- ``-t, --theme <NAME>``: Select a color theme (``catppuccin``, ``catppuccin-latte``, ``dracula``, ``nord``, ``gruvbox``, ``tokyo-night``, ``one-dark``, ``solarized``, ``rose-pine``, ``synthwave``)
- ``--list-themes``: List all available built-in color themes
- ``--color-blocks <BOOL>``: Toggle display of color palette blocks (``true`` or ``false``)

Logos
~~~~~

- ``--ascii-distro, --distro <NAME>``: Manually specify the distribution logo (e.g. ``arch``, ``nixos``, ``debian``, ``fedora``, ``macos``)
- ``--logo-only``: Display only the distribution ASCII logo
- ``--list-logos``: List all 230+ available distribution logos
- ``--logo-position <POSITION>``: Position of the logo relative to text (``left``, ``right``, ``top``, ``bottom``)
- ``--logo-size <SIZE>``: Logo size variant (``small``, ``medium``, ``large``)
- ``--logo-style <STYLE>``: Logo rendering style (``ascii``, ``unicode``, ``minimal``)

Display and Borders
~~~~~~~~~~~~~~~~~~~

- ``-b, --border``: Wrap the output inside a box border
- ``--border-style <STYLE>``: Box drawing style (``single``, ``double``, ``rounded``, ``bold``)
- ``--no-separators``: Hide the colon separators between labels and values

Output Formats
~~~~~~~~~~~~~~

- ``-f, --format <FORMAT>``: Output format (``normal``, ``json``, ``yaml``)

Output Formats
--------------

Terminal ANSI (Default)
~~~~~~~~~~~~~~~~~~~~~~~

Formatted for terminal display with ANSI 24-bit color:

.. code-block:: bash

    piko

JSON Output
~~~~~~~~~~~

Structured JSON for scripts, pipelines, and monitoring agents:

.. code-block:: bash

    piko --format json

Example:

.. code-block:: json

    {
      "OS": "NixOS 26.05 (Yarara) x86_64",
      "Host": "20LJS43V00 ThinkPad X380 Yoga",
      "Kernel": "7.1.5",
      "Uptime": "1h 54m",
      "Packages": "1339 (nix-system), 5 (nix-user), 11 (flatpak)",
      "Shell": "zsh 5.9.1",
      "DE": "niri",
      "WM": "niri",
      "Terminal": "alacritty",
      "CPU": "Intel Core i5-8350U @ 1.70GHz (8) @ 3.10GHz",
      "GPU": "Intel Graphics Controller",
      "Memory": "4.00 GiB / 15.37 GiB (26%)",
      "Disk": "/boot: 0.15 GiB / 1.00 GiB (15%) [vfat]\n/home: 459.62 GiB / 1262.02 GiB (36%) [btrfs]",
      "Battery": "100% [Full]"
    }

YAML Output
~~~~~~~~~~~

Human-readable YAML format:

.. code-block:: bash

    piko --format yaml

Common Examples
---------------

Quick Overview with Compact Logo
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

.. code-block:: bash

    piko --logo-size small

Bordered Output with Nord Theme
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

.. code-block:: bash

    piko --theme nord --border --border-style rounded

Inspect Available Distribution Logos
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

.. code-block:: bash

    piko --list-logos

Querying Fields in Shell Scripts
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

Using ``jq`` with Piko's JSON output:

.. code-block:: bash

    # Extract RAM utilization
    piko --format json | jq -r '.Memory'

    # Extract battery status
    piko --format json | jq -r '.Battery'

