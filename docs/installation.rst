Installation Guide
=================

This guide covers how to install Piko on different systems and platforms.

Prerequisites
------------

**Rust Programming Language**
Piko is written in Rust and requires Rust 1.75+ to build from source.

To install Rust, visit `https://rustup.rs/` or run:

.. code-block:: bash

    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

After installation, restart your terminal or run:

.. code-block:: bash

    source "$HOME/.cargo/env"

Installation Methods
-------------------

From Source (Recommended)
~~~~~~~~~~~~~~~~~~~~~~~~~

1. **Clone the repository:**

   .. code-block:: bash

       git clone https://github.com/Elxes04/piko.git
       cd piko

2. **Run the installation script:**

   .. code-block:: bash

       ./install.sh

   This script will:
   - Build Piko from source
   - Install the binary to `/usr/local/bin/`
   - Install configuration files to `/etc/piko/`
   - Set appropriate permissions

3. **Verify installation:**

   .. code-block:: bash

       piko --version

Manual Installation
~~~~~~~~~~~~~~~~~~

If you prefer manual installation:

1. **Build the project:**

   .. code-block:: bash

       cargo build --release

2. **Install binary:**

   .. code-block:: bash

       sudo cp target/release/piko /usr/local/bin/

3. **Create configuration directory:**

   .. code-block:: bash

       sudo mkdir -p /etc/piko

4. **Install configuration files:**

   .. code-block:: bash

       sudo cp config/default_config.toml /etc/piko/
       sudo cp config/compact_config.toml /etc/piko/
       sudo cp config/border_config.toml /etc/piko/

From Cargo
~~~~~~~~~~

Install directly from Cargo registry:

.. code-block:: bash

    cargo install piko

**Note:** When installing from Cargo, you'll need to manually copy configuration files or create your own.

Package Managers
~~~~~~~~~~~~~~~

**Arch Linux (AUR):**

.. code-block:: bash

    yay -S piko



Installation Locations
---------------------

After installation, Piko will be available in the following locations:

- **Binary:** `/usr/local/bin/piko`
- **System Configuration:** `/etc/piko/`
- **User Configuration:** `~/.config/piko/` (macOS)

Configuration Files
------------------

The following configuration files are installed by default:

- **`default_config.toml`** - Default Dracula-inspired theme
- **`compact_config.toml`** - Compact layout theme
- **`border_config.toml`** - Bordered output theme

Post-Installation
-----------------

1. **Test the installation:**

   .. code-block:: bash

       piko

2. **List available logos:**

   .. code-block:: bash

       piko --list-logos

3. **Show help:**

   .. code-block:: bash

       piko --help

Troubleshooting
--------------

**Permission Denied Errors**
If you encounter permission errors, ensure the binary is executable:

.. code-block:: bash

    sudo chmod +x /usr/local/bin/piko

**Configuration File Not Found**
If Piko can't find the configuration file:

1. Verify the file exists: ``ls -la /etc/piko/``
2. Check file permissions: ``ls -la /etc/piko/default_config.toml``
3. Ensure the file is readable: ``sudo chmod 644 /etc/piko/default_config.toml``

**Rust Not Found**
If you get "cargo: command not found":

.. code-block:: bash

    source "$HOME/.cargo/env"

Or restart your terminal after installing Rust.

Uninstallation
--------------

To remove Piko:

1. **Remove the binary:**

   .. code-block:: bash

       sudo rm /usr/local/bin/piko

2. **Remove configuration files (optional):**

   .. code-block:: bash

       sudo rm -rf /etc/piko

3. **Remove from Cargo (if installed via Cargo):**

   .. code-block:: bash

       cargo uninstall piko

Next Steps
----------

After installation, you can:

- Read the :doc:`configuration` guide to customize Piko
- Check the :doc:`usage` guide for command-line options
- Explore :doc:`advanced_features` for advanced customization

For additional help, please open an issue on the `GitHub repository <https://github.com/Elxes04/piko/issues>`_.
