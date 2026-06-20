use crate::config::{self, types::SpeedCoolConfig};
use crate::core::{engine::ProfileEngine, profile::Profile};
use crate::sensors::{cpu::CpuSensor, battery::BatterySensor};
use crate::safe_restore::SafeRestore;
use crate::auto_update::AutoUpdate;
use crate::ipc::{protocol::{IpcRequest, IpcResponse}, server::IpcServer};
use std::sync::{Arc, Mutex};
use std::sync::mpsc;
use std::time::Duration;

pub struct Daemon {
    config: SpeedCoolConfig,
    current_profile: Arc<Mutex<Profile>>,
}

impl Daemon {
    pub fn new() -> Self {
        let config = config::parser::load_config();
        Self {
            current_profile: Arc::new(Mutex::new(Profile::Balanced)),
            config,
        }
    }

    pub fn run(&self) -> Result<(), String> {
        tracing::info!("SpeedCool daemon starting...");

        SafeRestore::check_and_recover()?;
        let profile = Profile::Balanced;
        let _ = ProfileEngine::apply(&profile);
        *self.current_profile.lock().unwrap() = profile;

        let profiles = Arc::clone(&self.current_profile);
        let handler: Box<dyn Fn(IpcRequest) -> IpcResponse + Send + Sync> = Box::new(move |req| {
            match req {
                IpcRequest::GetStatus => {
                    let cpu = CpuSensor::read(0);
                    let bat = BatterySensor::read();
                    let profile_val = profiles.lock().unwrap();
                    let data = serde_json::json!({
                        "profile": profile_val.as_str(),
                        "cpu": {
                            "temp": cpu.temp,
                            "usage": cpu.usage,
                            "governor": cpu.governor,
                            "freq_mhz": cpu.frequencies.first().unwrap_or(&0) / 1000,
                            "turbo": cpu.turbo_enabled,
                        },
                        "battery": {
                            "capacity": bat.capacity,
                            "status": bat.status,
                            "on_ac": bat.status == "Charging" || bat.status == "Full",
                        },
                    });
                    IpcResponse::ok_with_data("Status OK", data)
                }
                IpcRequest::SetProfile(name) => {
                    match Profile::from_str(&name) {
                        Some(p) => {
                            match ProfileEngine::apply(&p) {
                                Ok(()) => {
                                    let mut profile = profiles.lock().unwrap();
                                    *profile = p;
                                    IpcResponse::ok(&format!("Profile set to {}", name))
                                }
                                Err(e) => IpcResponse::err(&format!("Failed to set profile: {}", e)),
                            }
                        }
                        None => IpcResponse::err(&format!("Invalid profile: {}", name)),
                    }
                }
                IpcRequest::GetProfile => {
                    let p = profiles.lock().unwrap();
                    IpcResponse::ok_with_data("OK", serde_json::json!({"profile": p.as_str()}))
                }
                IpcRequest::GetConfig => {
                    let config = config::parser::load_config();
                    IpcResponse::ok_with_data("OK", serde_json::to_value(&config).unwrap_or_default())
                }
                IpcRequest::ReloadConfig => {
                    IpcResponse::ok("Config reloaded")
                }
                IpcRequest::Shutdown => {
                    std::process::exit(0);
                }
                _ => IpcResponse::err("Unknown command"),
            }
        });

        let ipc_server = IpcServer::start(handler)?;
        let (tx, rx) = mpsc::channel();

        ctrlc::set_handler(move || {
            let _ = tx.send(());
        }).map_err(|e| format!("Failed to set Ctrl+C handler: {}", e))?;

        tracing::info!("SpeedCool daemon running on IPC socket");
        ipc_server.run(rx);

        Ok(())
    }
}
