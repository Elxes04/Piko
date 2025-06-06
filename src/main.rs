mod config;
mod system_info;
mod output;

use clap::Parser;
use reqwest::blocking::get;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;
use toml::Value;

/// Simple Neofetch-like tool
#[derive(Parser)]
#[command(author, version, about)]
struct Cli
{
    /// Path to a custom config file
    #[arg(long)]
    config: Option<PathBuf>,
}

fn get_default_config_path() -> PathBuf
{
    if cfg!(target_os = "macos")
    {
        dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("piko")
            .join("default_config.toml")
    }
    else
    {
        PathBuf::from("/etc/piko/default_config.toml")
    }
}

fn ensure_default_config_exists()
{
    let config_path = get_default_config_path();

    if !config_path.exists()
    {
        println!("Configuration file not found. Attempting to download it...");

        let parent_dir = config_path.parent().unwrap();

        // Create the directory depending on the operating system
        if cfg!(target_os = "macos")
        {
            fs::create_dir_all(parent_dir)
                .expect("Failed to create the configuration directory on macOS");
        }
        else
        {
            // On Linux or other systems: use sudo to create the directory
            let result = Command::new("sudo")
                .args(&["mkdir", "-p", parent_dir.to_str().unwrap()])
                .status()
                .expect("Failed to create the configuration directory");

            if !result.success()
            {
                eprintln!("Error creating the configuration directory.");
                std::process::exit(1);
            }
        }

        // Download the file
        let url = "https://raw.githubusercontent.com/Elxes04/Piko/main/config/default_config.toml";
        let response = get(url).expect("Unable to download the configuration file");
        let content = response.text().expect("Error reading the content");

        if cfg!(target_os = "macos")
        {
            // Directly write the file on macOS
            fs::write(&config_path, content).expect("Unable to write the configuration file");
        }
        else
        {
            // Use sudo and tee to write with root permissions on other systems
            let mut child = Command::new("sudo")
                .arg("tee")
                .arg(config_path.to_str().unwrap())
                .stdin(std::process::Stdio::piped())
                .spawn()
                .expect("Unable to start sudo tee");

            child.stdin
                .as_mut()
                .expect("Unable to open stdin")
                .write_all(content.as_bytes())
                .expect("Unable to write the file");

            let output = child.wait().expect("Error during file writing");
            if !output.success()
            {
                eprintln!("Error during the configuration file writing");
                std::process::exit(1);
            }
        }

        println!("Configuration file downloaded to {}", config_path.display());
    }
}

fn load_config(config_path: Option<PathBuf>) -> Value
{
    let path = config_path.unwrap_or_else(get_default_config_path);

    let contents = fs::read_to_string(&path)
        .unwrap_or_else(|_| panic!("Unable to read the configuration file from {:?}", path));

    contents.parse().expect("Invalid TOML configuration")
}

fn main()
{
    let cli = Cli::parse();

    if cli.config.is_none()
    {
        ensure_default_config_exists();
    }

    let config_value = load_config(cli.config);

    let config = config::Config::from_value(&config_value)
        .expect("Unable to deserialize the configuration file");

    let system_info = system_info::get_system_info();
    output::display_output(&system_info, &config_value);
}
