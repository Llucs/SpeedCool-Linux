use crate::utils::fs::read_sysfs;
use serde::Serialize;
use std::path::Path;

#[derive(Debug, Clone, Serialize)]
pub struct BatteryInfo {
    pub present: bool,
    pub capacity: f64,
    pub status: String,
    pub health: String,
    pub cycle_count: i64,
    pub voltage_now: u64,
    pub current_now: i64,
    pub power_now: f64,
}

impl Default for BatteryInfo {
    fn default() -> Self {
        Self {
            present: false,
            capacity: 0.0,
            status: "Unknown".into(),
            health: "Unknown".into(),
            cycle_count: 0,
            voltage_now: 0,
            current_now: 0,
            power_now: 0.0,
        }
    }
}

pub struct BatterySensor;

impl BatterySensor {
    fn find_battery_path() -> Option<String> {
        let base = "/sys/class/power_supply";
        let dir = std::fs::read_dir(base).ok()?;
        for entry in dir.flatten() {
            let name = entry.file_name().into_string().ok()?;
            if name.starts_with("BAT") {
                return Some(format!("{}/{}", base, name));
            }
        }
        None
    }

    pub fn read() -> BatteryInfo {
        let mut info = BatteryInfo::default();
        let bat_path = match Self::find_battery_path() {
            Some(p) => p,
            None => return info,
        };

        info.present = true;
        info.capacity = read_sysfs(&format!("{}/capacity", bat_path))
            .and_then(|s| s.parse().ok())
            .unwrap_or(0.0);
        info.status = read_sysfs(&format!("{}/status", bat_path)).unwrap_or_default();
        info.cycle_count = read_sysfs(&format!("{}/cycle_count", bat_path))
            .and_then(|s| s.parse().ok())
            .unwrap_or(0);
        info.voltage_now = read_sysfs(&format!("{}/voltage_now", bat_path))
            .and_then(|s| s.parse().ok())
            .unwrap_or(0);
        info.current_now = read_sysfs(&format!("{}/current_now", bat_path))
            .and_then(|s| s.parse().ok())
            .unwrap_or(0);
        info.power_now = if info.voltage_now > 0 && info.current_now > 0 {
            (info.voltage_now as f64 * info.current_now as f64) / 1_000_000_000_000.0
        } else {
            0.0
        };
        info
    }

    pub fn on_ac() -> bool {
        let status = Self::read().status;
        status == "Charging" || status == "Full"
    }

    pub fn capacity_pct() -> f64 {
        Self::read().capacity
    }
}
