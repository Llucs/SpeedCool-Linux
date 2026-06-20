use crate::utils::fs::{read_sysfs, read_int};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct ThermalInfo {
    pub zone: u32,
    pub temp_c: f64,
    pub kind: String,
}

pub struct ThermalSensor;

impl ThermalSensor {
    pub fn read_all() -> Vec<ThermalInfo> {
        let mut zones = vec![];
        for i in 0..20 {
            let type_path = format!("/sys/class/thermal/thermal_zone{}/type", i);
            let temp_path = format!("/sys/class/thermal/thermal_zone{}/temp", i);
            if let Some(kind) = read_sysfs(&type_path) {
                let temp = read_int(&temp_path).unwrap_or(0) as f64 / 1000.0;
                zones.push(ThermalInfo { zone: i, temp_c: temp, kind });
            }
        }
        zones
    }
}
