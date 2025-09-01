use crate::distro_logo::{DistroLogo, LogoManager};
use colored::*;
use std::collections::HashMap;
use toml::Value;

#[derive(Debug)]
struct LogoDisplayOptions {
    position: String,
    size: String,
    style: String,
    show_border: bool,
    compact: bool,
}

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

#[allow(dead_code)]
pub struct OutputConfig {
    pub layout: String,
}

#[allow(dead_code)]
impl OutputConfig {
    pub fn new(layout: String) -> Self {
        OutputConfig { layout }
    }
}

pub fn display_output(system_info: &HashMap<String, String>, config: &Value) {
    let logo_manager = LogoManager::new();
    let show_logo = config
        .get("output")
        .and_then(|output| output.get("show_logo"))
        .and_then(|v| v.as_bool())
        .unwrap_or(true);

    let logo_position = config
        .get("output")
        .and_then(|output| output.get("logo_position"))
        .and_then(|v| v.as_str())
        .unwrap_or("left");

    let logo_size = config
        .get("output")
        .and_then(|output| output.get("logo_size"))
        .and_then(|v| v.as_str())
        .unwrap_or("medium");

    let logo_style = config
        .get("output")
        .and_then(|output| output.get("logo_style"))
        .and_then(|v| v.as_str())
        .unwrap_or("ascii");

    let compact_mode = config
        .get("display")
        .and_then(|display| display.get("compact"))
        .and_then(|v| v.as_bool())
        .unwrap_or(false);

    let show_border = config
        .get("display")
        .and_then(|display| display.get("border"))
        .and_then(|v| v.as_bool())
        .unwrap_or(false);

    let show_separators = config
        .get("display")
        .and_then(|display| display.get("show_separators"))
        .and_then(|v| v.as_bool())
        .unwrap_or(true);

    let separator_style = config
        .get("display")
        .and_then(|display| display.get("separator_style"))
        .and_then(|v| v.as_str())
        .unwrap_or("dash");

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
    let show_flags = config
        .get("output")
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

    // Get logo if enabled
    let logo = if show_logo {
        Some(logo_manager.get_detected_logo())
    } else {
        None
    };

    // Prepare system info lines
    let mut info_lines = Vec::new();
    for key in info_keys {
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
            .unwrap_or(true);

        if show_entry {
            if let Some(value) = system_info.get(&key) {
                let color_hex = colors.get(&key).unwrap_or(&default_color);
                let rgb_color = hex_to_rgb(color_hex).unwrap_or((255, 255, 255));
                let symbol = symbols.get(&key).map(String::as_str).unwrap_or("");

                // Handle multi-line values (like disk information)
                let lines: Vec<&str> = value.lines().collect();
                if lines.len() == 1 {
                    // Single line - format normally
                    let line = format!(
                        "{} {}: {}",
                        symbol,
                        key.truecolor(rgb_color.0, rgb_color.1, rgb_color.2),
                        lines[0].truecolor(rgb_color.0, rgb_color.1, rgb_color.2)
                    );
                    info_lines.push(line);
                } else {
                    // Multiple lines - first line with label, others indented
                    for (i, line_content) in lines.iter().enumerate() {
                        if i == 0 {
                            // First line with label
                            let line = format!(
                                "{} {}: {}",
                                symbol,
                                key.truecolor(rgb_color.0, rgb_color.1, rgb_color.2),
                                line_content.truecolor(rgb_color.0, rgb_color.1, rgb_color.2)
                            );
                            info_lines.push(line);
                        } else {
                            // Additional lines indented
                            let line = format!(
                                "{}",
                                line_content.truecolor(rgb_color.0, rgb_color.1, rgb_color.2)
                            );
                            info_lines.push(line);
                        }
                    }
                }
            }
        }
    }

    // Display with logo
    if let Some(distro_logo) = logo {
        let options = LogoDisplayOptions {
            position: logo_position.to_string(),
            size: logo_size.to_string(),
            style: logo_style.to_string(),
            show_border,
            compact: compact_mode,
        };
        display_with_logo(distro_logo, &info_lines, &options);
    } else {
        display_simple(
            &info_lines,
            show_border,
            show_separators,
            separator_style,
            compact_mode,
        );
    }
}

fn display_with_logo(logo: &DistroLogo, info_lines: &[String], options: &LogoDisplayOptions) {
    let logo_art = get_logo_art(logo, &options.size, &options.style);
    let logo_height = logo_art.len();
    let info_height = info_lines.len();

    // Find the maximum width of the logo for proper alignment
    let logo_width = logo_art.iter().map(|line| line.len()).max().unwrap_or(0);
    let padding = if options.compact { 2 } else { 4 };

    if options.show_border {
        print_border_start();
    }

    // Display logo and info side by side with proper alignment
    for i in 0..std::cmp::max(logo_height, info_height) {
        let logo_line = if i < logo_height {
            Some(&logo_art[i])
        } else {
            None
        };

        let info_line = if i < info_height {
            Some(&info_lines[i])
        } else {
            None
        };

        match options.position.as_str() {
            "left" => {
                if let Some(logo_line) = logo_line {
                    // Print logo line with proper padding to align info
                    let current_logo_width = logo_line.len();
                    print!("{}", logo_line);
                    if let Some(info_line) = info_line {
                        println!(
                            "{}{}",
                            " ".repeat(logo_width - current_logo_width + padding),
                            info_line
                        );
                    } else {
                        println!();
                    }
                } else if let Some(info_line) = info_line {
                    // When logo is shorter, align info with the logo
                    println!("{}{}", " ".repeat(logo_width + padding), info_line);
                }
            }
            "right" => {
                if let Some(info_line) = info_line {
                    print!("{}", info_line);
                    if let Some(logo_line) = logo_line {
                        println!("{}{}", " ".repeat(padding), logo_line);
                    } else {
                        println!();
                    }
                } else if let Some(logo_line) = logo_line {
                    let info_width = info_lines.iter().map(|line| line.len()).max().unwrap_or(0);
                    println!("{}{}", " ".repeat(info_width + padding), logo_line);
                }
            }
            _ => {
                // Default to left
                if let Some(logo_line) = logo_line {
                    // Print logo line with proper padding to align info
                    let current_logo_width = logo_line.len();
                    print!("{}", logo_line);
                    if let Some(info_line) = info_line {
                        println!(
                            "{}{}",
                            " ".repeat(logo_width - current_logo_width + padding),
                            info_line
                        );
                    } else {
                        println!();
                    }
                } else if let Some(info_line) = info_line {
                    // When logo is shorter, align info with the logo
                    println!("{}{}", " ".repeat(logo_width + padding), info_line);
                }
            }
        }
    }

    if options.show_border {
        print_border_end();
    }
}

fn display_simple(
    info_lines: &[String],
    show_border: bool,
    _show_separators: bool,
    _separator_style: &str,
    _compact: bool,
) {
    if show_border {
        print_border_start();
    }

    for line in info_lines.iter() {
        println!("{}", line);
    }

    if show_border {
        print_border_end();
    }
}

fn get_logo_art(logo: &DistroLogo, size: &str, style: &str) -> Vec<String> {
    let mut art = logo.ascii_art.clone();

    // Apply size modifications
    match size {
        "small" => {
            // Reduce size by taking every other line
            art = art.into_iter().step_by(2).collect();
        }
        "large" => {
            // Keep original size
        }
        _ => {
            // Medium size - keep original
        }
    }

    // Apply style modifications
    match style {
        "minimal" => {
            // Take only first few lines
            art.truncate(3);
        }
        "unicode" => {
            // Keep original (could be enhanced with Unicode characters)
        }
        _ => {
            // ASCII style - keep original
        }
    }

    art
}

fn print_border_start() {
    println!("┌─────────────────────────────────────────────────────────────┐");
}

fn print_border_end() {
    println!("└─────────────────────────────────────────────────────────────┘");
}

// New function to display logo only
pub fn display_logo_only(config: &Value) {
    let logo_manager = LogoManager::new();
    let logo = logo_manager.get_detected_logo();

    let size = config
        .get("output")
        .and_then(|output| output.get("logo_size"))
        .and_then(|v| v.as_str())
        .unwrap_or("medium");

    let style = config
        .get("output")
        .and_then(|output| output.get("logo_style"))
        .and_then(|v| v.as_str())
        .unwrap_or("ascii");

    let logo_art = get_logo_art(logo, size, style);

    for line in logo_art {
        println!("{}", line);
    }
}

// New function to list available logos
pub fn list_available_logos() {
    let logo_manager = LogoManager::new();
    let logos = logo_manager.list_available_logos();

    println!("Available logos:");
    for logo in logos {
        println!("  - {}", logo);
    }
}

// New function to display system info in JSON format
pub fn display_json(system_info: &HashMap<String, String>) {
    use serde_json;
    let json = serde_json::to_string_pretty(system_info).unwrap();
    println!("{}", json);
}

// New function to display system info in YAML format
pub fn display_yaml(system_info: &HashMap<String, String>) {
    use serde_yaml;
    let yaml = serde_yaml::to_string(system_info).unwrap();
    println!("{}", yaml);
}
