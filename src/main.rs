mod config;
mod distro_logo;
mod output;
mod system_info;

use clap::Parser;
use std::fs;
use std::path::PathBuf;

/// Piko - A minimal, fast, and highly customizable system information tool written in Rust
#[derive(Parser, Debug)]
#[command(name = "piko", author = "Elxes04", version, about, long_about = None)]
struct Cli {
    /// Path to a custom config file
    #[arg(short, long)]
    config: Option<PathBuf>,

    /// Select color theme (e.g. catppuccin, dracula, nord, gruvbox, tokyo-night, one-dark, rose-pine, synthwave)
    #[arg(short, long)]
    theme: Option<String>,

    /// Distribution logo override (e.g. nixos, arch, ubuntu, debian, fedora, manjaro, mint, pop, alpine, void, macos)
    #[arg(long, aliases = ["ascii-distro", "distro"])]
    ascii_distro: Option<String>,

    /// Show only the distribution logo
    #[arg(long)]
    logo_only: bool,

    /// List all available distribution logos
    #[arg(long, aliases = ["list-distros"])]
    list_logos: bool,

    /// List all available color themes
    #[arg(long)]
    list_themes: bool,

    /// Output format: normal, json, yaml
    #[arg(short, long, default_value = "normal")]
    format: String,

    /// Show logo position: left, right, top, bottom
    #[arg(long)]
    logo_position: Option<String>,

    /// Logo size: small, medium, large
    #[arg(long)]
    logo_size: Option<String>,

    /// Logo style: ascii, unicode, minimal
    #[arg(long)]
    logo_style: Option<String>,

    /// Show border around output
    #[arg(short, long)]
    border: bool,

    /// Border style: single, double, rounded, bold
    #[arg(long)]
    border_style: Option<String>,

    /// Hide separators between info lines
    #[arg(long)]
    no_separators: bool,

    /// Toggle display of color palette blocks (true/false)
    #[arg(long)]
    color_blocks: Option<bool>,

    /// Generate default config file in ~/.config/piko/config.toml
    #[arg(long)]
    gen_config: bool,

    /// Print the default configuration to standard output
    #[arg(long)]
    print_config: bool,

    /// Export current configuration to file
    #[arg(long)]
    export_config: Option<PathBuf>,

    /// Import configuration from file
    #[arg(long)]
    import_config: Option<PathBuf>,
}

fn main() {
    let cli = Cli::parse();

    // 1. Handle informational list commands first
    if cli.list_logos {
        output::list_available_logos();
        return;
    }

    if cli.list_themes {
        output::list_available_themes();
        return;
    }

    if cli.print_config {
        print!("{}", config::EMBEDDED_DEFAULT_CONFIG);
        return;
    }

    if cli.gen_config {
        if let Some(config_dir) = dirs::config_dir() {
            let target_path = config_dir.join("piko").join("config.toml");
            match config::export_default_config_to(&target_path) {
                Ok(_) => {
                    println!(
                        "Successfully created configuration file at: {}",
                        target_path.display()
                    );
                }
                Err(e) => {
                    eprintln!("Error creating configuration file: {}", e);
                    std::process::exit(1);
                }
            }
        } else {
            eprintln!("Could not determine user config directory.");
            std::process::exit(1);
        }
        return;
    }

    // 2. Load configuration (from CLI path, env var, standard paths, or embedded defaults)
    let config_path_to_load = cli.import_config.as_deref().or(cli.config.as_deref());
    let mut config = config::load_config_or_default(config_path_to_load);

    // 3. Apply CLI overrides
    if let Some(theme_name) = cli.theme {
        config.theme.name = theme_name;
    }

    if let Some(distro_name) = cli.ascii_distro {
        config.logo.distro = distro_name;
    }

    if let Some(pos) = cli.logo_position {
        config.output.logo_position = pos;
    }

    if let Some(size) = cli.logo_size {
        config.output.logo_size = size;
    }

    if let Some(style) = cli.logo_style {
        config.output.logo_style = style;
    }

    if cli.border {
        config.display.border = true;
    }

    if let Some(bstyle) = cli.border_style {
        config.display.border = true;
        config.display.border_style = bstyle;
    }

    if cli.no_separators {
        config.display.show_separators = false;
    }

    if let Some(cb) = cli.color_blocks {
        config.output.show_color_blocks = cb;
    }

    // 4. Handle export
    if let Some(ref export_path) = cli.export_config {
        match toml::to_string_pretty(&config) {
            Ok(toml_str) => {
                if let Err(e) = fs::write(export_path, toml_str) {
                    eprintln!("Error exporting configuration: {}", e);
                    std::process::exit(1);
                }
                println!("Configuration exported to: {}", export_path.display());
                return;
            }
            Err(e) => {
                eprintln!("Error serializing configuration: {}", e);
                std::process::exit(1);
            }
        }
    }

    // 5. Handle logo-only mode
    if cli.logo_only {
        output::display_logo_only(&config);
        return;
    }

    // 6. Get system information
    let system_info = system_info::get_system_info();

    // 7. Display output based on requested format
    match cli.format.to_lowercase().as_str() {
        "json" => output::display_json(&system_info),
        "yaml" | "yml" => output::display_yaml(&system_info),
        _ => output::display_output(&system_info, &config),
    }
}
