Installation Guide
==================

This guide covers how to install Piko on Linux, macOS, and Termux.

Prerequisites
-------------

Piko is written in Rust and requires a working Rust toolchain (Cargo and Rustc 1.75+).

If you do not have Rust installed:

.. code-block:: bash

    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    source "$HOME/.cargo/env"

Building from Source
--------------------

1. Clone the repository:

   .. code-block:: bash

       git clone https://github.com/Elxes04/piko.git
       cd piko

2. Build and install with Cargo:

   .. code-block:: bash

       cargo build --release
       sudo cp target/release/piko /usr/local/bin/

   Alternatively, install directly to your Cargo bin directory:

   .. code-block:: bash

       cargo install --path .

3. Or run the install script:

   .. code-block:: bash

       ./install.sh

Install via Cargo
-----------------

If installing from crates.io:

.. code-block:: bash

    cargo install piko

Configuration Setup (Optional)
------------------------------

Piko does not require any external configuration files to run. If you wish to customize fields or color schemes, generate a configuration file in your user directory:

.. code-block:: bash

    piko --gen-config

This creates `~/.config/piko/config.toml` with default settings and explanatory comments.

Verification
------------

Test that Piko is installed and executable:

.. code-block:: bash

    piko --version
    piko

Uninstallation
--------------

To remove Piko:

.. code-block:: bash

    # If installed manually:
    sudo rm /usr/local/bin/piko

    # If installed via Cargo:
    cargo uninstall piko

    # Remove user configuration (optional):
    rm -rf ~/.config/piko

