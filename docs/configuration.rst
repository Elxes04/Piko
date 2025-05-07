Configuration Options for Piko
==============================

Piko allows users to customize its behavior through a configuration file. This document outlines the available configuration options and how to set them.

Configuration File Location
---------------------------

The default configuration file is located at:

- **macOS**: ``~/.config/piko/default_config.toml``
- **Linux**: ``/etc/piko/default_config.toml``

If the configuration file does not exist, Piko will attempt to download a default one automatically.

Configuration Format
--------------------

The configuration file is written in TOML format. Example:

.. code-block:: toml

   # Example configuration file for Piko

   [general]
   option1 = "value1"
   option2 = "value2"

   [system]
   show_os_info = true
   show_hardware_info = false

   [output]
   output_format = "pretty"  # Options: "pretty", "json", "yaml"

Configuration Options
---------------------

**General Settings**

- ``option1``: Description of option1.
- ``option2``: Description of option2.

**System Settings**

- ``show_os_info``: Display OS information (boolean).
- ``show_hardware_info``: Display hardware info (boolean).

**Output Settings**

- ``output_format``:
  - ``pretty``: Human-readable format.
  - ``json``: JSON format.
  - ``yaml``: YAML format.

Customizing Configuration
-------------------------

To customize Piko's behavior, edit the configuration file. Save and restart Piko to apply changes.

Troubleshooting
---------------

- Ensure the file is in valid TOML format.
- Check for typos in option names.
- Refer to the default configuration file as a guide.
