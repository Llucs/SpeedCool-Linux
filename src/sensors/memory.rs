use crate::utils::fs::read_sysfs;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct MemoryInfo {
    pub total_kb: u64,
    pub free_kb: u64,
    pub available_kb: u64,
    pub cached_kb: u64,
    pub swap_total_kb: u64,
    pub swap_free_kb: u64,
}

impl Default for MemoryInfo {
    fn default() -> Self {
        Self {
            total_kb: 0,
            free_kb: 0,
            available_kb: 0,
            cached_kb: 0,
            swap_total_kb: 0,
            swap_free_kb: 0,
        }
    }
}

pub struct MemorySensor;

impl MemorySensor {
    pub fn read() -> MemoryInfo {
        let content = read_sysfs("/proc/meminfo").unwrap_or_default();
        let mut info = MemoryInfo::default();

        for line in content.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() < 2 {
                continue;
            }
            let key = parts[0].trim_end_matches(':');
            let val = parts[1].parse::<u64>().unwrap_or(0);
            match key {
                "MemTotal" => info.total_kb = val,
                "MemFree" => info.free_kb = val,
                "MemAvailable" => info.available_kb = val,
                "Cached" => info.cached_kb = val,
                "SwapTotal" => info.swap_total_kb = val,
                "SwapFree" => info.swap_free_kb = val,
                _ => {}
            }
        }
        info
    }

    pub fn usage_pct() -> f64 {
        let info = Self::read();
        if info.total_kb == 0 {
            return 0.0;
        }
        let used = info.total_kb - info.available_kb;
        (used as f64 / info.total_kb as f64) * 100.0
    }
}
