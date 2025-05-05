use std::collections::HashMap;
use toml::Value;
use colored::*;

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
            let default_color = "white".to_string();
            let color = colors.get(&key).unwrap_or(&default_color);
            let symbol_key = key.replace(' ', "_");
            let symbol = symbols.get(&symbol_key).map(String::as_str).unwrap_or("");
            println!("{}", format!("{} {}: {}", symbol, key, value).color(color.as_str()));
        }
    }
}
