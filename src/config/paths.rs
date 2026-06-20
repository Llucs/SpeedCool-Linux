use dirs;
use std::path::PathBuf;

pub fn config_dir() -> PathBuf {
    PathBuf::from("/etc/speedcool")
}

pub fn data_dir() -> PathBuf {
    PathBuf::from("/var/lib/speedcool")
}

pub fn runtime_dir() -> PathBuf {
    PathBuf::from("/run/speedcool")
}

pub fn log_dir() -> PathBuf {
    PathBuf::from("/var/log/speedcool")
}

pub fn default_config() -> &'static str {
    include_str!("../../config/default.toml")
}
