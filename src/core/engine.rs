use crate::core::profile::Profile;
use crate::sensors::cpu::CpuSensor;
use crate::utils::fs;

pub struct ProfileEngine;

impl ProfileEngine {
    pub fn apply(profile: &Profile) -> Result<(), String> {
        tracing::info!("Applying profile: {}", profile.as_str());

        CpuSensor::set_governor(profile.governor())?;
        CpuSensor::set_turbo(profile.turbo())?;
        let _ = CpuSensor::set_epp(profile.epp());

        let _ = Self::set_swappiness(profile.swappiness());
        let _ = Self::set_io_scheduler(profile.io_scheduler());

        if let Ok(_) = crate::gpu::GpuSensor::read() {
            let _ = crate::gpu::GpuSensor::set_power_level(profile.gpu_power());
        }

        tracing::info!("Profile {} applied successfully", profile.as_str());
        Ok(())
    }

    fn set_swappiness(val: u8) -> Result<(), String> {
        fs::write_sysfs("/proc/sys/vm/swappiness", &val.to_string())
    }

    fn set_io_scheduler(scheduler: &str) -> Result<(), String> {
        let disks = crate::sensors::disk::DiskSensor::list();
        for disk in &disks {
            let _ = crate::sensors::disk::DiskSensor::set_scheduler(&disk.name, scheduler);
        }
        Ok(())
    }

    pub fn auto_select(cpu_temp: f64, cpu_load: f64, on_ac: bool, battery_pct: f64, thresholds: &crate::config::types::ThresholdsConfig) -> Profile {
        if cpu_temp > thresholds.cpu_temp_critical {
            return Profile::Eco;
        }
        if !on_ac && battery_pct < thresholds.battery_pct_low {
            return Profile::Eco;
        }
        if cpu_load > thresholds.cpu_load_high {
            return Profile::Performance;
        }
        if cpu_load < thresholds.cpu_load_low {
            return Profile::Eco;
        }
        Profile::Balanced
    }
}
