// This file handles reading and parsing the configuration file.
// It exports a struct Config that holds the customizable layout settings for the output.

use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use toml::Value;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub layout: String,
    pub output: OutputConfig,
    pub colors: HashMap<String, String>,
}

#[derive(Debug, Deserialize)]
pub struct OutputConfig {
    pub info_keys: Vec<String>,
}

impl Config {
    pub fn new(config_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let config_content = fs::read_to_string(config_path)?;
        let config: Config = toml::de::from_str(&config_content)?;
        Ok(config)
    }
}

pub fn load_config(config_path: &str) -> Result<Value, String> {
    let config_content = fs::read_to_string(config_path)
        .map_err(|e| format!("Failed to read config file: {}: {}", config_path, e))?;
    config_content
        .parse::<Value>()
        .map_err(|e| format!("Failed to parse config file: {}", e))
}