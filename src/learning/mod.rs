use crate::core::engine::ProfileEngine;
use crate::core::profile::Profile;
use crate::sensors::cpu::CpuSensor;
use crate::sensors::battery::BatterySensor;
use std::collections::VecDeque;

#[derive(Debug, Clone)]
struct DataPoint {
    cpu_temp: f64,
    cpu_load: f64,
    battery_pct: f64,
    on_ac: bool,
    hour: u8,
    minute: u8,
}

pub struct LearningEngine {
    history: VecDeque<DataPoint>,
    profile_schedule: Vec<(u8, Profile)>,
}

impl LearningEngine {
    pub fn new() -> Self {
        Self {
            history: VecDeque::with_capacity(10080),
            profile_schedule: vec![],
        }
    }

    pub fn record(&mut self) {
        let cpu = CpuSensor::read(0);
        let bat = BatterySensor::read();
        let now = chrono::Local::now();
        let dp = DataPoint {
            cpu_temp: cpu.temp,
            cpu_load: cpu.usage,
            battery_pct: bat.capacity,
            on_ac: bat.status == "Charging" || bat.status == "Full",
            hour: now.hour() as u8,
            minute: now.minute() as u8,
        };
        self.history.push_back(dp);
        if self.history.len() > 10080 {
            self.history.pop_front();
        }
    }

    pub fn learn(&mut self) {
        if self.history.len() < 1440 {
            return;
        }

        let mut eco_hours = vec![0u32; 24];
        let mut perf_hours = vec![0u32; 24];
        let mut total_hours = vec![0u32; 24];

        for dp in &self.history {
            let h = dp.hour as usize;
            total_hours[h] += 1;
            if dp.cpu_load < 20.0 || (!dp.on_ac && dp.battery_pct < 30.0) {
                eco_hours[h] += 1;
            } else if dp.cpu_load > 60.0 {
                perf_hours[h] += 1;
            }
        }

        self.profile_schedule.clear();
        for h in 0..24 {
            if total_hours[h] > 10 {
                let eco_ratio = eco_hours[h] as f64 / total_hours[h] as f64;
                let perf_ratio = perf_hours[h] as f64 / total_hours[h] as f64;
                if eco_ratio > 0.6 {
                    self.profile_schedule.push((h as u8, Profile::Eco));
                } else if perf_ratio > 0.5 {
                    self.profile_schedule.push((h as u8, Profile::Performance));
                } else {
                    self.profile_schedule.push((h as u8, Profile::Balanced));
                }
            }
        }

        tracing::info!(
            "Learning: analyzed {} data points across {} days",
            self.history.len(),
            self.history.len() / 1440
        );
    }

    pub fn get_scheduled_profile(&self, hour: u8) -> Option<&Profile> {
        let mut best: Option<&Profile> = None;
        let mut min_diff = 24u8;
        for (h, p) in &self.profile_schedule {
            let diff = if *h >= hour { *h - hour } else { 24 + *h - hour };
            if diff < min_diff {
                min_diff = diff;
                best = Some(p);
            }
        }
        best
    }
}
