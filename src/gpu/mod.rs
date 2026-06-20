use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct GpuInfo {
    pub vendor: String,
    pub model: String,
    pub temp_c: f64,
    pub usage_pct: f64,
    pub memory_total_mb: u64,
    pub memory_used_mb: u64,
    pub core_clock_mhz: u64,
    pub memory_clock_mhz: u64,
    pub power_watts: f64,
}

impl Default for GpuInfo {
    fn default() -> Self {
        Self {
            vendor: "Unknown".into(),
            model: "Unknown".into(),
            temp_c: 0.0, usage_pct: 0.0,
            memory_total_mb: 0, memory_used_mb: 0,
            core_clock_mhz: 0, memory_clock_mhz: 0,
            power_watts: 0.0,
        }
    }
}

pub struct GpuSensor;

impl GpuSensor {
    pub fn read() -> GpuInfo {
        let mut info = GpuInfo::default();
        if let Some(nv) = Self::read_nvidia() {
            return nv;
        }
        if let Some(amd) = Self::read_amd() {
            return amd;
        }
        if let Some(intel) = Self::read_intel() {
            return intel;
        }
        info
    }

    fn read_nvidia() -> Option<GpuInfo> {
        let path = "/proc/driver/nvidia/gpus/0/information";
        let content = std::fs::read_to_string(path).ok()?;
        let mut info = GpuInfo::default();
        info.vendor = "NVIDIA".into();
        for line in content.lines() {
            if line.contains("Model") {
                info.model = line.split(':').nth(1)?.trim().to_string();
            }
        }
        if let Ok(out) = std::process::Command::new("nvidia-smi")
            .args(["--query-gpu=temperature.gpu,utilization.gpu,memory.total,memory.used,clocks.current.graphics,clocks.current.memory,power.draw"])
            .args(["--format=csv,noheader,nounits"])
            .output()
        {
            let s = String::from_utf8_lossy(&out.stdout);
            let parts: Vec<&str> = s.trim().split(',').collect();
            if parts.len() >= 7 {
                info.temp_c = parts[0].trim().parse().unwrap_or(0.0);
                info.usage_pct = parts[1].trim().parse().unwrap_or(0.0);
                info.memory_total_mb = parts[2].trim().parse().unwrap_or(0);
                info.memory_used_mb = parts[3].trim().parse().unwrap_or(0);
                info.core_clock_mhz = parts[4].trim().parse().unwrap_or(0);
                info.memory_clock_mhz = parts[5].trim().parse().unwrap_or(0);
                info.power_watts = parts[6].trim().parse().unwrap_or(0.0);
            }
        }
        Some(info)
    }

    fn read_amd() -> Option<GpuInfo> {
        let base = "/sys/class/drm";
        let dir = std::fs::read_dir(base).ok()?;
        for entry in dir.flatten() {
            let name = entry.file_name().into_string().ok()?;
            if !name.contains("card") || name.contains("-") {
                continue;
            }
            let dev_dir = format!("{}/{}", base, name);
            let mut info = GpuInfo::default();
            info.vendor = "AMD".into();

            let vbios = std::fs::read_to_string(format!("{}/device/vbios_version", dev_dir)).ok()?;
            info.model = vbios.trim().to_string();

            let temp_path = format!("{}/device/hwmon/hwmon*/temp1_input", dev_dir);
            if let Ok(glob) = glob::glob(&temp_path) {
                for p in glob.flatten() {
                    if let Ok(t) = std::fs::read_to_string(p) {
                        info.temp_c = t.trim().parse::<f64>().unwrap_or(0.0) / 1000.0;
                    }
                }
            }
            return Some(info);
        }
        None
    }

    fn read_intel() -> Option<GpuInfo> {
        let base = "/sys/class/drm";
        let dir = std::fs::read_dir(base).ok()?;
        for entry in dir.flatten() {
            let name = entry.file_name().into_string().ok()?;
            if !name.starts_with("card") || !name.contains("i915") {
                continue;
            }
            let mut info = GpuInfo::default();
            info.vendor = "Intel".into();
            info.model = "Intel Integrated Graphics".into();

            let temp_path = format!("{}/{}/device/hwmon/hwmon*/temp1_input", base, name);
            if let Ok(glob) = glob::glob(&temp_path) {
                for p in glob.flatten() {
                    if let Ok(t) = std::fs::read_to_string(p) {
                        info.temp_c = t.trim().parse::<f64>().unwrap_or(0.0) / 1000.0;
                    }
                }
            }
            return Some(info);
        }
        None
    }

    pub fn set_power_level(level: &str) -> Result<(), String> {
        if let Ok(_) = Self::read_nvidia() {
            let val = match level {
                "low" => "1",
                "auto" => "2",
                "high" => "0",
                _ => return Err("Invalid power level".into()),
            };
            crate::utils::fs::write_sysfs(
                "/sys/devices/pci0000:00/.../power/control",
                if level == "high" { "on" } else { "auto" },
            )
        } else {
            Ok(())
        }
    }
}
