// This file handles reading and parsing the configuration file.
// It exports a struct Config that holds the customizable layout settings for the output.

use serde::Deserialize;
use std::collections::HashMap;
use toml::Value;

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Config {
    pub layout: String,
    pub output: OutputConfig,
    pub colors: HashMap<String, String>,
    pub symbols: HashMap<String, String>,
    pub logo: LogoConfig,
    pub theme: ThemeConfig,
    pub display: DisplayConfig,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct OutputConfig {
    pub info_keys: Vec<String>,
    pub show_logo: Option<bool>,
    pub logo_position: Option<String>, // "left", "right", "top", "bottom"
    pub logo_size: Option<String>,     // "small", "medium", "large"
    pub logo_color: Option<String>,
    pub logo_style: Option<String>, // "ascii", "unicode", "minimal"
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct LogoConfig {
    pub enabled: Option<bool>,
    pub distro: Option<String>, // "auto", "arch", "ubuntu", etc.
    pub custom_ascii: Option<Vec<String>>,
    pub custom_colors: Option<Vec<String>>,
    pub padding: Option<usize>,
    pub alignment: Option<String>, // "left", "center", "right"
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct ThemeConfig {
    pub name: Option<String>, // "default", "dracula", "nord", "gruvbox", "custom"
    pub primary_color: Option<String>,
    pub secondary_color: Option<String>,
    pub accent_color: Option<String>,
    pub background_color: Option<String>,
    pub text_color: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct DisplayConfig {
    pub width: Option<usize>,
    pub height: Option<usize>,
    pub border: Option<bool>,
    pub border_style: Option<String>, // "single", "double", "rounded", "none"
    pub border_color: Option<String>,
    pub padding: Option<usize>,
    pub margin: Option<usize>,
    pub alignment: Option<String>, // "left", "center", "right"
    pub compact: Option<bool>,
    pub show_separators: Option<bool>,
    pub separator_style: Option<String>, // "dash", "equals", "dots", "none"
}

impl Config {
    pub fn from_value(value: &Value) -> Result<Self, Box<dyn std::error::Error>> {
        let config: Config = value.clone().try_into()?;
        Ok(config)
    }

    #[allow(dead_code)]
    pub fn get_theme_colors(&self) -> HashMap<String, String> {
        let mut colors = self.colors.clone();

        // Note: theme is now a field, not an Option
        if let Some(primary) = &self.theme.primary_color {
            colors.insert("primary".to_string(), primary.clone());
        }
        if let Some(secondary) = &self.theme.secondary_color {
            colors.insert("secondary".to_string(), secondary.clone());
        }
        if let Some(accent) = &self.theme.accent_color {
            colors.insert("accent".to_string(), accent.clone());
        }

        colors
    }

    #[allow(dead_code)]
    pub fn should_show_logo(&self) -> bool {
        self.output.show_logo.unwrap_or(true)
    }

    #[allow(dead_code)]
    pub fn get_logo_position(&self) -> &str {
        self.output.logo_position.as_deref().unwrap_or("left")
    }

    #[allow(dead_code)]
    pub fn get_logo_size(&self) -> &str {
        self.output.logo_size.as_deref().unwrap_or("medium")
    }

    #[allow(dead_code)]
    pub fn get_logo_style(&self) -> &str {
        self.output.logo_style.as_deref().unwrap_or("ascii")
    }

    #[allow(dead_code)]
    pub fn is_compact_mode(&self) -> bool {
        self.display.compact.unwrap_or(false)
    }

    #[allow(dead_code)]
    pub fn should_show_border(&self) -> bool {
        self.display.border.unwrap_or(false)
    }

    #[allow(dead_code)]
    pub fn should_show_separators(&self) -> bool {
        self.display.show_separators.unwrap_or(true)
    }
}

// Default implementations
impl Default for LogoConfig {
    fn default() -> Self {
        LogoConfig {
            enabled: Some(true),
            distro: Some("auto".to_string()),
            custom_ascii: None,
            custom_colors: None,
            padding: Some(2),
            alignment: Some("left".to_string()),
        }
    }
}

impl Default for ThemeConfig {
    fn default() -> Self {
        ThemeConfig {
            name: Some("default".to_string()),
            primary_color: Some("#FF79C6".to_string()),
            secondary_color: Some("#50FA7B".to_string()),
            accent_color: Some("#BD93F9".to_string()),
            background_color: None,
            text_color: Some("#F8F8F2".to_string()),
        }
    }
}

impl Default for DisplayConfig {
    fn default() -> Self {
        DisplayConfig {
            width: None,
            height: None,
            border: Some(false),
            border_style: Some("single".to_string()),
            border_color: Some("#6272A4".to_string()),
            padding: Some(1),
            margin: Some(0),
            alignment: Some("left".to_string()),
            compact: Some(false),
            show_separators: Some(true),
            separator_style: Some("dash".to_string()),
        }
    }
}
