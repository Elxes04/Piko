mod config;
mod system_info;
mod output;

use clap::Parser;
use std::env;
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

fn get_default_config_path() -> Option<PathBuf> {
    if let Ok(xdg) = env::var("XDG_CONFIG_HOME") {
        Some(PathBuf::from(xdg).join("piko").join("default_config.toml"))
    } else if let Ok(home) = env::var("HOME") {
        Some(PathBuf::from(home).join(".config").join("piko").join("default_config.toml"))
    } else {
        None
    }
}

fn load_config_with_fallback(config_path: Option<PathBuf>) -> Value {
    if let Some(path) = config_path {
        let contents = fs::read_to_string(&path)
            .unwrap_or_else(|_| panic!("Failed to read config from {:?}", path));
        contents.parse().expect("Invalid TOML in config file")
    } else if let Some(default_path) = get_default_config_path() {
        if default_path.exists() {
            let contents = fs::read_to_string(&default_path)
                .expect("Failed to read default config");
            contents.parse().expect("Invalid TOML in default config")
        } else {
            // fallback: use embedded config
            const DEFAULT_CONFIG: &str = include_str!("../config/default_config.toml");
            DEFAULT_CONFIG.parse().expect("Failed to parse embedded default config")
        }
    } else {
        panic!("No suitable config path found.");
    }
}

fn main() {
    let cli = Cli::parse();

    let config_value = load_config_with_fallback(cli.config);

    let config = config::Config::from_value(&config_value)
        .expect("Failed to deserialize config");

    let system_info = system_info::get_system_info();
    output::display_output(&system_info, &config_value);
}
