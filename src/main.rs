mod config;
mod output;
mod system_info;

use clap::Parser;
use std::fs;
use std::path::PathBuf;
use toml::Value;

/// Simple Neofetch-like tool
#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    /// Path to a custom config file
    #[arg(long)]
    config: Option<PathBuf>,
}

fn get_default_config_path() -> PathBuf {
    if cfg!(target_os = "macos") {
        dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("piko")
            .join("default_config.toml")
    } else {
        PathBuf::from("/usr/share/piko/default_config.toml")
    }
}

fn ensure_default_config_exists() {
    let config_path = get_default_config_path();

    if !config_path.exists() {
        eprintln!("Configuration file not found at {}", config_path.display());
        eprintln!("Please ensure Piko is properly installed.");
        std::process::exit(1);
    }
}

fn load_config(config_path: Option<PathBuf>) -> Value {
    let path = config_path.unwrap_or_else(get_default_config_path);

    let contents = fs::read_to_string(&path)
        .unwrap_or_else(|_| panic!("Unable to read the configuration file from {:?}", path));

    contents.parse().expect("Invalid TOML configuration")
}

fn main() {
    let cli = Cli::parse();

    if cli.config.is_none() {
        ensure_default_config_exists();
    }

    let config_value = load_config(cli.config);

    let config = config::Config::from_value(&config_value)
        .expect("Unable to deserialize the configuration file");

    let system_info = system_info::get_system_info();
    output::display_output(&system_info, &config_value);
}
