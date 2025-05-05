// src/system_info.rs

use std::process::Command;
use std::collections::HashMap;
use whoami;
use sysinfo::{System, SystemExt};

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
        Command::new("df")
            .arg("-h")
            .output()
            .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
            .unwrap_or_else(|_| "Unknown Disk".to_string())
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

    info.insert(
        "Desktop Environment".to_string(),
        std::env::var("XDG_CURRENT_DESKTOP").unwrap_or_else(|_| "Unknown".to_string()),
    );

    info
}