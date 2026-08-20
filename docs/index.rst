Piko Documentation
==================

Piko is a lightweight, customizable system information tool written in Rust, inspired by Neofetch.

.. toctree::
   :maxdepth: 2
   :caption: Contents:

   installation
   configuration
   usage

Features
--------

- Fast execution (<10ms) using direct procfs/sysfs parsers
- Zero-config execution with embedded defaults
- 10 built-in color themes
- 230+ distribution ASCII logos with authentic Neofetch color palettes and small variants
- Detailed system metrics: OS, Host/Model, Kernel, Uptime, Packages (11 package managers), Shell, DE, WM, Terminal, CPU, GPU, Memory, Disks, and Battery
- Terminal color blocks palette
- Border formatting (single, double, rounded, bold)
- Normal ANSI, JSON, and YAML output formats

Quick Start
-----------

.. code-block:: bash

   # Display system info using default settings
   piko

   # Switch color themes
   piko --theme nord

   # Use a small logo with a border
   piko --distro arch --logo-size small --border

   # Generate a default configuration file
   piko --gen-config

   # Print help
   piko --help

Documentation Sections
----------------------

- :doc:`installation` - How to install Piko
- :doc:`configuration` - How to configure Piko
- :doc:`usage` - Command-line options and examples

