Usage Guide
===========

This guide covers how to use Piko effectively, including all command-line options and practical examples.

Basic Usage
----------

**Show system information with default settings:**
.. code-block:: bash

    piko

**Show help:**
.. code-block:: bash

    piko --help

**Show version:**
.. code-block:: bash

    piko --version

Command Line Options
--------------------

Basic Options
~~~~~~~~~~~~

**Configuration:**
- `--config <FILE>` - Use custom configuration file
- `--export-config <FILE>` - Export current configuration to file
- `--import-config <FILE>` - Import configuration from file

**Information Display:**
- `--logo-only` - Show only the distribution logo
- `--list-logos` - List all available distribution logos
- `--format <FORMAT>` - Output format (normal, json, yaml)

Logo Options
~~~~~~~~~~~~

**Position:**
- `--logo-position <POSITION>` - Logo position (left, right, top, bottom)

**Size:**
- `--logo-size <SIZE>` - Logo size (small, medium, large)

**Style:**
- `--logo-style <STYLE>` - Logo style (ascii, unicode, minimal)

Display Options
~~~~~~~~~~~~~~

**Borders:**
- `--border` - Show border around output

**Separators:**
- `--no-separators` - Hide separators between info lines

**Layout:**
- `--compact` - Enable compact mode (deprecated, use config file)

Output Formats
--------------

Normal Output
~~~~~~~~~~~~~

Default human-readable output with colors and formatting:

.. code-block:: bash

    piko

JSON Output
~~~~~~~~~~~

Machine-readable JSON format for scripting and automation:

.. code-block:: bash

    piko --format json

Example output:
.. code-block:: json

    {
      "OS": "GNU/Linux",
      "Kernel Version": "6.16.4-arch1-1",
      "Desktop Environment": "GNOME",
      "Display Server": "Wayland",
      "Uptime": "02h 44m",
      "CPU Model": "Intel(R) Core(TM) i5-8350U CPU @ 1.70GHz",
      "Memory": "4.16 GiB / 15.36 GiB (27%)",
      "GPU Model": "Intel Corporation UHD Graphics 620 [8086:5917] (rev 07)",
      "Username": "elxes",
      "Hostname": "arch",
      "Disk": "Disk (/): 13.25 GiB / 80.00 GiB (17%) - btrfs"
    }

YAML Output
~~~~~~~~~~~

Human-readable YAML format:

.. code-block:: bash

    piko --format yaml

Example output:
.. code-block:: yaml

    OS: GNU/Linux
    Kernel Version: 6.16.4-arch1-1
    Desktop Environment: GNOME
    Display Server: Wayland
    Uptime: 02h 44m
    CPU Model: Intel(R) Core(TM) i5-8350U CPU @ 1.70GHz
    Memory: 4.16 GiB / 15.36 GiB (27%)
    GPU Model: Intel Corporation UHD Graphics 620 [8086:5917] (rev 07)
    Username: elxes
    Hostname: arch
    Disk: Disk (/): 13.25 GiB / 80.00 GiB (17%) - btrfs

Logo Display Modes
------------------

Logo Only Mode
~~~~~~~~~~~~~~

Display just the distribution logo without system information:

.. code-block:: bash

    piko --logo-only

Useful for:
- Testing logo display
- Creating custom layouts
- Minimal output needs

Logo Customization
~~~~~~~~~~~~~~~~~

**Change logo position:**
.. code-block:: bash

    # Logo on the right
    piko --logo-position right
    
    # Logo on top
    piko --logo-position top
    
    # Logo on bottom
    piko --logo-position bottom

**Change logo size:**
.. code-block:: bash

    # Small logo
    piko --logo-size small
    
    # Medium logo (default)
    piko --logo-size medium
    
    # Large logo
    piko --logo-size large

**Change logo style:**
.. code-block:: bash

    # ASCII art style
    piko --logo-style ascii
    
    # Unicode style
    piko --logo-style unicode
    
    # Minimal style
    piko --logo-style minimal

Display Customization
--------------------

Border Display
~~~~~~~~~~~~~

Add decorative borders around the output:

.. code-block:: bash

    piko --border

**Border styles** (configured in config file):
- Single line borders
- Double line borders
- Rounded corners
- Custom colors

Separator Control
~~~~~~~~~~~~~~~~

Hide separators between information lines:

.. code-block:: bash

    piko --no-separators

**Separator styles** (configured in config file):
- Dashes (`-`)
- Equals signs (`=`)
- Dots (`.`)
- Custom characters

Configuration Management
-----------------------

Export Configuration
~~~~~~~~~~~~~~~~~~~

Save your current configuration to a file:

.. code-block:: bash

    piko --export-config my_piko_config.toml

Useful for:
- Backing up configurations
- Sharing themes with others
- Version control

Import Configuration
~~~~~~~~~~~~~~~~~~~

Load a configuration from a file:

.. code-block:: bash

    piko --import-config my_piko_config.toml

Custom Configuration
~~~~~~~~~~~~~~~~~~~

Use a specific configuration file:

.. code-block:: bash

    piko --config /path/to/custom_config.toml

Common use cases:
- Different themes for different environments
- Work vs. personal configurations
- Testing new configurations

Practical Examples
-----------------

Quick System Overview
~~~~~~~~~~~~~~~~~~~~~

**Minimal output with small logo:**
.. code-block:: bash

    piko --logo-size small --no-separators

**Professional look with borders:**
.. code-block:: bash

    piko --border --logo-size large

**Compact display:**
.. code-block:: bash

    piko --config /etc/piko/compact_config.toml

Screenshots and Presentations
~~~~~~~~~~~~~~~~~~~~~~~~~~~~

**For screenshots:**
.. code-block:: bash

    piko --border --logo-size large

**For documentation:**
.. code-block:: bash

    piko --format json > system_info.json

**For scripts:**
.. code-block:: bash

    piko --format yaml > system_info.yaml

Automation and Scripting
~~~~~~~~~~~~~~~~~~~~~~~~

**Get specific information in JSON:**
.. code-block:: bash

    piko --format json | jq '.OS'

**Check system uptime:**
.. code-block:: bash

    piko --format json | jq -r '.Uptime'

**Monitor memory usage:**
.. code-block:: bash

    piko --format json | jq -r '.Memory'

**Create system report:**
.. code-block:: bash

    echo "System Report - $(date)" > system_report.txt
    piko --format yaml >> system_report.txt

Troubleshooting
--------------

Common Issues
~~~~~~~~~~~~

**Configuration not found:**
.. code-block:: bash

    # Check if config file exists
    ls -la /etc/piko/
    
    # Use specific config file
    piko --config /etc/piko/default_config.toml

**Logo not displaying:**
.. code-block:: bash

    # Test logo display
    piko --logo-only
    
    # Check logo list
    piko --list-logos

**Colors not working:**
.. code-block:: bash

    # Check terminal color support
    echo -e "\033[38;2;255;0;0mRed Text\033[0m"
    
    # Use plain output
    piko --format json

**Permission errors:**
.. code-block:: bash

    # Check binary permissions
    ls -la /usr/local/bin/piko
    
    # Fix permissions if needed
    sudo chmod +x /usr/local/bin/piko

Performance Optimization
-----------------------

**Fast startup:**
.. code-block:: bash

    # Use minimal logo
    piko --logo-size small --logo-style minimal

**Minimal output:**
.. code-block:: bash

    # Use compact config
    piko --config /etc/piko/compact_config.toml

**Script-friendly:**
.. code-block:: bash

    # JSON output for parsing
    piko --format json

Integration Examples
-------------------

**With shell scripts:**
.. code-block:: bash

    #!/bin/bash
    echo "System Information:"
    piko --logo-size small --no-separators
    
    echo -e "\nDetailed Info:"
    piko --format json | jq '.'

**With monitoring tools:**
.. code-block:: bash

    # Create monitoring script
    while true; do
        echo "$(date): $(piko --format json | jq -r '.Memory')"
        sleep 60
    done

**With documentation:**
.. code-block:: bash

    # Generate system documentation
    cat > system_doc.md << EOF
    # System Documentation
    
    ## System Information
    \`\`\`
    $(piko)
    \`\`\`
    
    ## Technical Details
    \`\`\`json
    $(piko --format json)
    \`\`\`
    EOF

Best Practices
--------------

1. **Use appropriate output formats:**
   - `normal` for human reading
   - `json` for scripting
   - `yaml` for documentation

2. **Optimize for your use case:**
   - Small logos for frequent use
   - Large logos for presentations
   - Compact configs for monitoring

3. **Leverage configuration files:**
   - Create custom themes
   - Save common configurations
   - Share configurations with team

4. **Integrate with existing tools:**
   - Use with monitoring systems
   - Include in system reports
   - Add to automation scripts

For more advanced usage patterns, see the :doc:`advanced_features` guide.
