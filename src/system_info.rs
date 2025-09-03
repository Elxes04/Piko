// src/system_info.rs

use std::collections::HashMap;
use std::fs;
use std::process::Command;
use sysinfo::{DiskExt, System, SystemExt};

#[allow(dead_code)]
pub struct SystemInfo {
    pub os: String,
    pub cpu: String,
    pub memory: String,
    pub disk: String,
}

impl SystemInfo {
    fn is_termux() -> bool {
        std::env::var("TERMUX_VERSION").is_ok() || 
        std::env::var("PREFIX").map(|p| p.contains("/data/data/com.termux")).unwrap_or(false)
    }

    fn is_android() -> bool {
        std::env::var("ANDROID_DATA").is_ok() || 
        std::env::var("ANDROID_ROOT").is_ok() ||
        std::path::Path::new("/system").exists()
    }

    fn get_os_info() -> String {
        if Self::is_termux() {
            "Android (Termux)".to_string()
        } else if Self::is_android() {
            "Android".to_string()
        } else if cfg!(target_os = "linux") {
            Command::new("uname")
                .arg("-o")
                .output()
                .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
                .unwrap_or_else(|_| "Unknown OS".to_string())
        } else if cfg!(target_os = "macos") {
            Command::new("sw_vers")
                .arg("-productName")
                .output()
                .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
                .unwrap_or_else(|_| "Unknown OS".to_string())
        } else {
            "Unsupported OS".to_string()
        }
    }

    fn get_cpu_info() -> String {
        if Self::is_termux() || Self::is_android() {
            // On Android/Termux, try to get CPU info from /proc/cpuinfo
            fs::read_to_string("/proc/cpuinfo")
                .ok()
                .and_then(|content| {
                    content
                        .lines()
                        .find(|line| line.starts_with("Hardware"))
                        .map(|line| line.split(':').nth(1).unwrap_or("").trim().to_string())
                })
                .unwrap_or_else(|| "ARM CPU".to_string())
        } else {
            Command::new("lscpu")
                .output()
                .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
                .unwrap_or_else(|_| "Unknown CPU".to_string())
        }
    }

    fn get_disk_info() -> String {
        let mut system = System::new_all();
        system.refresh_disks_list();

        let mut disk_info = String::new();
        let mut physical_disks = Vec::new();

        for disk in system.disks() {
            let mount_point = disk.mount_point().to_string_lossy();
            let file_system = String::from_utf8_lossy(disk.file_system());
            
            // Filter out Android-specific partitions and subvolumes
            if Self::is_android() || Self::is_termux() {
                // Skip Android system partitions, subvolumes, and temporary mounts
                if mount_point.contains("/system") || 
                   mount_point.contains("/vendor") || 
                   mount_point.contains("/product") ||
                   mount_point.contains("/data") ||
                   mount_point.contains("/cache") ||
                   mount_point.contains("/mnt") ||
                   mount_point.contains("/storage") ||
                   mount_point.contains("/dev/block") ||
                   mount_point.contains("tmpfs") ||
                   mount_point.contains("proc") ||
                   mount_point.contains("sysfs") ||
                   mount_point.contains("devpts") ||
                   mount_point.contains("debugfs") ||
                   mount_point.contains("configfs") ||
                   mount_point.contains("selinuxfs") ||
                   mount_point.contains("cgroup") ||
                   mount_point.contains("pstore") ||
                   mount_point.contains("efivarfs") ||
                   mount_point.contains("fuse") ||
                   mount_point.contains("sdcardfs") ||
                   mount_point.contains("f2fs") ||
                   mount_point.contains("ext4") && mount_point.contains("/dev/block") {
                    continue;
                }
            } else {
                // On regular Linux, filter out subvolumes and temporary filesystems
                if mount_point.contains("@") || 
                   mount_point.contains("tmpfs") ||
                   mount_point.contains("proc") ||
                   mount_point.contains("sysfs") ||
                   mount_point.contains("devpts") ||
                   mount_point.contains("debugfs") ||
                   mount_point.contains("configfs") ||
                   mount_point.contains("selinuxfs") ||
                   mount_point.contains("cgroup") ||
                   mount_point.contains("pstore") ||
                   mount_point.contains("efivarfs") ||
                   mount_point.contains("fuse") ||
                   mount_point.contains("squashfs") ||
                   mount_point.contains("overlay") {
                    continue;
                }
            }

            // Only show actual disk partitions with meaningful space
            let total_space = disk.total_space() as f64 / 1024.0 / 1024.0 / 1024.0; // Convert to GiB
            if total_space > 0.1 { // Only show partitions with more than 100MB
                let available_space = disk.available_space() as f64 / 1024.0 / 1024.0 / 1024.0; // Convert to GiB
                let used_space = total_space - available_space;
                let used_percentage = (used_space / total_space) * 100.0;

                physical_disks.push((mount_point, total_space, used_space, used_percentage, file_system));
            }
        }

        // Sort by mount point for consistent output
        physical_disks.sort_by(|a, b| a.0.cmp(&b.0));

        for (mount_point, total_space, used_space, used_percentage, file_system) in physical_disks {
            disk_info.push_str(&format!(
                "Disk ({}): {:.2} GiB / {:.2} GiB ({:.0}%) - {}\n",
                mount_point, used_space, total_space, used_percentage, file_system
            ));
        }

        if disk_info.is_empty() {
            if Self::is_termux() || Self::is_android() {
                "No accessible disk partitions found (Android restrictions)".to_string()
            } else {
                "No disks found".to_string()
            }
        } else {
            disk_info.trim_end().to_string() // Remove trailing newline
        }
    }

    fn get_cpu_model() -> String {
        if Self::is_termux() || Self::is_android() {
            // On Android/Termux, try different CPU info fields
            fs::read_to_string("/proc/cpuinfo")
                .ok()
                .and_then(|content| {
                    // Try multiple possible fields for Android
                    content
                        .lines()
                        .find(|line| line.starts_with("Hardware"))
                        .or_else(|| content.lines().find(|line| line.starts_with("model name")))
                        .or_else(|| content.lines().find(|line| line.starts_with("Processor")))
                        .map(|line| line.split(':').nth(1).unwrap_or("").trim().to_string())
                })
                .unwrap_or_else(|| "ARM Processor".to_string())
        } else {
            fs::read_to_string("/proc/cpuinfo")
                .ok()
                .and_then(|content| {
                    content
                        .lines()
                        .find(|line| line.starts_with("model name"))
                        .map(|line| line.split(':').nth(1).unwrap_or("").trim().to_string())
                })
                .unwrap_or_else(|| "Unknown CPU Model".to_string())
        }
    }

    pub fn get_uptime_pretty(system: &System) -> String {
        let uptime_seconds = system.uptime();
        let hours = uptime_seconds / 3600;
        let minutes = (uptime_seconds % 3600) / 60;
        let days = hours / 24;

        if days > 0 {
            format!("{}d {:02}h {:02}m", days, hours % 24, minutes)
        } else {
            format!("{:02}h {:02}m", hours, minutes)
        }
    }

    fn get_gpu_model() -> String {
        if Self::is_termux() || Self::is_android() {
            // On Android, try to get GPU info from different sources
            if let Ok(output) = Command::new("dumpsys")
                .arg("SurfaceFlinger")
                .output() {
                if let Ok(content) = String::from_utf8(output.stdout) {
                    if let Some(line) = content.lines().find(|line| line.contains("GLES:")) {
                        return line.split("GLES:").nth(1).unwrap_or("Unknown GPU").trim().to_string();
                    }
                }
            }
            
            // Fallback to reading from /proc
            if let Ok(content) = fs::read_to_string("/proc/gpuinfo") {
                if let Some(line) = content.lines().find(|line| line.contains("GPU")) {
                    return line.split(':').nth(1).unwrap_or("Unknown GPU").trim().to_string();
                }
            }
            
            "ARM Mali GPU".to_string()
        } else {
            Command::new("lspci")
                .arg("-nn")
                .output()
                .ok()
                .and_then(|output| {
                    String::from_utf8_lossy(&output.stdout)
                        .lines()
                        .find(|line| line.contains("VGA compatible controller"))
                        .map(|line| {
                            line.split_once(": ")
                                .map(|x| x.1)
                                .unwrap_or("Unknown GPU Model")
                                .trim()
                                .to_string()
                        })
                })
                .unwrap_or_else(|| "Unknown GPU Model".to_string())
        }
    }

    fn get_kernel_version() -> String {
        Command::new("uname")
            .arg("-r")
            .output()
            .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
            .unwrap_or_else(|_| "Unknown Kernel Version".to_string())
    }

    fn get_display_server() -> String {
        if Self::is_termux() || Self::is_android() {
            "Android SurfaceFlinger".to_string()
        } else if std::env::var("WAYLAND_DISPLAY").is_ok() {
            "Wayland".to_string()
        } else if std::env::var("DISPLAY").is_ok() {
            "Xorg".to_string()
        } else {
            "Unknown Display Server".to_string()
        }
    }
}

pub fn get_system_info() -> HashMap<String, String> {
    let mut info = HashMap::new();
    let mut system = System::new_all();
    system.refresh_all();

    info.insert("Username".to_string(), whoami::username().to_string());
    info.insert(
        "Hostname".to_string(),
        whoami::fallible::hostname().unwrap_or_else(|_| "Unknown".to_string()),
    );

    // Detect environment
    let is_termux = SystemInfo::is_termux();
    let is_android = SystemInfo::is_android();
    
    if is_termux {
        info.insert("Environment".to_string(), "Termux (Android)".to_string());
    } else if is_android {
        info.insert("Environment".to_string(), "Android".to_string());
    } else {
        info.insert(
            "Desktop Environment".to_string(),
            std::env::var("XDG_CURRENT_DESKTOP")
                .or_else(|_| std::env::var("DESKTOP_SESSION"))
                .unwrap_or_else(|_| "Unknown".to_string()),
        );
    }

    let used_memory_gib = system.used_memory() as f64 / f64::powf(1024.0, 3.0);
    let total_memory_gib = system.total_memory() as f64 / f64::powf(1024.0, 3.0);
    let memory_percentage = (used_memory_gib / total_memory_gib) * 100.0;

    info.insert(
        "Memory".to_string(),
        format!(
            "{:.2} GiB / {:.2} GiB ({:.0}%)",
            used_memory_gib, total_memory_gib, memory_percentage
        ),
    );

    info.insert("CPU".to_string(), SystemInfo::get_cpu_info());
    info.insert("CPU Model".to_string(), SystemInfo::get_cpu_model());
    info.insert("GPU Model".to_string(), SystemInfo::get_gpu_model());
    info.insert("OS".to_string(), SystemInfo::get_os_info());
    info.insert(
        "Kernel Version".to_string(),
        SystemInfo::get_kernel_version(),
    );
    info.insert(
        "Display Server".to_string(),
        SystemInfo::get_display_server(),
    );
    info.insert("Disk".to_string(), SystemInfo::get_disk_info());
    info.insert("Uptime".to_string(), SystemInfo::get_uptime_pretty(&system));

    info
}
