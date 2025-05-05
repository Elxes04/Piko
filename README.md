## Overview
`Piko` is a command-line application written in Rust that gathers and displays system information in a customizable format. Inspired by tools like Neofetch, this application allows users to view details about their operating system, CPU, memory usage, and more, with the ability to customize the output layout through a configuration file.

## Features
- Gather system information such as:
  - Operating System
  - CPU details
  - Memory usage
  - Disk usage
  - Network information (WIP)
- Customizable output layout (WIP)
- Cross-platform support for Linux and macOS

## Project Structure
```
rust-system-info
├── src
│   ├── main.rs          # Entry point of the application
│   ├── system_info.rs   # Functions to gather system information
│   ├── output.rs        # Functions to format and display output
│   └── config.rs        # Configuration handling
├── Cargo.toml           # Cargo configuration file
├── config
│   ├── default_config.toml  # Default configuration settings
│   └── README.md          # Configuration customization documentation
└── README.md             # Project documentation
```

## Installation
To build and run the application, ensure you have Rust installed on your system. You can install Rust using [rustup](https://rustup.rs/).

Clone the repository and navigate to the project directory:

```bash
git clone https://github.com/Elxes04/piko.git
cd rust-system-info
```

Build the project using Cargo:

```bash
cargo build --release
```

## Usage
After building the project, you can run the application with:

```bash
./target/release/rust-system-info
```

The application will read the configuration from `config/default_config.toml` and display the system information according to the specified layout.

## Configuration
The layout of the output can be customized by editing the `config/default_config.toml` file. Refer to `config/README.md` for detailed instructions on available options and how to modify them.

## Contributing
Contributions are welcome! If you have suggestions for improvements or new features, please open an issue or submit a pull request.

## License
This project is licensed under the MIT License. See the LICENSE file for more details.
