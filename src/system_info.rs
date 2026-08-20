// src/system_info.rs
// Fast, accurate system information extractor for Linux, Termux/Android, and macOS.

use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::process::Command;
use sysinfo::{CpuExt, SystemExt};

pub struct SystemInfo;

impl SystemInfo {
    pub fn is_termux() -> bool {
        std::env::var("TERMUX_VERSION").is_ok()
            || std::env::var("PREFIX")
                .map(|p| p.contains("/data/data/com.termux"))
                .unwrap_or(false)
    }

    pub fn is_android() -> bool {
        std::env::var("ANDROID_DATA").is_ok()
            || std::env::var("ANDROID_ROOT").is_ok()
            || Path::new("/system").exists()
    }

    pub fn get_title() -> String {
        let username = whoami::username();
        let hostname = whoami::fallible::hostname().unwrap_or_else(|_| "localhost".to_string());
        format!("{}@{}", username, hostname)
    }

    pub fn get_os_info() -> String {
        if Self::is_termux() {
            let android_ver = Self::get_android_prop("ro.build.version.release")
                .unwrap_or_else(|| "Unknown".to_string());
            return format!(
                "Android {} (Termux) {}",
                android_ver,
                std::env::consts::ARCH
            );
        }

        if Self::is_android() {
            let android_ver = Self::get_android_prop("ro.build.version.release")
                .unwrap_or_else(|| "Unknown".to_string());
            return format!("Android {} {}", android_ver, std::env::consts::ARCH);
        }

        // On Linux, read /etc/os-release
        if let Ok(content) = fs::read_to_string("/etc/os-release") {
            let mut pretty_name = None;
            let mut name = None;
            let mut version = None;

            for line in content.lines() {
                if let Some(val) = line.strip_prefix("PRETTY_NAME=") {
                    pretty_name = Some(val.trim_matches('"').trim_matches('\'').to_string());
                } else if let Some(val) = line.strip_prefix("NAME=") {
                    name = Some(val.trim_matches('"').trim_matches('\'').to_string());
                } else if let Some(val) = line.strip_prefix("VERSION=") {
                    version = Some(val.trim_matches('"').trim_matches('\'').to_string());
                }
            }

            if let Some(p) = pretty_name {
                return format!("{} {}", p, std::env::consts::ARCH);
            } else if let Some(n) = name {
                if let Some(v) = version {
                    return format!("{} {} {}", n, v, std::env::consts::ARCH);
                }
                return format!("{} {}", n, std::env::consts::ARCH);
            }
        }

        if cfg!(target_os = "macos") {
            let product_name = Command::new("sw_vers")
                .arg("-productName")
                .output()
                .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
                .unwrap_or_else(|_| "macOS".to_string());
            let product_version = Command::new("sw_vers")
                .arg("-productVersion")
                .output()
                .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
                .unwrap_or_default();
            return format!(
                "{} {} {}",
                product_name,
                product_version,
                std::env::consts::ARCH
            )
            .trim()
            .to_string();
        }

        // Fallback to uname
        let os_type = Command::new("uname")
            .arg("-s")
            .output()
            .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
            .unwrap_or_else(|_| "Linux".to_string());
        format!("{} {}", os_type, std::env::consts::ARCH)
    }

    pub fn get_host_info() -> Option<String> {
        if Self::is_termux() || Self::is_android() {
            let brand = Self::get_android_prop("ro.product.brand");
            let model = Self::get_android_prop("ro.product.model");
            match (brand, model) {
                (Some(b), Some(m)) => return Some(format!("{} {}", b, m)),
                (None, Some(m)) => return Some(m),
                _ => {}
            }
        }

        // Check DMI sysfs (Linux)
        let product_name = fs::read_to_string("/sys/devices/virtual/dmi/id/product_name")
            .or_else(|_| fs::read_to_string("/sys/class/dmi/id/product_name"))
            .map(|s| s.trim().to_string())
            .ok();

        let product_version = fs::read_to_string("/sys/devices/virtual/dmi/id/product_version")
            .or_else(|_| fs::read_to_string("/sys/class/dmi/id/product_version"))
            .map(|s| s.trim().to_string())
            .ok();

        if let Some(name) = product_name {
            if !name.is_empty() && name != "None" && name != "System Product Name" {
                if let Some(ver) = product_version {
                    if !ver.is_empty()
                        && ver != "None"
                        && ver != "System Version"
                        && !name.contains(&ver)
                    {
                        return Some(format!("{} {}", name, ver));
                    }
                }
                return Some(name);
            }
        }

        // Device tree model (ARM SBCs like Raspberry Pi)
        if let Ok(model) = fs::read_to_string("/proc/device-tree/model") {
            let cleaned = model.trim_matches('\0').trim().to_string();
            if !cleaned.is_empty() {
                return Some(cleaned);
            }
        }

        None
    }

    pub fn get_kernel_version() -> String {
        fs::read_to_string("/proc/sys/kernel/osrelease")
            .map(|s| s.trim().to_string())
            .unwrap_or_else(|_| {
                Command::new("uname")
                    .arg("-r")
                    .output()
                    .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
                    .unwrap_or_else(|_| "Unknown".to_string())
            })
    }

    pub fn get_uptime_pretty() -> String {
        if let Ok(content) = fs::read_to_string("/proc/uptime") {
            if let Some(first) = content.split_whitespace().next() {
                if let Ok(seconds_f) = first.parse::<f64>() {
                    let total_secs = seconds_f as u64;
                    let days = total_secs / 86400;
                    let hours = (total_secs % 86400) / 3600;
                    let minutes = (total_secs % 3600) / 60;

                    if days > 0 {
                        return format!("{}d {}h {}m", days, hours, minutes);
                    } else if hours > 0 {
                        return format!("{}h {}m", hours, minutes);
                    } else {
                        return format!("{}m", minutes);
                    }
                }
            }
        }

        let sys = sysinfo::System::new();
        let uptime = sys.uptime();
        let days = uptime / 86400;
        let hours = (uptime % 86400) / 3600;
        let minutes = (uptime % 3600) / 60;

        if days > 0 {
            format!("{}d {}h {}m", days, hours, minutes)
        } else if hours > 0 {
            format!("{}h {}m", hours, minutes)
        } else {
            format!("{}m", minutes)
        }
    }

    pub fn get_packages_count() -> Option<String> {
        let mut packages = Vec::new();

        // 1. Nix System & User packages
        if Path::new("/nix/store").exists() {
            let mut nix_count = 0;
            if let Ok(entries) = fs::read_dir("/run/current-system/sw/bin") {
                nix_count += entries.count();
            }
            if nix_count > 0 {
                packages.push(format!("{} (nix-system)", nix_count));
            }
            if let Some(home) = dirs::home_dir() {
                let user_nix = home.join(".nix-profile/bin");
                if let Ok(entries) = fs::read_dir(user_nix) {
                    let user_count = entries.count();
                    if user_count > 0 {
                        packages.push(format!("{} (nix-user)", user_count));
                    }
                }
            }
        }

        // 2. Pacman (Arch Linux)
        if Path::new("/var/lib/pacman/local").exists() {
            if let Ok(entries) = fs::read_dir("/var/lib/pacman/local") {
                let count = entries
                    .filter(|e| e.as_ref().map(|d| d.path().is_dir()).unwrap_or(false))
                    .count();
                if count > 1 {
                    // Subtract ALPM_DB_VERSION entry if present
                    packages.push(format!("{} (pacman)", count.saturating_sub(1)));
                }
            }
        }

        // 3. DPKG / APT (Debian, Ubuntu)
        if Path::new("/var/lib/dpkg/status").exists() {
            if let Ok(content) = fs::read_to_string("/var/lib/dpkg/status") {
                let count = content
                    .lines()
                    .filter(|l| l.starts_with("Status: install ok installed"))
                    .count();
                if count > 0 {
                    packages.push(format!("{} (dpkg)", count));
                }
            }
        }

        // 4. RPM (Fedora, RHEL, openSUSE)
        if Path::new("/var/lib/rpm").exists() {
            if let Ok(output) = Command::new("rpm")
                .args(["-qa", "--nodigest", "--nosignature"])
                .output()
            {
                let count = output.stdout.iter().filter(|&&b| b == b'\n').count();
                if count > 0 {
                    packages.push(format!("{} (rpm)", count));
                }
            }
        }

        // 5. Flatpak
        let mut flatpak_count = 0;
        if let Ok(entries) = fs::read_dir("/var/lib/flatpak/app") {
            flatpak_count += entries.count();
        }
        if let Some(home) = dirs::home_dir() {
            if let Ok(entries) = fs::read_dir(home.join(".local/share/flatpak/app")) {
                flatpak_count += entries.count();
            }
        }
        if flatpak_count > 0 {
            packages.push(format!("{} (flatpak)", flatpak_count));
        }

        // 6. Snap
        if Path::new("/snap").exists() {
            if let Ok(entries) = fs::read_dir("/snap") {
                let count = entries
                    .filter(|e| {
                        e.as_ref()
                            .map(|d| {
                                d.path().is_dir()
                                    && !d.file_name().to_string_lossy().starts_with("bin")
                            })
                            .unwrap_or(false)
                    })
                    .count();
                if count > 0 {
                    packages.push(format!("{} (snap)", count));
                }
            }
        }

        // 7. Alpine APK
        if Path::new("/lib/apk/db/installed").exists() {
            if let Ok(content) = fs::read_to_string("/lib/apk/db/installed") {
                let count = content.lines().filter(|l| l.starts_with("P:")).count();
                if count > 0 {
                    packages.push(format!("{} (apk)", count));
                }
            }
        }

        // 8. Homebrew (macOS / Linux)
        let brew_cellar = Path::new("/opt/homebrew/Cellar");
        let brew_cellar_usr = Path::new("/usr/local/Cellar");
        if brew_cellar.exists() {
            if let Ok(entries) = fs::read_dir(brew_cellar) {
                let count = entries.count();
                if count > 0 {
                    packages.push(format!("{} (brew)", count));
                }
            }
        } else if brew_cellar_usr.exists() {
            if let Ok(entries) = fs::read_dir(brew_cellar_usr) {
                let count = entries.count();
                if count > 0 {
                    packages.push(format!("{} (brew)", count));
                }
            }
        }

        // 9. Void XBPS
        if Path::new("/var/db/xbps").exists() {
            if let Ok(output) = Command::new("xbps-query").arg("-l").output() {
                let count = output.stdout.iter().filter(|&&b| b == b'\n').count();
                if count > 0 {
                    packages.push(format!("{} (xbps)", count));
                }
            }
        }

        // 10. Gentoo Portage
        if Path::new("/var/db/pkg").exists() {
            if let Ok(entries) = fs::read_dir("/var/db/pkg") {
                let mut count = 0;
                for cat in entries.flatten() {
                    if let Ok(sub) = fs::read_dir(cat.path()) {
                        count += sub.count();
                    }
                }
                if count > 0 {
                    packages.push(format!("{} (emerge)", count));
                }
            }
        }

        // 11. Cargo binaries
        if let Some(home) = dirs::home_dir() {
            if let Ok(entries) = fs::read_dir(home.join(".cargo/bin")) {
                let count = entries.count();
                if count > 0 && packages.is_empty() {
                    packages.push(format!("{} (cargo)", count));
                }
            }
        }

        if packages.is_empty() {
            None
        } else {
            Some(packages.join(", "))
        }
    }

    pub fn get_shell() -> String {
        if let Ok(shell_path) = std::env::var("SHELL") {
            let shell_name = Path::new(&shell_path)
                .file_name()
                .and_then(|s| s.to_str())
                .unwrap_or(&shell_path);

            // Attempt to get shell version
            let version = Command::new(shell_name)
                .arg("--version")
                .output()
                .ok()
                .and_then(|out| {
                    let s = String::from_utf8_lossy(&out.stdout);
                    s.split_whitespace()
                        .find(|w| w.chars().any(|c| c.is_ascii_digit()))
                        .map(|w| w.trim_matches(',').to_string())
                });

            if let Some(ver) = version {
                return format!("{} {}", shell_name, ver);
            }
            return shell_name.to_string();
        }
        "Unknown".to_string()
    }

    pub fn get_terminal() -> Option<String> {
        // Direct terminal environment variables
        if let Ok(term) = std::env::var("TERM_PROGRAM") {
            if let Ok(ver) = std::env::var("TERM_PROGRAM_VERSION") {
                return Some(format!("{} {}", term, ver));
            }
            return Some(term);
        }

        if let Ok(term) = std::env::var("TERMINAL") {
            return Some(term);
        }

        if let Ok(term_emulator) = std::env::var("TERMINAL_EMULATOR") {
            return Some(term_emulator);
        }

        if let Ok(alacritty) = std::env::var("ALACRITTY_LOG") {
            if !alacritty.is_empty() {
                return Some("Alacritty".to_string());
            }
        }

        if std::env::var("KITTY_PID").is_ok() || std::env::var("KITTY_WINDOW_ID").is_ok() {
            return Some("kitty".to_string());
        }

        if std::env::var("WEZTERM_PANE").is_ok() {
            return Some("WezTerm".to_string());
        }

        if std::env::var("FOOT_SERVER_PATH").is_ok() {
            return Some("foot".to_string());
        }

        if std::env::var("TMUX").is_ok() {
            return Some("tmux".to_string());
        }

        if let Ok(term) = std::env::var("TERM") {
            if term != "xterm-256color" && term != "xterm" && term != "screen" {
                return Some(term);
            }
        }

        None
    }

    pub fn get_desktop_environment() -> Option<String> {
        if Self::is_termux() || Self::is_android() {
            return None;
        }

        let de = std::env::var("XDG_CURRENT_DESKTOP")
            .or_else(|_| std::env::var("DESKTOP_SESSION"))
            .or_else(|_| std::env::var("XDG_SESSION_DESKTOP"))
            .ok()?;

        let de_clean = match de.to_lowercase().as_str() {
            "gnome" | "ubuntu:gnome" => "GNOME".to_string(),
            "kde" | "plasma" | "plasmashell" => "KDE Plasma".to_string(),
            "xfce" | "xfce4" => "XFCE".to_string(),
            "cinnamon" => "Cinnamon".to_string(),
            "mate" => "MATE".to_string(),
            "lxqt" => "LXQt".to_string(),
            "cosmic" => "COSMIC".to_string(),
            "pantheon" => "Pantheon".to_string(),
            "deepin" => "Deepin".to_string(),
            _ => de,
        };

        Some(de_clean)
    }

    pub fn get_window_manager() -> Option<String> {
        if Self::is_termux() || Self::is_android() {
            return Some("SurfaceFlinger".to_string());
        }

        // Check common Wayland/X11 environment variables
        if let Ok(wm) = std::env::var("XDG_CURRENT_DESKTOP") {
            let wm_lower = wm.to_lowercase();
            if wm_lower.contains("niri") {
                return Some("niri".to_string());
            } else if wm_lower.contains("hyprland") {
                return Some("Hyprland".to_string());
            } else if wm_lower.contains("sway") {
                return Some("Sway".to_string());
            } else if wm_lower.contains("river") {
                return Some("River".to_string());
            } else if wm_lower.contains("wayfire") {
                return Some("Wayfire".to_string());
            } else if wm_lower.contains("i3") {
                return Some("i3".to_string());
            } else if wm_lower.contains("bspwm") {
                return Some("bspwm".to_string());
            } else if wm_lower.contains("dwm") {
                return Some("dwm".to_string());
            } else if wm_lower.contains("awesome") {
                return Some("awesome".to_string());
            } else if wm_lower.contains("xmonad") {
                return Some("xmonad".to_string());
            }
        }

        if std::env::var("HYPRLAND_INSTANCE_SIGNATURE").is_ok() {
            return Some("Hyprland".to_string());
        }
        if std::env::var("SWAYSOCK").is_ok() {
            return Some("Sway".to_string());
        }
        if std::env::var("I3SOCK").is_ok() {
            return Some("i3".to_string());
        }

        // On Wayland, display server name can give hints
        if std::env::var("WAYLAND_DISPLAY").is_ok() {
            if let Ok(desktop) = std::env::var("DESKTOP_SESSION") {
                return Some(desktop);
            }
        }

        None
    }

    pub fn get_cpu_info() -> String {
        if Self::is_termux() || Self::is_android() {
            if let Ok(content) = fs::read_to_string("/proc/cpuinfo") {
                for line in content.lines() {
                    if line.starts_with("Hardware")
                        || line.starts_with("model name")
                        || line.starts_with("Processor")
                    {
                        if let Some(part) = line.split(':').nth(1) {
                            let name = part.trim();
                            if !name.is_empty() {
                                return name.to_string();
                            }
                        }
                    }
                }
            }
            return "ARM Processor".to_string();
        }

        if let Ok(content) = fs::read_to_string("/proc/cpuinfo") {
            let mut model_name = None;
            let mut cores = 0;
            let mut mhz = None;

            for line in content.lines() {
                if line.starts_with("model name") && model_name.is_none() {
                    model_name = line.split(':').nth(1).map(|s| s.trim().to_string());
                } else if line.starts_with("processor") {
                    cores += 1;
                } else if line.starts_with("cpu MHz") && mhz.is_none() {
                    mhz = line
                        .split(':')
                        .nth(1)
                        .and_then(|s| s.trim().parse::<f64>().ok());
                }
            }

            if let Some(mut model) = model_name {
                // Clean CPU noise
                model = model
                    .replace("(R)", "")
                    .replace("(TM)", "")
                    .replace("CPU", "")
                    .replace("Dual-Core", "")
                    .replace("Quad-Core", "")
                    .replace("Six-Core", "")
                    .replace("Eight-Core", "")
                    .replace("Processor", "");

                // Collapse multiple spaces
                let cleaned_model = model.split_whitespace().collect::<Vec<_>>().join(" ");

                if cores > 0 {
                    if let Some(freq) = mhz {
                        let ghz = freq / 1000.0;
                        return format!("{} ({}) @ {:.2}GHz", cleaned_model, cores, ghz);
                    }
                    return format!("{} ({})", cleaned_model, cores);
                }
                return cleaned_model;
            }
        }

        let mut sys = sysinfo::System::new();
        sys.refresh_cpu();
        if let Some(cpu) = sys.cpus().first() {
            let brand = cpu.brand();
            if !brand.is_empty() {
                return brand.to_string();
            }
        }

        "Unknown CPU".to_string()
    }

    fn clean_gpu_name(vendor: &str, model: &str) -> String {
        let mut name = model.to_string();

        // If bracketed name exists like "Kaby Lake-R GT2 [UHD Graphics 620]", prefer "[UHD Graphics 620]"
        if let Some(start) = name.find('[') {
            if let Some(end) = name.rfind(']') {
                if end > start {
                    let bracket_content = name[start + 1..end].trim();
                    if !bracket_content.is_empty() {
                        name = bracket_content.to_string();
                    }
                }
            }
        }

        // Clean common noise
        name = name
            .replace("Corporation", "")
            .replace("(R)", "")
            .replace("(TM)", "")
            .replace("Integrated Graphics Controller", "Graphics")
            .replace("VGA compatible controller:", "")
            .replace("3D controller:", "")
            .replace("Display controller:", "");

        let name_lower = name.to_lowercase();
        let vendor_lower = vendor.to_lowercase();

        if (vendor_lower.contains("intel") || vendor_lower == "0x8086")
            && !name_lower.starts_with("intel")
        {
            name = format!("Intel {}", name);
        } else if (vendor_lower.contains("nvidia") || vendor_lower == "0x10de")
            && !name_lower.starts_with("nvidia")
        {
            name = format!("NVIDIA {}", name);
        } else if (vendor_lower.contains("amd")
            || vendor_lower.contains("ati")
            || vendor_lower.contains("advanced micro devices")
            || vendor_lower == "0x1002")
            && !name_lower.starts_with("amd")
        {
            name = format!("AMD {}", name);
        }

        name.split_whitespace().collect::<Vec<_>>().join(" ")
    }

    pub fn get_gpu_info() -> Option<String> {
        if Self::is_termux() || Self::is_android() {
            if let Ok(output) = Command::new("dumpsys").arg("SurfaceFlinger").output() {
                if let Ok(content) = String::from_utf8(output.stdout) {
                    if let Some(line) = content.lines().find(|l| l.contains("GLES:")) {
                        return Some(
                            line.split("GLES:")
                                .nth(1)
                                .unwrap_or("ARM GPU")
                                .trim()
                                .to_string(),
                        );
                    }
                }
            }
            return Some("ARM Mali / Adreno GPU".to_string());
        }

        #[cfg(target_os = "macos")]
        {
            if let Ok(output) = Command::new("system_profiler")
                .arg("SPDisplaysDataType")
                .output()
            {
                let text = String::from_utf8_lossy(&output.stdout);
                for line in text.lines() {
                    if line.trim().starts_with("Chipset Model:") {
                        if let Some(name) = line.split(':').nth(1) {
                            return Some(name.trim().to_string());
                        }
                    }
                }
            }
        }

        #[cfg(target_os = "windows")]
        {
            if let Ok(output) = Command::new("wmic")
                .args(["path", "win32_VideoController", "get", "name"])
                .output()
            {
                let text = String::from_utf8_lossy(&output.stdout);
                for line in text.lines().skip(1) {
                    let clean = line.trim();
                    if !clean.is_empty() {
                        return Some(clean.to_string());
                    }
                }
            }
        }

        let mut gpus: Vec<String> = Vec::new();

        // 1. Scan PCI devices in sysfs + udevadm info
        if let Ok(entries) = fs::read_dir("/sys/bus/pci/devices") {
            for entry in entries.flatten() {
                let path = entry.path();
                let class_path = path.join("class");
                if let Ok(class) = fs::read_to_string(&class_path) {
                    let class_trim = class.trim();
                    // 0x030000 = VGA, 0x038000 = Display controller, 0x030200 = 3D controller
                    if class_trim.starts_with("0x0300")
                        || class_trim.starts_with("0x0380")
                        || class_trim.starts_with("0x0302")
                    {
                        let vendor_id = fs::read_to_string(path.join("vendor"))
                            .unwrap_or_default()
                            .trim()
                            .to_string();
                        let device_id = fs::read_to_string(path.join("device"))
                            .unwrap_or_default()
                            .trim()
                            .to_string();

                        let mut found_name = None;

                        // Method 1a: Try udevadm info (Available on systemd / udev Linux systems)
                        if let Ok(output) = Command::new("udevadm")
                            .args(["info", "-p", path.to_str().unwrap_or("")])
                            .output()
                        {
                            if output.status.success() {
                                let text = String::from_utf8_lossy(&output.stdout);
                                let mut vendor_db = String::new();
                                let mut model_db = String::new();

                                for line in text.lines() {
                                    if let Some(v) =
                                        line.strip_prefix("E: ID_VENDOR_FROM_DATABASE=")
                                    {
                                        vendor_db = v.trim().to_string();
                                    } else if let Some(m) =
                                        line.strip_prefix("E: ID_MODEL_FROM_DATABASE=")
                                    {
                                        model_db = m.trim().to_string();
                                    }
                                }

                                if !model_db.is_empty() {
                                    let clean = Self::clean_gpu_name(&vendor_db, &model_db);
                                    if !clean.is_empty() {
                                        found_name = Some(clean);
                                    }
                                }
                            }
                        }

                        // Method 1b: Try lspci -s <slot>
                        if found_name.is_none() {
                            if let Ok(output) = Command::new("lspci")
                                .arg("-s")
                                .arg(entry.file_name())
                                .output()
                            {
                                if output.status.success() {
                                    let text = String::from_utf8_lossy(&output.stdout);
                                    if let Some(part) = text.split(':').nth(2) {
                                        let clean = Self::clean_gpu_name(&vendor_id, part.trim());
                                        if !clean.is_empty() {
                                            found_name = Some(clean);
                                        }
                                    }
                                }
                            }
                        }

                        // Method 1c: Fallback based on vendor ID
                        if found_name.is_none() {
                            let v_name = match vendor_id.as_str() {
                                "0x8086" => "Intel",
                                "0x10de" => "NVIDIA",
                                "0x1002" => "AMD",
                                _ => "",
                            };
                            if !v_name.is_empty() {
                                found_name = Some(format!("{} GPU ({})", v_name, device_id));
                            }
                        }

                        if let Some(name) = found_name {
                            if !gpus.contains(&name) {
                                gpus.push(name);
                            }
                        }
                    }
                }
            }
        }

        // 2. Fallback: vulkaninfo --summary / glxinfo -B
        if gpus.is_empty() {
            if let Ok(output) = Command::new("vulkaninfo").arg("--summary").output() {
                if output.status.success() {
                    let text = String::from_utf8_lossy(&output.stdout);
                    for line in text.lines() {
                        if line.contains("deviceName") && line.contains('=') {
                            if let Some(name) = line.split('=').nth(1) {
                                let clean = name.trim();
                                if !clean.to_lowercase().contains("llvmpipe") && !clean.is_empty() {
                                    gpus.push(Self::clean_gpu_name("", clean));
                                    break;
                                }
                            }
                        }
                    }
                }
            }
        }

        if gpus.is_empty() {
            if let Ok(output) = Command::new("glxinfo").arg("-B").output() {
                if output.status.success() {
                    let text = String::from_utf8_lossy(&output.stdout);
                    for line in text.lines() {
                        if line.contains("OpenGL renderer string:") {
                            if let Some(name) = line.split(':').nth(1) {
                                let clean = name.replace("Mesa", "").trim().to_string();
                                if !clean.to_lowercase().contains("llvmpipe") && !clean.is_empty() {
                                    gpus.push(Self::clean_gpu_name("", &clean));
                                    break;
                                }
                            }
                        }
                    }
                }
            }
        }

        if !gpus.is_empty() {
            return Some(gpus.join(", "));
        }

        None
    }

    pub fn get_memory_info() -> String {
        if let Ok(content) = fs::read_to_string("/proc/meminfo") {
            let mut total_kb = 0u64;
            let mut avail_kb = 0u64;

            for line in content.lines() {
                if line.starts_with("MemTotal:") {
                    total_kb = line
                        .split_whitespace()
                        .nth(1)
                        .and_then(|s| s.parse().ok())
                        .unwrap_or(0);
                } else if line.starts_with("MemAvailable:") {
                    avail_kb = line
                        .split_whitespace()
                        .nth(1)
                        .and_then(|s| s.parse().ok())
                        .unwrap_or(0);
                }
            }

            if total_kb > 0 {
                let used_kb = total_kb.saturating_sub(avail_kb);
                let total_gib = total_kb as f64 / (1024.0 * 1024.0);
                let used_gib = used_kb as f64 / (1024.0 * 1024.0);
                let percent = (used_gib / total_gib) * 100.0;
                return format!(
                    "{:.2} GiB / {:.2} GiB ({:.0}%)",
                    used_gib, total_gib, percent
                );
            }
        }

        let mut sys = sysinfo::System::new();
        sys.refresh_memory();
        let total_gib = sys.total_memory() as f64 / (1024.0 * 1024.0 * 1024.0);
        let used_gib = sys.used_memory() as f64 / (1024.0 * 1024.0 * 1024.0);
        let percent = if total_gib > 0.0 {
            (used_gib / total_gib) * 100.0
        } else {
            0.0
        };
        format!(
            "{:.2} GiB / {:.2} GiB ({:.0}%)",
            used_gib, total_gib, percent
        )
    }

    fn is_valid_disk_mount(mount: &str) -> bool {
        if mount == "/" || mount == "/home" || mount.starts_with("/home/") {
            return true;
        }
        if mount.starts_with("/mnt")
            || mount.starts_with("/media")
            || mount.starts_with("/run/media")
            || mount.starts_with("/Volumes")
            || mount.starts_with("/data")
            || mount.starts_with("/storage")
            || mount.starts_with("/drive")
        {
            return true;
        }
        false
    }

    pub fn get_disk_info() -> String {
        use sysinfo::{DiskExt, SystemExt};
        let mut sys = sysinfo::System::new();
        sys.refresh_disks_list();

        let mut primary_disks: Vec<(String, f64, f64, f64, String)> = Vec::new();
        let mut seen_spaces = std::collections::HashSet::new();

        for disk in sys.disks() {
            let mount = disk.mount_point().to_string_lossy().to_string();
            let fs_name = String::from_utf8_lossy(disk.file_system()).to_lowercase();

            // Only allow valid user/primary disk mounts
            if !Self::is_valid_disk_mount(&mount) {
                continue;
            }

            // Ignore virtual filesystems
            if fs_name == "tmpfs"
                || fs_name == "devtmpfs"
                || fs_name == "overlay"
                || fs_name == "squashfs"
                || fs_name == "ramfs"
                || fs_name == "efivarfs"
                || fs_name == "nullfs"
                || mount.contains("@")
            {
                continue;
            }

            let total_gib = disk.total_space() as f64 / (1024.0 * 1024.0 * 1024.0);
            if total_gib < 1.0 {
                continue;
            }

            let avail_gib = disk.available_space() as f64 / (1024.0 * 1024.0 * 1024.0);
            let used_gib = total_gib - avail_gib;
            let percent = (used_gib / total_gib) * 100.0;

            // Deduplicate partitions having identical space & used space (btrfs subvolumes / bind mounts)
            let space_key = ((total_gib * 100.0) as u64, (used_gib * 100.0) as u64);
            if seen_spaces.insert(space_key) {
                primary_disks.push((mount, used_gib, total_gib, percent, fs_name));
            }
        }

        // If no disks matched filters, add root '/'
        if primary_disks.is_empty() {
            for disk in sys.disks() {
                if disk.mount_point().to_string_lossy() == "/" {
                    let total_gib = disk.total_space() as f64 / (1024.0 * 1024.0 * 1024.0);
                    let avail_gib = disk.available_space() as f64 / (1024.0 * 1024.0 * 1024.0);
                    let used_gib = total_gib - avail_gib;
                    let percent = (used_gib / total_gib) * 100.0;
                    let fs_name = String::from_utf8_lossy(disk.file_system()).to_string();
                    primary_disks.push(("/".to_string(), used_gib, total_gib, percent, fs_name));
                    break;
                }
            }
        }

        primary_disks.sort_by(|a, b| a.0.cmp(&b.0));

        let mut lines = Vec::new();
        for (mount, used, total, percent, fs_name) in primary_disks {
            lines.push(format!(
                "{}: {:.2} GiB / {:.2} GiB ({:.0}%) [{}]",
                mount, used, total, percent, fs_name
            ));
        }

        if lines.is_empty() {
            "No disks found".to_string()
        } else {
            lines.join("\n")
        }
    }

    pub fn get_battery_info() -> Option<String> {
        // Scan /sys/class/power_supply/BAT*
        if let Ok(entries) = fs::read_dir("/sys/class/power_supply") {
            for entry in entries.flatten() {
                let name = entry.file_name().to_string_lossy().to_string();
                if name.starts_with("BAT") {
                    let cap_path = entry.path().join("capacity");
                    let status_path = entry.path().join("status");

                    if let Ok(cap_str) = fs::read_to_string(cap_path) {
                        let cap = cap_str.trim();
                        let status = fs::read_to_string(status_path).unwrap_or_default();
                        let status_clean = status.trim();

                        if !status_clean.is_empty() {
                            return Some(format!("{}% [{}]", cap, status_clean));
                        }
                        return Some(format!("{}%", cap));
                    }
                }
            }
        }
        None
    }

    fn get_android_prop(prop_name: &str) -> Option<String> {
        Command::new("getprop")
            .arg(prop_name)
            .output()
            .ok()
            .and_then(|out| {
                let val = String::from_utf8_lossy(&out.stdout).trim().to_string();
                if val.is_empty() {
                    None
                } else {
                    Some(val)
                }
            })
    }
}

pub fn get_system_info() -> HashMap<String, String> {
    let mut info = HashMap::new();

    info.insert("Title".to_string(), SystemInfo::get_title());
    info.insert("OS".to_string(), SystemInfo::get_os_info());

    if let Some(host) = SystemInfo::get_host_info() {
        info.insert("Host".to_string(), host);
    }

    info.insert("Kernel".to_string(), SystemInfo::get_kernel_version());
    info.insert("Uptime".to_string(), SystemInfo::get_uptime_pretty());

    if let Some(pkgs) = SystemInfo::get_packages_count() {
        info.insert("Packages".to_string(), pkgs);
    }

    info.insert("Shell".to_string(), SystemInfo::get_shell());

    if let Some(de) = SystemInfo::get_desktop_environment() {
        info.insert("DE".to_string(), de);
    }

    if let Some(wm) = SystemInfo::get_window_manager() {
        info.insert("WM".to_string(), wm);
    }

    if let Some(term) = SystemInfo::get_terminal() {
        info.insert("Terminal".to_string(), term);
    }

    info.insert("CPU".to_string(), SystemInfo::get_cpu_info());

    if let Some(gpu) = SystemInfo::get_gpu_info() {
        info.insert("GPU".to_string(), gpu);
    }

    info.insert("Memory".to_string(), SystemInfo::get_memory_info());
    info.insert("Disk".to_string(), SystemInfo::get_disk_info());

    if let Some(battery) = SystemInfo::get_battery_info() {
        info.insert("Battery".to_string(), battery);
    }

    info
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clean_gpu_name() {
        let name =
            SystemInfo::clean_gpu_name("Intel Corporation", "Kaby Lake-R GT2 [UHD Graphics 620]");
        assert_eq!(name, "Intel UHD Graphics 620");

        let name_nvidia =
            SystemInfo::clean_gpu_name("NVIDIA Corporation", "GA106 [GeForce RTX 3060]");
        assert_eq!(name_nvidia, "NVIDIA GeForce RTX 3060");

        let name_amd = SystemInfo::clean_gpu_name(
            "Advanced Micro Devices, Inc.",
            "Navi 22 [Radeon RX 6700 XT]",
        );
        assert_eq!(name_amd, "AMD Radeon RX 6700 XT");
    }

    #[test]
    fn test_system_info_collection() {
        let info = get_system_info();
        assert!(info.contains_key("Title"));
        assert!(info.contains_key("OS"));
        assert!(info.contains_key("Kernel"));
        assert!(info.contains_key("Memory"));
    }

    #[test]
    fn test_valid_disk_mount() {
        assert!(SystemInfo::is_valid_disk_mount("/"));
        assert!(SystemInfo::is_valid_disk_mount("/home"));
        assert!(SystemInfo::is_valid_disk_mount("/home/user"));
        assert!(SystemInfo::is_valid_disk_mount("/mnt/data"));
        assert!(SystemInfo::is_valid_disk_mount("/media/usb"));
        assert!(SystemInfo::is_valid_disk_mount("/run/media/user/drive"));
        assert!(SystemInfo::is_valid_disk_mount("/Volumes/Macintosh HD"));

        assert!(!SystemInfo::is_valid_disk_mount("/boot"));
        assert!(!SystemInfo::is_valid_disk_mount("/boot/efi"));
        assert!(!SystemInfo::is_valid_disk_mount("/usr"));
        assert!(!SystemInfo::is_valid_disk_mount("/usr/local"));
        assert!(!SystemInfo::is_valid_disk_mount("/var"));
        assert!(!SystemInfo::is_valid_disk_mount("/srv"));
        assert!(!SystemInfo::is_valid_disk_mount("/tmp"));
        assert!(!SystemInfo::is_valid_disk_mount("/.host-etc"));
        assert!(!SystemInfo::is_valid_disk_mount("/nix"));
    }
}
