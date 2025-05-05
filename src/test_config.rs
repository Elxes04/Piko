use std::fs;
use toml::Value;

fn main() {
    let config_path = "config/default_config.toml";
    let config_content = fs::read_to_string(config_path).expect("Failed to read config file");
    let config: Value = config_content.parse().expect("Failed to parse config file");
    println!("{:#?}", config);
}