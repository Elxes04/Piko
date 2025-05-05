// src/main.rs

mod config;
mod system_info;
mod output;

fn main() {
    let config_path = "config/default_config.toml";
    let config = config::load_config(config_path).expect("Failed to load configuration");

    let system_info = system_info::get_system_info();

    output::display_output(&system_info, &config);
}