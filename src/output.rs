use std::collections::HashMap;
use toml::Value;

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
    // Display system information based on config
    if let Some(info_keys) = config.get("info_keys").and_then(|v| v.as_array()) {
        for key in info_keys {
            if let Some(key_str) = key.as_str() {
                if let Some(value) = system_info.get(key_str) {
                    println!("{}: {}", key_str, value);
                }
            }
        }
    } else {
        // Default behavior: display all info
        for (key, value) in system_info {
            println!("{}: {}", key, value);
        }
    }
}