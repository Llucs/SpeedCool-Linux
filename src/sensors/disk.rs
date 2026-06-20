use crate::utils::fs::{read_sysfs, read_int};
use serde::Serialize;
use std::path::Path;

#[derive(Debug, Clone, Serialize)]
pub struct DiskInfo {
    pub name: String,
    pub scheduler: String,
    pub ro: bool,
    pub size_sectors: u64,
    pub model: String,
}

impl Default for DiskInfo {
    fn default() -> Self {
        Self { name: String::new(), scheduler: String::new(), ro: false, size_sectors: 0, model: String::new() }
    }
}

pub struct DiskSensor;

impl DiskSensor {
    pub fn list() -> Vec<DiskInfo> {
        let mut disks = vec![];
        let dir = match std::fs::read_dir("/sys/block") {
            Ok(d) => d,
            Err(_) => return disks,
        };
        for entry in dir.flatten() {
            let name = entry.file_name().into_string().unwrap_or_default();
            if name.starts_with("loop") || name.starts_with("ram") || name.starts_with("zram") {
                continue;
            }
            let base = format!("/sys/block/{}", name);
            let mut info = DiskInfo::default();
            info.name = name;
            info.scheduler = read_sysfs(&format!("{}/queue/scheduler", base))
                .map(|s| {
                    s.split_whitespace()
                        .find(|p| p.starts_with('['))
                        .unwrap_or("")
                        .trim_matches(|c| c == '[' || c == ']')
                        .to_string()
                })
                .unwrap_or_default();
            info.ro = read_sysfs(&format!("{}/ro", base))
                .and_then(|s| s.parse::<u8>().ok())
                .map(|v| v == 1)
                .unwrap_or(false);
            info.size_sectors = read_sysfs(&format!("{}/size", base))
                .and_then(|s| s.parse().ok())
                .unwrap_or(0);
            info.model = read_sysfs(&format!("{}/device/model", base)).unwrap_or_default();
            disks.push(info);
        }
        disks
    }

    pub fn set_scheduler(disk: &str, scheduler: &str) -> Result<(), String> {
        let path = format!("/sys/block/{}/queue/scheduler", disk);
        if Path::new(&path).exists() {
            crate::utils::fs::write_sysfs(&path, scheduler)
        } else {
            Err(format!("Disk {} not found", disk))
        }
    }

    pub fn trim(disk: &str) -> Result<(), String> {
        let path = format!("/sys/block/{}/device/queue_depth", disk);
        if Path::new(&path).exists() {
            let trim_path = format!("/sys/block/{}/device/scsi_disk/*/manage_start_stop", disk);
            crate::utils::fs::write_sysfs(&trim_path, "1")
        } else {
            Err("TRIM not supported on this disk".into())
        }
    }
}
