# Installation Instructions for Piko

## Prerequisites

Before installing Piko, ensure that you have the following prerequisites:

- **Rust**: Piko is built using Rust, so you need to have Rust installed on your system. You can install Rust by following the instructions on the [official Rust website](https://www.rust-lang.org/tools/install).
- **Cargo**: Cargo is the Rust package manager and is included with the Rust installation. It is used to build and manage Rust projects.

## Installation Steps

1. **Clone the Repository**:
   Open your terminal and clone the Piko repository using the following command:
   ```
   git clone https://github.com/Elxes04/Piko.git
   ```

2. **Navigate to the Project Directory**:
   Change into the Piko directory:
   ```
   cd Piko
   ```

3. **Build the Project**:
   Use Cargo to build the project:
   ```
   cargo build --release
   ```

4. **Install the Executable**:
   After building, you can install the Piko executable globally by running:
   ```
   cargo install --path .
   ```

## Troubleshooting

- If you encounter any issues during the installation, ensure that you have the latest version of Rust and Cargo installed.
- Check for any error messages in the terminal and refer to the Rust documentation for guidance on resolving common issues.

## Verification

To verify that Piko has been installed correctly, you can run the following command:
```
piko --version
```
This should display the version of Piko that you have installed.