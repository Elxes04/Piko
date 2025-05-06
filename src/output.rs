use std::collections::HashMap;
use toml::Value;
use colored::*;

// Helper function to convert HEX color to RGB
fn hex_to_rgb(hex: &str) -> Option<(u8, u8, u8)> {
    if hex.starts_with('#') && hex.len() == 7 {
        let r = u8::from_str_radix(&hex[1..3], 16).ok()?;
        let g = u8::from_str_radix(&hex[3..5], 16).ok()?;
        let b = u8::from_str_radix(&hex[5..7], 16).ok()?;
        Some((r, g, b))
    } else {
        None
    }
}

pub struct OutputConfig {
    pub layout: String,
}

impl OutputConfig {
    pub fn new(layout: String) -> Self {
        OutputConfig { layout }
    }
}

pub fn display_output(system_info: &HashMap<String, String>, config: &Value) {
    let info_keys = config
        .get("output")
        .and_then(|output| output.get("info_keys"))
        .and_then(|keys| keys.as_array())
        .map(|keys| {
            keys.iter()
                .filter_map(|key| key.as_str().map(String::from))
                .collect::<Vec<String>>()
        })
        .unwrap_or_else(|| system_info.keys().cloned().collect());

    let dummy_map = toml::map::Map::new();
    let show_flags = config.get("output")
        .and_then(|output| output.as_table())
        .unwrap_or(&dummy_map);

    let colors = config
        .get("colors")
        .and_then(|c| c.as_table())
        .map(|t| {
            t.iter()
                .filter_map(|(k, v)| v.as_str().map(|val| (k.clone(), val.to_string())))
                .collect::<HashMap<String, String>>()
        })
        .unwrap_or_default();

    let symbols = config
        .get("symbols")
        .and_then(|s| s.as_table())
        .map(|t| {
            t.iter()
                .filter_map(|(k, v)| v.as_str().map(|val| (k.clone(), val.to_string())))
                .collect::<HashMap<String, String>>()
        })
        .unwrap_or_default();

    let default_color = "#FFFFFF".to_string();

    for key in info_keys {
        // Build the flag key (e.g., "CPU Model" => "show_cpu_model")
        let flag_key = format!(
            "show_{}",
            key.to_lowercase()
                .chars()
                .map(|c| if c.is_alphanumeric() { c } else { '_' })
                .collect::<String>()
        );

        let show_entry = show_flags
            .get(&flag_key)
            .and_then(|v| v.as_bool())
            .unwrap_or(true); // Default to true if the flag is not present

        if show_entry {
            if let Some(value) = system_info.get(&key) {
                let color_hex = colors.get(&key).unwrap_or(&default_color);
                let rgb_color = hex_to_rgb(color_hex).unwrap_or((255, 255, 255));
                let symbol = symbols.get(&key).map(String::as_str).unwrap_or("");

                println!(
                    "{} {}: {}",
                    symbol,
                    key.truecolor(rgb_color.0, rgb_color.1, rgb_color.2),
                    value.truecolor(rgb_color.0, rgb_color.1, rgb_color.2)
                );
            }
        }
    }
}
