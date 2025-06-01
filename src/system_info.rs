// src/system_info.rs

use std::process::Command;
use std::collections::HashMap;
use whoami;
use sysinfo::{System, SystemExt, DiskExt};
use std::fs;

pub struct SystemInfo
{
    pub os: String,
    pub cpu: String,
    pub memory: String,
    pub disk: String,
}

impl SystemInfo
{
    pub fn new() -> Self
    {
        SystemInfo
        {
            os: Self::get_os_info(),
            cpu: Self::get_cpu_info(),
            memory: Self::get_memory_info(),
            disk: Self::get_disk_info(),
        }
    }

    fn get_os_info() -> String
    {
        if cfg!(target_os = "linux")
        {
            Command::new("uname")
                .arg("-o")
                .output()
                .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
                .unwrap_or_else(|_| "Unknown OS".to_string())
        }
        else if cfg!(target_os = "macos")
        {
            Command::new("sw_vers")
                .arg("-productName")
                .output()
                .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
                .unwrap_or_else(|_| "Unknown OS".to_string())
        }
        else
        {
            "Unsupported OS".to_string()
        }
    }

    fn get_cpu_info() -> String
    {
        Command::new("lscpu")
            .output()
            .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
            .unwrap_or_else(|_| "Unknown CPU".to_string())
    }

    fn get_memory_info() -> String
    {
        Command::new("free")
            .arg("-h")
            .output()
            .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
            .unwrap_or_else(|_| "Unknown Memory".to_string())
    }

    fn get_disk_info() -> String
    {
        let mut system = System::new_all();
        system.refresh_disks_list();

        let mut disk_info = String::new();

        for disk in system.disks()
        {
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

        if disk_info.is_empty()
        {
            "No disks found".to_string()
        }
        else
        {
            disk_info.trim_end().to_string() // Remove trailing newline
        }
    }

    fn get_cpu_model() -> String
    {
        fs::read_to_string("/proc/cpuinfo")
            .ok()
            .and_then(|content|
            {
                content
                    .lines()
                    .find(|line| line.starts_with("model name"))
                    .map(|line| line.split(':').nth(1).unwrap_or("").trim().to_string())
            })
            .unwrap_or_else(|| "Unknown CPU Model".to_string())
    }

    pub fn get_uptime_pretty(system: &System) -> String
    {
        let uptime_seconds = system.uptime();
        let hours = uptime_seconds / 3600;
        let minutes = (uptime_seconds % 3600) / 60;
        let days = hours / 24;
        
        if days > 0
        {
            format!("{}d {:02}h {:02}m", days, hours % 24, minutes)
        }
        else
        {
            format!("{:02}h {:02}m", hours, minutes)
        }
    }

    fn get_gpu_model() -> String
    {
        Command::new("lspci")
            .arg("-nn")
            .output()
            .ok()
            .and_then(|output|
            {
                String::from_utf8_lossy(&output.stdout)
                    .lines()
                    .find(|line| line.contains("VGA compatible controller"))
                    .map(|line|
                    {
                        line.splitn(2, ": ")
                            .nth(1)
                            .unwrap_or("Unknown GPU Model")
                            .trim()
                            .to_string()
                    })
            })
            .unwrap_or_else(|| "Unknown GPU Model".to_string())
    }
    
    fn get_kernel_version() -> String
    {
        Command::new("uname")
            .arg("-r")
            .output()
            .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
            .unwrap_or_else(|_| "Unknown Kernel Version".to_string())
    }

    fn get_display_server() -> String
    {
        if std::env::var("WAYLAND_DISPLAY").is_ok()
        {
            "Wayland".to_string()
        }
        else if std::env::var("DISPLAY").is_ok()
        {
            "Xorg".to_string()
        }
        else
        {
            "Unknown Display Server".to_string()
        }
    }
}

pub fn get_system_info() -> HashMap<String, String>
{
    let mut info = HashMap::new();
    let mut system = System::new_all();
    system.refresh_all();

    info.insert("Username".to_string(), whoami::username().to_string());
    info.insert
    (
        "Hostname".to_string(),
        whoami::fallible::hostname().unwrap_or_else(|_| "Unknown".to_string()),
    );

    info.insert
    (
        "Desktop Environment".to_string(),
        std::env::var("XDG_CURRENT_DESKTOP")
            .or_else(|_| std::env::var("DESKTOP_SESSION"))
            .unwrap_or_else(|_| "Unknown".to_string()),
    );

    let used_memory_gib = system.used_memory() as f64 / f64::powf(1024.0, 3.0);
    let total_memory_gib = system.total_memory() as f64 / f64::powf(1024.0, 3.0);
    let memory_percentage = (used_memory_gib / total_memory_gib) * 100.0;

    info.insert
    (
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
    info.insert("Kernel Version".to_string(), SystemInfo::get_kernel_version());
    info.insert("Display Server".to_string(), SystemInfo::get_display_server());
    info.insert("Disk".to_string(), SystemInfo::get_disk_info());
    info.insert("Uptime".to_string(), SystemInfo::get_uptime_pretty(&system));

    info
}
