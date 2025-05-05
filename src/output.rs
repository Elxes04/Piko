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

pub fn format_output(system_info: &str, config: &OutputConfig) -> String {
    match config.layout.as_str() {
        "simple" => format!("System Information:\n{}", system_info),
        "detailed" => format!("=== System Information ===\n{}", system_info),
        _ => format!("Unsupported layout. Defaulting to simple:\n{}", system_info),
    }
}

pub fn display_output(system_info: &HashMap<String, String>, config: &Value) {
    // Get the ordered keys to display
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

    // Get colors
    let colors = config
        .get("colors")
        .and_then(|c| c.as_table())
        .map(|t| {
            t.iter()
                .filter_map(|(k, v)| v.as_str().map(|val| (k.clone(), val.to_string())))
                .collect::<HashMap<String, String>>()
        })
        .unwrap_or_default();

    // Get symbols
    let symbols = config
        .get("symbols")
        .and_then(|s| s.as_table())
        .map(|t| {
            t.iter()
                .filter_map(|(k, v)| v.as_str().map(|val| (k.clone(), val.to_string())))
                .collect::<HashMap<String, String>>()
        })
        .unwrap_or_default();

    for key in info_keys {
        if let Some(value) = system_info.get(&key) {
            let default_color = "#FFFFFF".to_string(); // Default to white
            let color_hex = colors.get(&key).unwrap_or(&default_color);

            // Convert HEX to RGB
            let rgb_color = hex_to_rgb(color_hex).unwrap_or((255, 255, 255)); // Default to white if invalid

            let symbol = symbols.get(&key).map(String::as_str).unwrap_or("");

            // Use truecolor for RGB
            println!(
                "{} {}: {}",
                symbol,
                key.truecolor(rgb_color.0, rgb_color.1, rgb_color.2),
                value.truecolor(rgb_color.0, rgb_color.1, rgb_color.2)
            );
        }
    }
}
