// src/system_info.rs

use std::process::Command;
use std::collections::HashMap;
use whoami;
use sysinfo::{System, SystemExt, DiskExt};

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

    // Add Desktop Environment
    info.insert(
        "Desktop Environment".to_string(),
        std::env::var("XDG_CURRENT_DESKTOP").unwrap_or_else(|_| "Unknown".to_string()),
    );

    // Convert RAM usage from KB to GiB and calculate percentage
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

    // Add disk information
    let disk_info = SystemInfo::get_disk_info();
    info.insert("Disks".to_string(), disk_info);

    info
}