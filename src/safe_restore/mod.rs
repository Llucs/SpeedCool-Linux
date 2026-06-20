use crate::utils::fs;
use crate::core::engine::ProfileEngine;
use crate::core::profile::Profile;
use std::path::Path;

pub struct SafeRestore;

impl SafeRestore {
    const BOOT_COUNTER: &'static str = "/tmp/speedcool_boot_count";
    const MAX_BOOTS: u32 = 3;
    const WINDOW_SECS: u64 = 300;

    pub fn check_and_recover() -> Result<(), String> {
        let count = Self::boot_count();

        if count >= Self::MAX_BOOTS {
            tracing::warn!("Detected {} rapid boots! Restoring Eco profile.", count);
            let _ = ProfileEngine::apply(&Profile::Eco);
            let _ = std::fs::write(Self::BOOT_COUNTER, "0");
            return Ok(());
        }

        Self::write_boot_count(count + 1);
        Ok(())
    }

    pub fn boot_count() -> u32 {
        match std::fs::read_to_string(Self::BOOT_COUNTER) {
            Ok(s) => s.trim().parse().unwrap_or(0),
            Err(_) => 0,
        }
    }

    fn write_boot_count(count: u32) {
        let _ = std::fs::write(Self::BOOT_COUNTER, count.to_string());
    }

    pub fn mark_stable_boot() {
        let _ = std::fs::write(Self::BOOT_COUNTER, "0");
        tracing::info!("Stable boot confirmed, boot counter reset");
    }
}
