use crate::config;

pub struct AutoUpdate;

impl AutoUpdate {
    pub fn check_for_updates(current_version: &str) -> Result<Option<String>, String> {
        let url = "https://api.github.com/repos/Llucs/SpeedCool-Linux/releases/latest";
        let client = reqwest::blocking::Client::builder()
            .user_agent("SpeedCool-Linux/1.0")
            .build()
            .map_err(|e| e.to_string())?;
        let resp = client.get(url).send().map_err(|e| e.to_string())?;
        let json: serde_json::Value = resp.json().map_err(|e| e.to_string())?;
        let latest = json["tag_name"].as_str().unwrap_or("v0.0.0");
        let latest_clean = latest.trim_start_matches('v');
        let current_clean = current_version.trim_start_matches('v');

        let latest_sem = semver::Version::parse(latest_clean).unwrap_or(semver::Version::new(0, 0, 0));
        let current_sem = semver::Version::parse(current_clean).unwrap_or(semver::Version::new(0, 0, 0));

        if latest_sem > current_sem {
            Ok(Some(latest.to_string()))
        } else {
            Ok(None)
        }
    }

    pub fn do_update(version: &str) -> Result<(), String> {
        let url = format!(
            "https://github.com/Llucs/SpeedCool-Linux/releases/download/{}/speedcool-linux-x86_64.tar.gz",
            version
        );
        let client = reqwest::blocking::Client::builder()
            .user_agent("SpeedCool-Linux/1.0")
            .build()
            .map_err(|e| e.to_string())?;
        let resp = client.get(&url).send().map_err(|e| e.to_string())?;
        let bytes = resp.bytes().map_err(|e| e.to_string())?;

        let temp_dir = "/tmp/speedcool-update";
        std::fs::create_dir_all(temp_dir).map_err(|e| e.to_string())?;
        let tar_path = format!("{}/speedcool.tar.gz", temp_dir);
        std::fs::write(&tar_path, &bytes).map_err(|e| e.to_string())?;

        let _ = std::process::Command::new("tar")
            .args(["-xzf", &tar_path, "-C", temp_dir])
            .output()
            .map_err(|e| e.to_string())?;

        let _ = std::process::Command::new("cp")
            .args([format!("{}/speedcool", temp_dir), "/usr/local/bin/speedcool"])
            .output();

        let _ = std::process::Command::new("cp")
            .args([format!("{}/speedcool.service", temp_dir), "/etc/systemd/system/"])
            .output();

        let _ = std::fs::remove_dir_all(temp_dir);
        let _ = std::process::Command::new("systemctl").args(["daemon-reload"]).output();
        tracing::info!("Updated to version {}", version);
        Ok(())
    }
}
