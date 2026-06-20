use crate::core::profile::Profile;
use crate::sensors::{cpu::CpuSensor, memory::MemorySensor, battery::BatterySensor, disk::DiskSensor, thermal::ThermalSensor};
use crate::gpu::GpuSensor;
use crate::utils::fs;
use std::time::Instant;

#[derive(Debug, Clone, PartialEq)]
pub enum AppScreen {
    Main,
    Cpu,
    Memory,
    Gpu,
    Disk,
    Thermal,
    Battery,
}

pub struct App {
    pub screen: AppScreen,
    pub cpu_info: crate::sensors::cpu::CpuInfo,
    pub mem_info: crate::sensors::memory::MemoryInfo,
    pub bat_info: crate::sensors::battery::BatteryInfo,
    pub disks: Vec<crate::sensors::disk::DiskInfo>,
    pub gpu_info: crate::gpu::GpuInfo,
    pub thermal_zones: Vec<crate::sensors::thermal::ThermalInfo>,
    pub current_profile: Profile,
    pub should_quit: bool,
    pub last_poll: Instant,
    pub poll_interval: std::time::Duration,
    pub cpu_history: Vec<f64>,
    pub temp_history: Vec<f64>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            screen: AppScreen::Main,
            cpu_info: CpuSensor::read(0),
            mem_info: MemorySensor::read(),
            bat_info: BatterySensor::read(),
            disks: DiskSensor::list(),
            gpu_info: GpuSensor::read(),
            thermal_zones: ThermalSensor::read_all(),
            current_profile: Profile::Balanced,
            should_quit: false,
            last_poll: Instant::now(),
            poll_interval: std::time::Duration::from_secs(2),
            cpu_history: Vec::with_capacity(60),
            temp_history: Vec::with_capacity(60),
        }
    }
}

impl App {
    pub fn new() -> Self {
        let mut app = Self::default();
        app.poll();
        app
    }

    pub fn poll(&mut self) {
        self.cpu_info = CpuSensor::read(0);
        self.mem_info = MemorySensor::read();
        self.bat_info = BatterySensor::read();
        self.disks = DiskSensor::list();
        self.gpu_info = GpuSensor::read();
        self.thermal_zones = ThermalSensor::read_all();

        self.cpu_history.push(self.cpu_info.usage);
        if self.cpu_history.len() > 60 {
            self.cpu_history.remove(0);
        }
        self.temp_history.push(self.cpu_info.temp);
        if self.temp_history.len() > 60 {
            self.temp_history.remove(0);
        }
        self.last_poll = Instant::now();
    }

    pub fn mem_used_gb(&self) -> f64 {
        if self.mem_info.total_kb == 0 { return 0.0; }
        let used = self.mem_info.total_kb - self.mem_info.available_kb;
        used as f64 / (1024.0 * 1024.0)
    }

    pub fn mem_total_gb(&self) -> f64 {
        self.mem_info.total_kb as f64 / (1024.0 * 1024.0)
    }

    pub fn mem_pct(&self) -> f64 {
        if self.mem_info.total_kb == 0 { return 0.0; }
        let used = self.mem_info.total_kb - self.mem_info.available_kb;
        (used as f64 / self.mem_info.total_kb as f64) * 100.0
    }

    pub fn battery_pct(&self) -> f64 {
        self.bat_info.capacity
    }

    pub fn on_ac(&self) -> bool {
        self.bat_info.status == "Charging" || self.bat_info.status == "Full"
    }

    pub fn distro_name(&self) -> String {
        fs::detect_distro()
    }
}
