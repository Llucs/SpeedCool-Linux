use crate::core::profile::Profile;
use crate::core::engine::ProfileEngine;
use std::process::Command;

pub struct GamingMode {
    gamemode_available: bool,
    previous_profile: Option<Profile>,
}

impl GamingMode {
    pub fn new() -> Self {
        Self {
            gamemode_available: crate::utils::fs::has_gamemode(),
            previous_profile: None,
        }
    }

    pub fn detect_game() -> Option<String> {
        let game_processes = vec![
            "steam", "steamwebhelper", "hl2_linux", "cs2", "dota2",
            "lutris", "heroic", "wine", "wine64", "wine-preloader",
            "gamescope", "mangohud", "vkbasalt",
        ];

        if let Ok(out) = Command::new("ps")
            .args(["-eo", "comm", "--no-headers"])
            .output()
        {
            let s = String::from_utf8_lossy(&out.stdout);
            for proc_name in &game_processes {
                if s.contains(proc_name) {
                    return Some(proc_name.to_string());
                }
            }
        }

        None
    }

    pub fn enter_gaming_mode(&mut self) -> Result<(), String> {
        self.previous_profile = Some(Profile::Balanced);
        let _ = ProfileEngine::apply(&Profile::Performance);

        if self.gamemode_available {
            let _ = Command::new("gamemoderun")
                .arg("echo SpeedCool Gaming Mode")
                .output();
        }

        tracing::info!("Gaming mode activated");
        Ok(())
    }

    pub fn exit_gaming_mode(&mut self) -> Result<(), String> {
        if let Some(profile) = self.previous_profile.take() {
            let _ = ProfileEngine::apply(&profile);
        }
        tracing::info!("Gaming mode deactivated");
        Ok(())
    }

    pub fn is_available() -> bool {
        crate::utils::fs::has_gamemode()
    }
}
