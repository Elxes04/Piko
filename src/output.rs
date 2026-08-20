// src/output.rs
// Rich output engine for Piko with ANSI-aware visible width calculation,
// multi-color logos, border framing, and Neofetch-style color blocks.

use crate::config::{Config, ThemePalette};
use crate::distro_logo::{DistroLogo, LogoManager};
use colored::*;
use std::collections::HashMap;
use unicode_width::UnicodeWidthStr;

// Strip ANSI escape sequences to compute true terminal column width
fn strip_ansi(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut in_escape = false;
    let mut chars = s.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '\x1b' {
            in_escape = true;
        } else if in_escape {
            if c == 'm' || c == 'K' || c == 'H' || c == 'J' {
                in_escape = false;
            }
        } else if c == '$' && chars.peek() == Some(&'{') {
            // Skip unexpanded ${c1}..${c9} placeholders if present
            let mut buf = String::new();
            let mut found_close = false;
            for _ in 0..5 {
                if let Some(&nc) = chars.peek() {
                    buf.push(nc);
                    chars.next();
                    if nc == '}' {
                        found_close = true;
                        break;
                    }
                }
            }
            if !found_close {
                out.push('$');
                out.push_str(&buf);
            }
        } else {
            out.push(c);
        }
    }
    out
}

fn visible_width(s: &str) -> usize {
    let clean = strip_ansi(s);
    UnicodeWidthStr::width(clean.as_str())
}

// Convert HEX color (#RRGGBB) to RGB tuple
fn hex_to_rgb(hex: &str) -> (u8, u8, u8) {
    let hex_clean = hex.trim_start_matches('#');
    if hex_clean.len() == 6 {
        let r = u8::from_str_radix(&hex_clean[0..2], 16).unwrap_or(200);
        let g = u8::from_str_radix(&hex_clean[2..4], 16).unwrap_or(200);
        let b = u8::from_str_radix(&hex_clean[4..6], 16).unwrap_or(200);
        (r, g, b)
    } else {
        (205, 214, 244)
    }
}

pub fn display_output(system_info: &HashMap<String, String>, config: &Config) {
    let logo_manager = LogoManager::new();
    let palette = Config::get_palette(&config.theme.name);

    // 1. Prepare Information Lines
    let mut info_lines: Vec<String> = Vec::new();

    // Title line (username@hostname)
    if config.output.show_title {
        if let Some(title) = system_info.get("Title") {
            let title_rgb = hex_to_rgb(palette.title);
            let colored_title = title
                .bold()
                .truecolor(title_rgb.0, title_rgb.1, title_rgb.2);
            info_lines.push(format!("{}", colored_title));

            if config.output.show_title_separator {
                let underline_len = title.chars().count();
                let underline = "-".repeat(underline_len);
                let sep_rgb = hex_to_rgb(palette.border);
                info_lines.push(format!(
                    "{}",
                    underline.truecolor(sep_rgb.0, sep_rgb.1, sep_rgb.2)
                ));
            }
        }
    }

    let default_sep = if config.display.show_separators {
        &config.display.separator_symbol
    } else {
        ""
    };

    // Information keys
    for key in &config.output.info_keys {
        if key == "Title" {
            continue;
        }

        if let Some(value) = system_info.get(key) {
            let key_hex = config
                .colors
                .get(key)
                .map(|s| s.as_str())
                .unwrap_or(palette.key);
            let key_rgb = hex_to_rgb(key_hex);

            let val_hex = palette.value;
            let val_rgb = hex_to_rgb(val_hex);

            let sym_hex = palette.accent;
            let sym_rgb = hex_to_rgb(sym_hex);

            let symbol = config.symbols.get(key).map(|s| s.as_str()).unwrap_or("");
            let sym_part = if !symbol.is_empty() {
                format!("{} ", symbol.truecolor(sym_rgb.0, sym_rgb.1, sym_rgb.2))
            } else {
                String::new()
            };

            let lines: Vec<&str> = value.lines().collect();
            for (idx, line_val) in lines.iter().enumerate() {
                if idx == 0 {
                    let formatted_key = key.bold().truecolor(key_rgb.0, key_rgb.1, key_rgb.2);
                    let sep_part = if !default_sep.is_empty() {
                        format!(
                            "{} ",
                            default_sep.truecolor(key_rgb.0, key_rgb.1, key_rgb.2)
                        )
                    } else {
                        " ".to_string()
                    };
                    let formatted_val = line_val.truecolor(val_rgb.0, val_rgb.1, val_rgb.2);

                    info_lines.push(format!(
                        "{}{}{}{}",
                        sym_part, formatted_key, sep_part, formatted_val
                    ));
                } else {
                    let formatted_val = line_val.truecolor(val_rgb.0, val_rgb.1, val_rgb.2);
                    info_lines.push(format!("    {}", formatted_val));
                }
            }
        }
    }

    // Color blocks / palette
    if config.output.show_color_blocks {
        info_lines.push(String::new()); // blank line
        let block_char = match config.output.color_blocks_style.as_str() {
            "block" => "███ ",
            _ => "● ",
        };

        // Standard 8 ANSI colors
        let mut palette_line = String::new();
        let standard_colors = [
            (0, 0, 0),       // Black
            (235, 77, 75),   // Red
            (106, 176, 76),  // Green
            (240, 147, 43),  // Yellow
            (72, 126, 176),  // Blue
            (190, 46, 221),  // Magenta
            (34, 166, 179),  // Cyan
            (223, 228, 234), // White
        ];
        for (r, g, b) in standard_colors {
            palette_line.push_str(&format!("{}", block_char.truecolor(r, g, b)));
        }
        info_lines.push(palette_line);

        // Bright 8 ANSI colors
        let mut bright_line = String::new();
        let bright_colors = [
            (83, 92, 104),   // Bright Black (Gray)
            (255, 121, 121), // Bright Red
            (186, 220, 88),  // Bright Green
            (249, 202, 36),  // Bright Yellow
            (104, 109, 224), // Bright Blue
            (224, 86, 253),  // Bright Magenta
            (126, 214, 223), // Bright Cyan
            (255, 255, 255), // Bright White
        ];
        for (r, g, b) in bright_colors {
            bright_line.push_str(&format!("{}", block_char.truecolor(r, g, b)));
        }
        info_lines.push(bright_line);
    }

    // 2. Fetch Logo
    let override_distro = if config.logo.distro != "auto" {
        Some(config.logo.distro.as_str())
    } else {
        None
    };

    let distro_logo = logo_manager.get_detected_logo(override_distro);
    let logo_art = get_colored_logo_art(distro_logo, &config.output.logo_size, &palette);

    // 3. Render Output according to position & borders
    let final_lines = if config.output.show_logo {
        match config.output.logo_position.as_str() {
            "top" => render_top_layout(&logo_art, &info_lines, config.logo.padding),
            "bottom" => render_bottom_layout(&logo_art, &info_lines, config.logo.padding),
            "right" => render_right_layout(&logo_art, &info_lines, config.logo.padding),
            _ => render_left_layout(&logo_art, &info_lines, config.logo.padding),
        }
    } else {
        info_lines
    };

    if config.display.border {
        render_with_border(&final_lines, &config.display.border_style, palette.border);
    } else {
        for line in final_lines {
            println!("{}", line);
        }
    }
}

fn get_colored_logo_art(logo: &DistroLogo, size: &str, palette: &ThemePalette) -> Vec<String> {
    let art_source = if size == "small" && !logo.small_art.is_empty() {
        &logo.small_art
    } else {
        &logo.ascii_art
    };

    let logo_colors: Vec<(u8, u8, u8)> = if !logo.colors.is_empty() {
        logo.colors.iter().map(|hex| hex_to_rgb(hex)).collect()
    } else {
        vec![hex_to_rgb(palette.title)]
    };

    let mut colored_lines = Vec::new();
    let mut current_color_idx: usize = 0;

    for line in art_source {
        if line.contains("${c") {
            let mut colored = line.clone();
            // Track the last seen color tag on this line
            let mut last_tag_info: Option<(usize, usize)> = None;

            for ci in 0..8 {
                let tag = format!("${{c{}}}", ci + 1);
                if let Some(pos) = line.rfind(&tag) {
                    let is_later = match last_tag_info {
                        None => true,
                        Some((_, p)) => pos > p,
                    };
                    if is_later {
                        last_tag_info = Some((ci, pos));
                    }
                }

                if colored.contains(&tag) {
                    let color = if ci < logo_colors.len() {
                        logo_colors[ci]
                    } else {
                        logo_colors[ci % logo_colors.len()]
                    };
                    let ansi_code = format!("\x1b[1;38;2;{};{};{}m", color.0, color.1, color.2);
                    colored = colored.replace(&tag, &ansi_code);
                }
            }

            if let Some((ci, _)) = last_tag_info {
                current_color_idx = ci % logo_colors.len();
            }

            colored.push_str("\x1b[0m");
            colored_lines.push(colored);
        } else {
            let (r, g, b) = logo_colors[current_color_idx % logo_colors.len()];
            colored_lines.push(format!("{}", line.bold().truecolor(r, g, b)));
        }
    }

    colored_lines
}

fn render_left_layout(logo_lines: &[String], info_lines: &[String], padding: usize) -> Vec<String> {
    let mut output = Vec::new();
    let max_logo_width = logo_lines
        .iter()
        .map(|l| visible_width(l))
        .max()
        .unwrap_or(0);
    let total_lines = std::cmp::max(logo_lines.len(), info_lines.len());
    let pad_space = " ".repeat(padding);

    for i in 0..total_lines {
        let mut line = String::new();

        if let Some(logo_line) = logo_lines.get(i) {
            let width = visible_width(logo_line);
            let fill = " ".repeat(max_logo_width.saturating_sub(width));
            line.push_str(logo_line);
            line.push_str(&fill);
        } else {
            line.push_str(&" ".repeat(max_logo_width));
        }

        line.push_str(&pad_space);

        if let Some(info_line) = info_lines.get(i) {
            line.push_str(info_line);
        }

        output.push(line);
    }

    output
}

fn render_right_layout(
    logo_lines: &[String],
    info_lines: &[String],
    padding: usize,
) -> Vec<String> {
    let mut output = Vec::new();
    let max_info_width = info_lines
        .iter()
        .map(|l| visible_width(l))
        .max()
        .unwrap_or(0);
    let total_lines = std::cmp::max(logo_lines.len(), info_lines.len());
    let pad_space = " ".repeat(padding);

    for i in 0..total_lines {
        let mut line = String::new();

        if let Some(info_line) = info_lines.get(i) {
            let width = visible_width(info_line);
            let fill = " ".repeat(max_info_width.saturating_sub(width));
            line.push_str(info_line);
            line.push_str(&fill);
        } else {
            line.push_str(&" ".repeat(max_info_width));
        }

        line.push_str(&pad_space);

        if let Some(logo_line) = logo_lines.get(i) {
            line.push_str(logo_line);
        }

        output.push(line);
    }

    output
}

fn render_top_layout(logo_lines: &[String], info_lines: &[String], _padding: usize) -> Vec<String> {
    let mut output = Vec::new();
    for l in logo_lines {
        output.push(l.clone());
    }
    output.push(String::new());
    for i in info_lines {
        output.push(i.clone());
    }
    output
}

fn render_bottom_layout(
    logo_lines: &[String],
    info_lines: &[String],
    _padding: usize,
) -> Vec<String> {
    let mut output = Vec::new();
    for i in info_lines {
        output.push(i.clone());
    }
    output.push(String::new());
    for l in logo_lines {
        output.push(l.clone());
    }
    output
}

fn render_with_border(lines: &[String], style: &str, border_color_hex: &str) {
    let (tl, tr, bl, br, hz, vt) = match style {
        "double" => ("╔", "╗", "╚", "╝", "═", "║"),
        "rounded" => ("╭", "╮", "╰", "╯", "─", "│"),
        "bold" => ("┏", "┓", "┗", "┛", "━", "┃"),
        _ => ("┌", "┐", "└", "┘", "─", "│"),
    };

    let (r, g, b) = hex_to_rgb(border_color_hex);
    let max_width = lines.iter().map(|l| visible_width(l)).max().unwrap_or(40);
    let inner_width = max_width + 2;

    let top_border = format!("{}{}{}", tl, hz.repeat(inner_width), tr);
    println!("{}", top_border.truecolor(r, g, b));

    for line in lines {
        let width = visible_width(line);
        let padding = " ".repeat(max_width.saturating_sub(width));
        let formatted = format!(
            "{} {}{} {}",
            vt.truecolor(r, g, b),
            line,
            padding,
            vt.truecolor(r, g, b)
        );
        println!("{}", formatted);
    }

    let bottom_border = format!("{}{}{}", bl, hz.repeat(inner_width), br);
    println!("{}", bottom_border.truecolor(r, g, b));
}

pub fn display_logo_only(config: &Config) {
    let logo_manager = LogoManager::new();
    let override_distro = if config.logo.distro != "auto" {
        Some(config.logo.distro.as_str())
    } else {
        None
    };
    let palette = Config::get_palette(&config.theme.name);
    let logo = logo_manager.get_detected_logo(override_distro);
    let logo_art = get_colored_logo_art(logo, &config.output.logo_size, &palette);

    for line in logo_art {
        println!("{}", line);
    }
}

pub fn list_available_logos() {
    let logo_manager = LogoManager::new();
    let logos = logo_manager.list_available_logos();

    println!("Available distribution logos ({}):", logos.len());
    for logo in logos {
        println!("  • {}", logo);
    }
}

pub fn list_available_themes() {
    let themes = Config::list_available_themes();
    println!("Available themes ({}):", themes.len());
    for theme in themes {
        println!("  • {}", theme);
    }
}

pub fn display_json(system_info: &HashMap<String, String>) {
    if let Ok(json) = serde_json::to_string_pretty(system_info) {
        println!("{}", json);
    }
}

pub fn display_yaml(system_info: &HashMap<String, String>) {
    if let Ok(yaml) = serde_yaml::to_string(system_info) {
        println!("{}", yaml);
    }
}
