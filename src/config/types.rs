use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpeedCoolConfig {
    pub daemon: DaemonConfig,
    pub profiles: ProfilesConfig,
    pub thresholds: ThresholdsConfig,
    pub apps: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DaemonConfig {
    pub poll_interval_ms: u64,
    pub log_level: String,
    pub auto_update: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfilesConfig {
    pub eco: EcoConfig,
    pub balanced: BalancedConfig,
    pub performance: PerformanceConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EcoConfig {
    pub cpu_governor: String,
    pub turbo_enabled: bool,
    pub epp: String,
    pub swappiness: u8,
    pub io_scheduler: String,
    pub gpu_power_level: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BalancedConfig {
    pub cpu_governor: String,
    pub turbo_enabled: bool,
    pub epp: String,
    pub swappiness: u8,
    pub io_scheduler: String,
    pub gpu_power_level: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceConfig {
    pub cpu_governor: String,
    pub turbo_enabled: bool,
    pub epp: String,
    pub swappiness: u8,
    pub io_scheduler: String,
    pub gpu_power_level: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThresholdsConfig {
    pub cpu_temp_critical: f64,
    pub cpu_temp_warn: f64,
    pub gpu_temp_critical: f64,
    pub battery_pct_low: f64,
    pub cpu_load_high: f64,
    pub cpu_load_low: f64,
}

impl Default for SpeedCoolConfig {
    fn default() -> Self {
        Self {
            daemon: DaemonConfig {
                poll_interval_ms: 2000,
                log_level: "info".into(),
                auto_update: true,
            },
            profiles: ProfilesConfig {
                eco: EcoConfig {
                    cpu_governor: "powersave".into(),
                    turbo_enabled: false,
                    epp: "power".into(),
                    swappiness: 10,
                    io_scheduler: "kyber".into(),
                    gpu_power_level: "low".into(),
                },
                balanced: BalancedConfig {
                    cpu_governor: "schedutil".into(),
                    turbo_enabled: true,
                    epp: "balance_performance".into(),
                    swappiness: 60,
                    io_scheduler: "mq-deadline".into(),
                    gpu_power_level: "auto".into(),
                },
                performance: PerformanceConfig {
                    cpu_governor: "performance".into(),
                    turbo_enabled: true,
                    epp: "performance".into(),
                    swappiness: 10,
                    io_scheduler: "none".into(),
                    gpu_power_level: "high".into(),
                },
            },
            thresholds: ThresholdsConfig {
                cpu_temp_critical: 85.0,
                cpu_temp_warn: 70.0,
                gpu_temp_critical: 85.0,
                battery_pct_low: 20.0,
                cpu_load_high: 80.0,
                cpu_load_low: 20.0,
            },
            apps: HashMap::new(),
        }
    }
}
