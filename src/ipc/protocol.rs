use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum IpcRequest {
    GetStatus,
    SetProfile(String),
    GetProfile,
    GetConfig,
    ReloadConfig,
    RunDaemon,
    Shutdown,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpcResponse {
    pub success: bool,
    pub message: String,
    pub data: Option<serde_json::Value>,
}

impl IpcResponse {
    pub fn ok(msg: &str) -> Self {
        Self { success: true, message: msg.into(), data: None }
    }
    pub fn ok_with_data(msg: &str, data: serde_json::Value) -> Self {
        Self { success: true, message: msg.into(), data: Some(data) }
    }
    pub fn err(msg: &str) -> Self {
        Self { success: false, message: msg.into(), data: None }
    }
}
