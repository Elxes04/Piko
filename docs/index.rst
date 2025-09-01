Welcome to Piko's documentation!
===============================

Piko is a highly customizable system information tool written in Rust — inspired by Neofetch.

.. toctree::
   :maxdepth: 2
   :caption: Contents:

   installation
   configuration
   usage
   ADVANCED_FEATURES

Features
--------

- **Automatic Distribution Detection**: Automatically detects and displays your Linux distribution logo
- **Highly Customizable**: Extensive configuration options for colors, symbols, layout, and more
- **Multiple Output Formats**: Support for normal, JSON, and YAML output
- **Logo Support**: Built-in ASCII art logos for popular Linux distributions
- **Flexible Layout**: Configurable logo position, size, and style
- **Theme Support**: Multiple color themes and custom color schemes
- **Export/Import**: Save and load your custom configurations
- **Border Options**: Add decorative borders around your output

Quick Start
-----------

.. code-block:: bash

   # Show system information with default settings
   piko

   # Show only the distribution logo
   piko --logo-only

   # List all available distribution logos
   piko --list-logos

   # Get help
   piko --help

Installation
-----------

For installation instructions, see the :doc:`installation` guide.

Configuration
------------

For configuration options and customization, see the :doc:`configuration` guide.

Usage
-----

For command-line options and usage examples, see the :doc:`usage` guide.

Advanced Features
----------------

For advanced customization, automation, and integration, see the :doc:`ADVANCED_FEATURES` guide.

Indices and tables
==================

* :ref:`genindex`
* :ref:`modindex`
* :ref:`search`
