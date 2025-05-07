Usage of Piko
=============

Piko is a Neofetch-like tool that provides system information in a user-friendly format. This document outlines how to use the Piko program effectively.

Command-Line Options
--------------------

Piko can be run with various command-line options. The following options are available:

- ``--config <path>``: Specify a custom path to a configuration file. If not provided, Piko will use the default located at ``~/.config/piko/default_config.toml`` (macOS) or ``/etc/piko/default_config.toml`` (Linux).

Running Piko
------------

To run Piko, simply execute the following command:

.. code-block:: bash

   piko

Using a custom configuration file:

.. code-block:: bash

   piko --config /path/to/your/config.toml

Examples
--------

**Basic Usage**

.. code-block:: bash

   piko

**Using a Custom Configuration**

.. code-block:: bash

   piko --config /path/to/custom_config.toml

Features
--------

Piko provides various features, including:

- Displaying system information such as OS, kernel version, and hardware specifications.
- Customizable output format through configuration files.
- Easy installation and setup process.

For more configuration options, see the :doc:`configuration` page.
