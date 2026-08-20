// src/config.rs
// Strongly typed configuration system for Piko with embedded defaults,
// theme palettes, and standard XDG path resolution.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

pub const EMBEDDED_DEFAULT_CONFIG: &str = include_str!("../config/default_config.toml");

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(default = "default_layout")]
    pub layout: String,

    #[serde(default)]
    pub output: OutputConfig,

    #[serde(default)]
    pub logo: LogoConfig,

    #[serde(default)]
    pub theme: ThemeConfig,

    #[serde(default)]
    pub display: DisplayConfig,

    #[serde(default)]
    pub colors: HashMap<String, String>,

    #[serde(default)]
    pub symbols: HashMap<String, String>,
}

fn default_layout() -> String {
    "default".to_string()
}

impl Default for Config {
    fn default() -> Self {
        toml::from_str(EMBEDDED_DEFAULT_CONFIG).unwrap_or_else(|_| Config {
            layout: "default".to_string(),
            output: OutputConfig::default(),
            logo: LogoConfig::default(),
            theme: ThemeConfig::default(),
            display: DisplayConfig::default(),
            colors: HashMap::new(),
            symbols: HashMap::new(),
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutputConfig {
    #[serde(default = "default_info_keys")]
    pub info_keys: Vec<String>,

    #[serde(default = "default_true")]
    pub show_logo: bool,

    #[serde(default = "default_logo_position")]
    pub logo_position: String,

    #[serde(default = "default_logo_size")]
    pub logo_size: String,

    #[serde(default = "default_logo_style")]
    pub logo_style: String,

    #[serde(default = "default_true")]
    pub show_title: bool,

    #[serde(default = "default_true")]
    pub show_title_separator: bool,

    #[serde(default = "default_true")]
    pub show_color_blocks: bool,

    #[serde(default = "default_color_blocks_style")]
    pub color_blocks_style: String,
}

fn default_true() -> bool {
    true
}

fn default_logo_position() -> String {
    "left".to_string()
}

fn default_logo_size() -> String {
    "medium".to_string()
}

fn default_logo_style() -> String {
    "ascii".to_string()
}

fn default_color_blocks_style() -> String {
    "circle".to_string()
}

fn default_info_keys() -> Vec<String> {
    vec![
        "OS".to_string(),
        "Host".to_string(),
        "Kernel".to_string(),
        "Uptime".to_string(),
        "Packages".to_string(),
        "Shell".to_string(),
        "DE".to_string(),
        "WM".to_string(),
        "Terminal".to_string(),
        "CPU".to_string(),
        "GPU".to_string(),
        "Memory".to_string(),
        "Disk".to_string(),
        "Battery".to_string(),
    ]
}

impl Default for OutputConfig {
    fn default() -> Self {
        OutputConfig {
            info_keys: default_info_keys(),
            show_logo: true,
            logo_position: "left".to_string(),
            logo_size: "medium".to_string(),
            logo_style: "ascii".to_string(),
            show_title: true,
            show_title_separator: true,
            show_color_blocks: true,
            color_blocks_style: "circle".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogoConfig {
    #[serde(default = "default_true")]
    pub enabled: bool,

    #[serde(default = "default_distro")]
    pub distro: String,

    #[serde(default)]
    pub custom_ascii: Option<Vec<String>>,

    #[serde(default)]
    pub custom_colors: Option<Vec<String>>,

    #[serde(default = "default_logo_padding")]
    pub padding: usize,

    #[serde(default = "default_alignment")]
    pub alignment: String,
}

fn default_distro() -> String {
    "auto".to_string()
}

fn default_logo_padding() -> usize {
    2
}

fn default_alignment() -> String {
    "left".to_string()
}

impl Default for LogoConfig {
    fn default() -> Self {
        LogoConfig {
            enabled: true,
            distro: "auto".to_string(),
            custom_ascii: None,
            custom_colors: None,
            padding: 2,
            alignment: "left".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeConfig {
    #[serde(default = "default_theme_name")]
    pub name: String,

    pub primary_color: Option<String>,
    pub secondary_color: Option<String>,
    pub accent_color: Option<String>,
    pub background_color: Option<String>,
    pub text_color: Option<String>,
}

fn default_theme_name() -> String {
    "catppuccin".to_string()
}

impl Default for ThemeConfig {
    fn default() -> Self {
        ThemeConfig {
            name: "catppuccin".to_string(),
            primary_color: Some("#CBA6F7".to_string()),
            secondary_color: Some("#89B4FA".to_string()),
            accent_color: Some("#A6E3A1".to_string()),
            background_color: None,
            text_color: Some("#CDD6F4".to_string()),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisplayConfig {
    pub width: Option<usize>,
    pub height: Option<usize>,

    #[serde(default)]
    pub border: bool,

    #[serde(default = "default_border_style")]
    pub border_style: String,

    #[serde(default = "default_border_color")]
    pub border_color: String,

    #[serde(default = "default_padding")]
    pub padding: usize,

    #[serde(default)]
    pub margin: usize,

    #[serde(default = "default_alignment")]
    pub alignment: String,

    #[serde(default)]
    pub compact: bool,

    #[serde(default = "default_true")]
    pub show_separators: bool,

    #[serde(default = "default_separator_symbol")]
    pub separator_symbol: String,
}

fn default_border_style() -> String {
    "rounded".to_string()
}

fn default_border_color() -> String {
    "#6C7086".to_string()
}

fn default_padding() -> usize {
    1
}

fn default_separator_symbol() -> String {
    ":".to_string()
}

impl Default for DisplayConfig {
    fn default() -> Self {
        DisplayConfig {
            width: None,
            height: None,
            border: false,
            border_style: "rounded".to_string(),
            border_color: "#6C7086".to_string(),
            padding: 1,
            margin: 0,
            alignment: "left".to_string(),
            compact: false,
            show_separators: true,
            separator_symbol: ":".to_string(),
        }
    }
}

#[allow(dead_code)]
pub struct ThemePalette {
    pub title: &'static str,
    pub key: &'static str,
    pub value: &'static str,
    pub accent: &'static str,
    pub border: &'static str,
    pub colors: [&'static str; 8],
}

impl Config {
    pub fn get_palette(theme_name: &str) -> ThemePalette {
        match theme_name.to_lowercase().as_str() {
            "dracula" => ThemePalette {
                title: "#FF79C6",
                key: "#BD93F9",
                value: "#F8F8F2",
                accent: "#50FA7B",
                border: "#6272A4",
                colors: [
                    "#21222C", "#FF5555", "#50FA7B", "#F1FA8C", "#BD93F9", "#FF79C6", "#8BE9FD",
                    "#F8F8F2",
                ],
            },
            "catppuccin" | "catppuccin-mocha" | "mocha" => ThemePalette {
                title: "#CBA6F7",
                key: "#89B4FA",
                value: "#CDD6F4",
                accent: "#A6E3A1",
                border: "#6C7086",
                colors: [
                    "#45475A", "#F38BA8", "#A6E3A1", "#F9E2AF", "#89B4FA", "#CBA6F7", "#94E2D5",
                    "#BAC2DE",
                ],
            },
            "catppuccin-latte" | "latte" => ThemePalette {
                title: "#8839EF",
                key: "#1E66F5",
                value: "#4C4F69",
                accent: "#40A02B",
                border: "#9CA0B0",
                colors: [
                    "#5C5F77", "#D20F39", "#40A02B", "#DF8E1D", "#1E66F5", "#8839EF", "#179299",
                    "#ACB0BE",
                ],
            },
            "nord" => ThemePalette {
                title: "#88C0D0",
                key: "#81A1C1",
                value: "#ECEFF4",
                accent: "#A3BE8C",
                border: "#4C566A",
                colors: [
                    "#3B4252", "#BF616A", "#A3BE8C", "#EBCB8B", "#81A1C1", "#B48EAD", "#88C0D0",
                    "#E5E9F0",
                ],
            },
            "gruvbox" | "gruvbox-dark" => ThemePalette {
                title: "#FABD2F",
                key: "#FE8019",
                value: "#EBDBB2",
                accent: "#B8BB26",
                border: "#665C54",
                colors: [
                    "#282828", "#CC241D", "#98971A", "#D79921", "#458588", "#B16286", "#689D6A",
                    "#A89984",
                ],
            },
            "tokyo-night" | "tokyonight" => ThemePalette {
                title: "#7AA2F7",
                key: "#BB9AF7",
                value: "#C0CAF5",
                accent: "#9ECE6A",
                border: "#565F89",
                colors: [
                    "#1A1B26", "#F7768E", "#9ECE6A", "#E0AF68", "#7AA2F7", "#BB9AF7", "#7DCFFF",
                    "#A9B1D6",
                ],
            },
            "one-dark" | "onedark" => ThemePalette {
                title: "#61AFEF",
                key: "#98C379",
                value: "#ABB2BF",
                accent: "#E5C07B",
                border: "#5C6370",
                colors: [
                    "#282C34", "#E06C75", "#98C379", "#E5C07B", "#61AFEF", "#C678DD", "#56B6C2",
                    "#ABB2BF",
                ],
            },
            "solarized" | "solarized-dark" => ThemePalette {
                title: "#268BD2",
                key: "#2AA198",
                value: "#93A1A1",
                accent: "#859900",
                border: "#586E75",
                colors: [
                    "#073642", "#DC322F", "#859900", "#B58900", "#268BD2", "#D33682", "#2AA198",
                    "#EEE8D5",
                ],
            },
            "rose-pine" | "rosepine" => ThemePalette {
                title: "#EBBCBA",
                key: "#C4A7E7",
                value: "#E0DEF4",
                accent: "#9CCFD8",
                border: "#6E6A86",
                colors: [
                    "#191724", "#EB6F92", "#9CCFD8", "#F6C177", "#31748F", "#C4A7E7", "#EBBCBA",
                    "#E0DEF4",
                ],
            },
            "synthwave" => ThemePalette {
                title: "#FF7EDB",
                key: "#36F9F6",
                value: "#FFFFFF",
                accent: "#72F1B8",
                border: "#614D85",
                colors: [
                    "#262335", "#FE4450", "#72F1B8", "#FFE64D", "#36F9F6", "#FF7EDB", "#B072D1",
                    "#FFFFFF",
                ],
            },
            _ => ThemePalette {
                title: "#CBA6F7",
                key: "#89B4FA",
                value: "#CDD6F4",
                accent: "#A6E3A1",
                border: "#6C7086",
                colors: [
                    "#45475A", "#F38BA8", "#A6E3A1", "#F9E2AF", "#89B4FA", "#CBA6F7", "#94E2D5",
                    "#BAC2DE",
                ],
            },
        }
    }

    pub fn list_available_themes() -> Vec<&'static str> {
        vec![
            "catppuccin",
            "catppuccin-latte",
            "dracula",
            "nord",
            "gruvbox",
            "tokyo-night",
            "one-dark",
            "solarized",
            "rose-pine",
            "synthwave",
        ]
    }
}

#[allow(dead_code)]
pub fn is_termux() -> bool {
    std::env::var("TERMUX_VERSION").is_ok()
        || std::env::var("PREFIX")
            .map(|p| p.contains("/data/data/com.termux"))
            .unwrap_or(false)
}

pub fn find_config_file(custom_path: Option<&Path>) -> Option<PathBuf> {
    if let Some(path) = custom_path {
        if path.exists() {
            return Some(path.to_path_buf());
        }
    }

    if let Ok(env_path) = std::env::var("PIKO_CONFIG") {
        let path = PathBuf::from(env_path);
        if path.exists() {
            return Some(path);
        }
    }

    // Check XDG user config: ~/.config/piko/config.toml
    if let Some(config_dir) = dirs::config_dir() {
        let piko_toml = config_dir.join("piko").join("config.toml");
        if piko_toml.exists() {
            return Some(piko_toml);
        }
        let default_toml = config_dir.join("piko").join("default_config.toml");
        if default_toml.exists() {
            return Some(default_toml);
        }
    }

    // Check system config: /etc/piko/config.toml or /etc/piko/default_config.toml
    let system_paths = [
        PathBuf::from("/etc/piko/config.toml"),
        PathBuf::from("/etc/piko/default_config.toml"),
    ];
    for path in &system_paths {
        if path.exists() {
            return Some(path.clone());
        }
    }

    None
}

pub fn load_config_or_default(custom_path: Option<&Path>) -> Config {
    if let Some(path) = find_config_file(custom_path) {
        if let Ok(contents) = fs::read_to_string(&path) {
            if let Ok(config) = toml::from_str::<Config>(&contents) {
                return config;
            }
        }
    }

    // Fall back to built-in embedded default config
    Config::default()
}

pub fn export_default_config_to(path: &Path) -> std::io::Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, EMBEDDED_DEFAULT_CONFIG)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_embedded_default_config_valid() {
        let config: Config =
            toml::from_str(EMBEDDED_DEFAULT_CONFIG).expect("Embedded default config must parse");
        assert_eq!(config.layout, "default");
        assert!(config.output.show_logo);
        assert!(!config.output.info_keys.is_empty());
    }

    #[test]
    fn test_theme_palettes_available() {
        let themes = Config::list_available_themes();
        assert!(themes.contains(&"catppuccin"));
        assert!(themes.contains(&"dracula"));
        assert!(themes.contains(&"nord"));
        assert!(themes.contains(&"gruvbox"));

        let dracula = Config::get_palette("dracula");
        assert_eq!(dracula.title, "#FF79C6");
        assert_eq!(dracula.key, "#BD93F9");
    }
}
