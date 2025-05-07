Installation Instructions for Piko
==================================

Prerequisites
-------------

Before installing Piko, ensure that you have the following prerequisites:

- **Rust**: Piko is built using Rust, so you need to have Rust installed on your system. You can install Rust by following the instructions on the `official Rust website <https://www.rust-lang.org/tools/install>`_.
- **Cargo**: Cargo is the Rust package manager and is included with the Rust installation. It is used to build and manage Rust projects.

Installation Steps
------------------

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

Troubleshooting
---------------

- Ensure that you have the latest version of Rust and Cargo installed.
- Check for any error messages in the terminal and refer to the Rust documentation for guidance.

Verification
------------

To verify that Piko has been installed correctly, run:

.. code-block:: bash

   piko --version
