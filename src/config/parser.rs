use super::types::SpeedCoolConfig;
use super::paths;
use std::fs;

pub fn load_config() -> SpeedCoolConfig {
    let config_path = paths::config_dir().join("config.toml");
    if config_path.exists() {
        let content = fs::read_to_string(&config_path).unwrap_or_default();
        toml::from_str(&content).unwrap_or_default()
    } else {
        SpeedCoolConfig::default()
    }
}

pub fn save_config(config: &SpeedCoolConfig) -> Result<(), String> {
    let config_path = paths::config_dir().join("config.toml");
    let content = toml::to_string_pretty(config).map_err(|e| e.to_string())?;
    fs::write(&config_path, content).map_err(|e| e.to_string())
}
