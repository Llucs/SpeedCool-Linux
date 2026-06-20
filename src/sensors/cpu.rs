use crate::utils::fs::{self, read_int, read_sysfs, cpu_path, cpu_online_list};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct CpuInfo {
    pub model: String,
    pub cores: usize,
    pub frequencies: Vec<u64>,
    pub governor: String,
    pub available_governors: Vec<String>,
    pub turbo_enabled: bool,
    pub temp: f64,
    pub usage: f64,
    pub min_freq: u64,
    pub max_freq: u64,
}

impl Default for CpuInfo {
    fn default() -> Self {
        Self {
            model: String::new(),
            cores: 1,
            frequencies: vec![0],
            governor: "unknown".into(),
            available_governors: vec![],
            turbo_enabled: false,
            temp: 0.0,
            usage: 0.0,
            min_freq: 0,
            max_freq: 0,
        }
    }
}

pub struct CpuSensor;

impl CpuSensor {
    pub fn read(cpu: u32) -> CpuInfo {
        let mut info = CpuInfo::default();
        info.cores = fs::cpu_online_count();
        info.frequencies = vec![0; info.cores];

        for i in 0..info.cores {
            info.frequencies[i] = read_int(&cpu_path(i as u32, "scaling_cur_freq")).unwrap_or(0) as u64;
        }

        info.governor = read_sysfs(&cpu_path(cpu, "scaling_governor")).unwrap_or_default();
        info.min_freq = read_int(&cpu_path(cpu, "scaling_min_freq")).unwrap_or(0) as u64;
        info.max_freq = read_int(&cpu_path(cpu, "scaling_max_freq")).unwrap_or(0) as u64;

        if let Some(govs) = read_sysfs(&cpu_path(cpu, "scaling_available_governors")) {
            info.available_governors = govs.split_whitespace().map(String::from).collect();
        }

        info.turbo_enabled = read_int("/sys/devices/system/cpu/intel_pstate/no_turbo")
            .map(|v| v == 0)
            .or_else(|| {
                read_int("/sys/devices/system/cpu/cpufreq/boost")
                    .map(|v| v == 1)
            })
            .unwrap_or(true);

        info.temp = Self::read_temperature();
        info.usage = Self::read_usage();
        info.model = read_sysfs("/proc/cpuinfo")
            .and_then(|s| {
                s.lines()
                    .find(|l| l.starts_with("model name"))
                    .map(|l| {
                        let parts: Vec<&str> = l.split(':').collect();
                        if parts.len() > 1 {
                            parts[1].trim().to_string()
                        } else {
                            "Unknown".into()
                        }
                    })
            })
            .unwrap_or_else(|| "Unknown CPU".into());

        info
    }

    pub fn read_temp_for_core(core: u32) -> f64 {
        let path = format!("/sys/devices/system/cpu/cpu{}/thermal/temp", core);
        read_int(&path).map(|v| v as f64 / 1000.0).unwrap_or(0.0)
    }

    pub fn read_temperature() -> f64 {
        for zone in 0..20 {
            let path = format!("/sys/class/thermal/thermal_zone{}/temp", zone);
            if let Ok(s) = std::fs::read_to_string(&path) {
                let s = s.trim();
                if let Ok(temp) = s.parse::<f64>() {
                    return temp / 1000.0;
                }
            }
        }
        0.0
    }

    pub fn read_usage() -> f64 {
        let stat = read_sysfs("/proc/stat").unwrap_or_default();
        let line = stat.lines().next().unwrap_or("");
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 5 {
            return 0.0;
        }
        let user: u64 = parts.get(1).unwrap_or(&"0").parse().unwrap_or(0);
        let nice: u64 = parts.get(2).unwrap_or(&"0").parse().unwrap_or(0);
        let system: u64 = parts.get(3).unwrap_or(&"0").parse().unwrap_or(0);
        let idle: u64 = parts.get(4).unwrap_or(&"0").parse().unwrap_or(0);
        let total = user + nice + system + idle;
        if total == 0 {
            return 0.0;
        }
        let idle_delta = idle as f64;
        100.0 * (1.0 - idle_delta / total as f64)
    }

    pub fn set_governor(governor: &str) -> Result<(), String> {
        for cpu in cpu_online_list() {
            let path = cpu_path(cpu, "scaling_governor");
            fs::write_sysfs(&path, governor)?;
        }
        Ok(())
    }

    pub fn set_turbo(enabled: bool) -> Result<(), String> {
        let val = if enabled { "0" } else { "1" };
        let paths = [
            "/sys/devices/system/cpu/intel_pstate/no_turbo",
            "/sys/devices/system/cpu/cpufreq/boost",
        ];
        for path in &paths {
            if std::path::Path::new(path).exists() {
                fs::write_sysfs(path, val)?;
            }
        }
        Ok(())
    }

    pub fn set_epp(epp: &str) -> Result<(), String> {
        for cpu in cpu_online_list() {
            let path = format!(
                "/sys/devices/system/cpu/cpu{}/cpufreq/energy_performance_preference",
                cpu
            );
            if std::path::Path::new(&path).exists() {
                fs::write_sysfs(&path, epp)?;
            }
        }
        Ok(())
    }
}
