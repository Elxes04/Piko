Installation Guide
==================

This guide covers how to install Piko on Linux, macOS, and Termux.

Prerequisites
-------------

Piko is written in Rust and can be built using Cargo or installed directly using Nix flakes.

Nix / NixOS Flake
-----------------

Run Piko directly with Nix:

.. code-block:: bash

    nix run github:Elxes04/Piko

Install to user profile:

.. code-block:: bash

    nix profile install github:Elxes04/Piko

Add to your NixOS flake configuration:

.. code-block:: nix

    {
      inputs = {
        nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
        piko.url = "github:Elxes04/Piko";
      };

      outputs = { self, nixpkgs, piko, ... }: {
        nixosConfigurations.myhost = nixpkgs.lib.nixosSystem {
          modules = [
            ({ pkgs, ... }: {
              environment.systemPackages = [
                piko.packages.${pkgs.stdenv.hostPlatform.system}.default
              ];
            })
          ];
        };
      };
    }

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

