Usage of Piko
=============

Piko is a minimal, customizable system information tool written in Rust — inspired by Neofetch. This document outlines how to use the Piko program effectively.

Command-Line Options
--------------------

Piko can be run with various command-line options. The following options are available:

- ``--config <path>``: Specify a custom path to a configuration file. If not provided, Piko will use the default located at:
  - **Package installation**: `/usr/share/piko/default_config.toml`
  - **macOS**: `~/.config/piko/default_config.toml`

Running Piko
------------

Basic Usage
~~~~~~~~~~~

To run Piko with the default configuration:

.. code-block:: bash

   piko

Using a custom configuration file:

.. code-block:: bash

   piko --config /path/to/your/config.toml

Using Pre-made Color Schemes
~~~~~~~~~~~~~~~~~~~~~~~~~~~~

Piko comes with several pre-made color schemes in the `config/` directory:

.. code-block:: bash

   # Default scheme (Dracula-inspired)
   piko

   # Pastel scheme
   piko --config config/pastel_config.toml

   # Dark scheme
   piko --config config/dark_config.toml

Examples
--------

**Basic Usage**

.. code-block:: bash

   piko

**Using a Custom Configuration**

.. code-block:: bash

   piko --config /path/to/custom_config.toml

**Using Pre-made Color Schemes**

.. code-block:: bash

   piko --config config/pastel_config.toml
   piko --config config/dark_config.toml

**Creating and Using Your Own Scheme**

.. code-block:: bash

   cp config/default_config.toml ~/.config/piko/my_scheme.toml
   piko --config ~/.config/piko/my_scheme.toml

Features
--------

Piko provides various features, including:

- Displaying comprehensive system information such as OS, kernel version, and hardware specifications
- Multiple pre-made color schemes for different preferences
- Customizable output format through configuration files
- Fast and lightweight performance written in Rust
- Cross-platform support for Linux and macOS
- Easy installation and setup process

For more configuration options, see the :doc:`configuration` page.
