Installation Instructions for Piko
==================================

Prerequisites
-------------

Before installing Piko, ensure that you have the following prerequisites:

- **Rust**: Piko is built using Rust, so you need to have Rust installed on your system. You can install Rust by following the instructions on the `official Rust website <https://www.rust-lang.org/tools/install>`_.
- **Cargo**: Cargo is the Rust package manager and is included with the Rust installation. It is used to build and manage Rust projects.

Installation Methods
-------------------

Piko can be installed in several ways:

From Source
~~~~~~~~~~~

1. **Clone the Repository**:

   Open your terminal and clone the Piko repository using the following command:

   .. code-block:: bash

      git clone https://github.com/Elxes04/Piko.git

2. **Navigate to the Project Directory**:

   .. code-block:: bash

      cd Piko

3. **Build the Project**:

   .. code-block:: bash

      cargo build --release

4. **Install the Executable**:

   .. code-block:: bash

      cargo install --path .

Package Installation
~~~~~~~~~~~~~~~~~~~~

When Piko is installed via package manager, the default configuration file is automatically copied to `/usr/share/piko/default_config.toml` and will be used automatically.

Manual Installation
~~~~~~~~~~~~~~~~~~~

To install manually, copy the binary and configuration file:

.. code-block:: bash

   sudo cp target/release/piko /usr/local/bin/
   sudo mkdir -p /usr/share/piko
   sudo cp config/default_config.toml /usr/share/piko/

Configuration File Locations
---------------------------

After installation, Piko will automatically load the default configuration from:

- **Package installation**: `/usr/share/piko/default_config.toml`
- **macOS**: `~/.config/piko/default_config.toml`
- **Custom path**: Use `--config /path/to/config.toml`

The configuration file is included during installation and no longer needs to be downloaded from GitHub.

Troubleshooting
---------------

- Ensure that you have the latest version of Rust and Cargo installed.
- Check for any error messages in the terminal and refer to the Rust documentation for guidance.
- If the configuration file is not found, ensure it was properly copied during installation.

Verification
------------

To verify that Piko has been installed correctly, run:

.. code-block:: bash

   piko --version

To test the installation with the default configuration:

.. code-block:: bash

   piko
