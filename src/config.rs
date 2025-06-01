// This file handles reading and parsing the configuration file.
// It exports a struct Config that holds the customizable layout settings for the output.

use serde::Deserialize;
use std::collections::HashMap;
use toml::Value;

#[derive(Debug, Deserialize)]
pub struct Config
{
    pub layout: String,
    pub output: OutputConfig,
    pub colors: HashMap<String, String>,
}

#[derive(Debug, Deserialize)]
pub struct OutputConfig
{
    pub info_keys: Vec<String>,
}

impl Config
{
    pub fn from_value(value: &Value) -> Result<Self, Box<dyn std::error::Error>>
    {
        let config: Config = value.clone().try_into()?;
        Ok(config)
    }
}
