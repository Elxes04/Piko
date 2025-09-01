mod config;
mod distro_logo;
mod output;
mod system_info;

use clap::Parser;
use std::fs;
use std::path::PathBuf;
use toml::Value;

/// A highly customizable system information tool inspired by Neofetch
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Path to a custom config file
    #[arg(long)]
    config: Option<PathBuf>,

    /// Show only the distribution logo
    #[arg(long)]
    logo_only: bool,

    /// List all available distribution logos
    #[arg(long)]
    list_logos: bool,

    /// Output format: normal, json, yaml
    #[arg(long, default_value = "normal")]
    format: String,

    /// Show logo position: left, right, top, bottom
    #[arg(long, default_value = "left")]
    logo_position: String,

    /// Logo size: small, medium, large
    #[arg(long, default_value = "medium")]
    logo_size: String,

    /// Logo style: ascii, unicode, minimal
    #[arg(long, default_value = "ascii")]
    logo_style: String,

    /// Show border around output
    #[arg(long)]
    border: bool,

    /// Hide separators between info lines
    #[arg(long)]
    no_separators: bool,

    /// Export configuration to file
    #[arg(long)]
    export_config: Option<PathBuf>,

    /// Import configuration from file
    #[arg(long)]
    import_config: Option<PathBuf>,
}

fn get_default_config_path() -> PathBuf {
    if cfg!(target_os = "macos") {
        dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("piko")
            .join("default_config.toml")
    } else {
        PathBuf::from("/etc/piko/default_config.toml")
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

fn create_dynamic_config(cli: &Cli) -> Value {
    let mut config = load_config(None);

    // Override config with CLI options
    if let Some(output) = config.get_mut("output") {
        if let Some(output_table) = output.as_table_mut() {
            output_table.insert(
                "logo_position".to_string(),
                toml::Value::String(cli.logo_position.clone()),
            );
            output_table.insert(
                "logo_size".to_string(),
                toml::Value::String(cli.logo_size.clone()),
            );
            output_table.insert(
                "logo_style".to_string(),
                toml::Value::String(cli.logo_style.clone()),
            );
        }
    }

    if let Some(display) = config.get_mut("display") {
        if let Some(display_table) = display.as_table_mut() {
            display_table.insert("border".to_string(), toml::Value::Boolean(cli.border));
            display_table.insert(
                "show_separators".to_string(),
                toml::Value::Boolean(!cli.no_separators),
            );
        }
    }

    config
}

fn export_config(config: &Value, path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let config_str = toml::to_string_pretty(config)?;
    fs::write(path, config_str)?;
    println!("Configuration exported to: {}", path.display());
    Ok(())
}

fn import_config(path: &PathBuf) -> Result<Value, Box<dyn std::error::Error>> {
    let contents = fs::read_to_string(path)?;
    let config: Value = contents.parse()?;
    println!("Configuration imported from: {}", path.display());
    Ok(config)
}

fn main() {
    let cli = Cli::parse();

    // Handle special commands first
    if cli.list_logos {
        output::list_available_logos();
        return;
    }

    // Handle import/export
    if let Some(import_path) = &cli.import_config {
        match import_config(import_path) {
            Ok(config) => {
                let system_info = system_info::get_system_info();
                match cli.format.as_str() {
                    "json" => output::display_json(&system_info),
                    "yaml" => output::display_yaml(&system_info),
                    _ => output::display_output(&system_info, &config),
                }
                return;
            }
            Err(e) => {
                eprintln!("Error importing configuration: {}", e);
                std::process::exit(1);
            }
        }
    }

    // Load configuration
    let config_value = if cli.config.is_none() && !cli.logo_only {
        ensure_default_config_exists();
        create_dynamic_config(&cli)
    } else {
        load_config(cli.config)
    };

    // Handle export
    if let Some(export_path) = &cli.export_config {
        if let Err(e) = export_config(&config_value, export_path) {
            eprintln!("Error exporting configuration: {}", e);
            std::process::exit(1);
        }
        return;
    }

    // Parse configuration
    let _config = config::Config::from_value(&config_value)
        .expect("Unable to deserialize the configuration file");

    // Handle logo-only mode
    if cli.logo_only {
        output::display_logo_only(&config_value);
        return;
    }

    // Get system information
    let system_info = system_info::get_system_info();

    // Display output based on format
    match cli.format.as_str() {
        "json" => output::display_json(&system_info),
        "yaml" => output::display_yaml(&system_info),
        _ => output::display_output(&system_info, &config_value),
    }
}
