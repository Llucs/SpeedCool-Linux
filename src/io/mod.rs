use crate::utils::fs;

pub struct IOTuner;

impl IOTuner {
    pub fn set_swappiness(val: u8) -> Result<(), String> {
        fs::write_sysfs("/proc/sys/vm/swappiness", &val.to_string())
    }

    pub fn set_vfs_cache_pressure(val: u8) -> Result<(), String> {
        fs::write_sysfs("/proc/sys/vm/vfs_cache_pressure", &val.to_string())
    }

    pub fn set_readahead(disk: &str, kb: u32) -> Result<(), String> {
        let path = format!("/sys/block/{}/queue/read_ahead_kb", disk);
        fs::write_sysfs(&path, &kb.to_string())
    }

    pub fn trim(disk: &str) -> Result<(), String> {
        let path = format!("/sys/block/{}/device/scsi_disk/*/manage_start_stop", disk);
        fs::write_sysfs(&path, "1")
    }

    pub fn setup_zram(size_mb: u32) -> Result<(), String> {
        let _ = fs::write_sysfs("/sys/module/zram/parameters/num_devices", "1");
        let devices = std::fs::read_dir("/sys/class/block").map_err(|e| e.to_string())?;
        for dev in devices.flatten() {
            let name = dev.file_name().into_string().unwrap_or_default();
            if name.starts_with("zram") {
                let path = format!("/sys/block/{}/disksize", name);
                fs::write_sysfs(&path, &format!("{}M", size_mb))?;
                let _ = std::process::Command::new("mkswap")
                    .arg(format!("/dev/{}", name)).output();
                let _ = std::process::Command::new("swapon")
                    .arg(format!("/dev/{}", name)).output();
                return Ok(());
            }
        }
        Err("No zram device found".into())
    }
}
