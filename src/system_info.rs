// src/system_info.rs

use std::process::Command;
use std::collections::HashMap;
use whoami;
use sysinfo::{System, SystemExt, DiskExt};
use std::fs;

pub struct SystemInfo {
    pub os: String,
    pub cpu: String,
    pub memory: String,
    pub disk: String,
}

impl SystemInfo {
    pub fn new() -> Self {
        SystemInfo {
            os: Self::get_os_info(),
            cpu: Self::get_cpu_info(),
            memory: Self::get_memory_info(),
            disk: Self::get_disk_info(),
        }
    }

    fn get_os_info() -> String {
        if cfg!(target_os = "linux") {
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
        Command::new("lscpu")
            .output()
            .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
            .unwrap_or_else(|_| "Unknown CPU".to_string())
    }

    fn get_memory_info() -> String {
        Command::new("free")
            .arg("-h")
            .output()
            .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
            .unwrap_or_else(|_| "Unknown Memory".to_string())
    }

    fn get_disk_info() -> String {
        let mut system = System::new_all();
        system.refresh_disks_list();

        let mut disk_info = String::new();

        for disk in system.disks() {
            let total_space = disk.total_space() as f64 / 1024.0 / 1024.0 / 1024.0; // Convert to GiB
            let available_space = disk.available_space() as f64 / 1024.0 / 1024.0 / 1024.0; // Convert to GiB
            let used_space = total_space - available_space;
            let used_percentage = (used_space / total_space) * 100.0;

            let mount_point = disk.mount_point().to_string_lossy();
            let file_system = String::from_utf8_lossy(disk.file_system());

            disk_info.push_str(&format!(
                "Disk ({}): {:.2} GiB / {:.2} GiB ({:.0}%) - {}\n",
                mount_point,
                used_space,
                total_space,
                used_percentage,
                file_system
            ));
        }

        if disk_info.is_empty() {
            "No disks found".to_string()
        } else {
            disk_info.trim_end().to_string() // Remove trailing newline
        }
    }

    // Add function to get the CPU model
    fn get_cpu_model() -> String {
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

    // Add function to get the GPU model
    fn get_gpu_model() -> String {
        Command::new("lspci")
            .arg("-nn")
            .output()
            .ok()
            .and_then(|output| {
                String::from_utf8_lossy(&output.stdout)
                    .lines()
                    .find(|line| line.contains("VGA compatible controller"))
                    .and_then(|line| {
                        // Extract the part before the PCI ID and revision
                        line.split('[')
                            .next() // Take the part before the first '['
                            .map(|part| part.split(':').last().unwrap_or("").trim().to_string())
                    })
            })
            .unwrap_or_else(|| "Unknown GPU Model".to_string())
    }

    // Add function to get the kernel version
    fn get_kernel_version() -> String {
        Command::new("uname")
            .arg("-r")
            .output()
            .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
            .unwrap_or_else(|_| "Unknown Kernel Version".to_string())
    }

    // Add function to check if Xorg or Wayland is in use
    fn get_display_server() -> String {
        if std::env::var("WAYLAND_DISPLAY").is_ok() {
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

    info.insert("OS".to_string(), whoami::platform().to_string());
    info.insert("Username".to_string(), whoami::username().to_string());
    info.insert(
        "Hostname".to_string(),
        whoami::fallible::hostname().unwrap_or_else(|_| "Unknown".to_string()),
    );

    info.insert(
        "Desktop Environment".to_string(),
        std::env::var("XDG_CURRENT_DESKTOP")
            .or_else(|_| std::env::var("DESKTOP_SESSION"))
            .unwrap_or_else(|_| "Unknown".to_string()),
    );

    let used_memory_gib = system.used_memory() as f64 / 1024.0 / 1024.0 / 1024.0;
    let total_memory_gib = system.total_memory() as f64 / 1024.0 / 1024.0 / 1024.0;
    let memory_percentage = (used_memory_gib / total_memory_gib) * 100.0;

    info.insert(
        "Memory".to_string(),
        format!(
            "{:.2} GiB / {:.2} GiB ({:.0}%)",
            used_memory_gib, total_memory_gib, memory_percentage
        ),
    );

    let disk_info = SystemInfo::get_disk_info();
    info.insert("Disks".to_string(), disk_info);

    // Add new entries
    info.insert("CPU Model".to_string(), SystemInfo::get_cpu_model());
    info.insert("GPU Model".to_string(), SystemInfo::get_gpu_model());
    info.insert("Kernel Version".to_string(), SystemInfo::get_kernel_version());
    info.insert("Display Server".to_string(), SystemInfo::get_display_server());

    info
}