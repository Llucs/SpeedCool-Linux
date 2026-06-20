use crate::ipc::protocol::{IpcRequest, IpcResponse};
use crate::config::paths;
use std::io::{Read, Write};
use std::os::unix::net::UnixStream;
use std::time::Duration;

pub struct IpcClient;

impl IpcClient {
    pub fn send(request: IpcRequest) -> Result<IpcResponse, String> {
        let socket_path = paths::runtime_dir().join("speedcool.sock");
        let mut stream = UnixStream::connect(&socket_path)
            .map_err(|e| format!("Cannot connect to daemon: {}. Is speedcool daemon running?", e))?;
        stream.set_read_timeout(Some(Duration::from_secs(5)))
            .map_err(|e| e.to_string())?;
        stream.set_write_timeout(Some(Duration::from_secs(5)))
            .map_err(|e| e.to_string())?;

        let data = serde_json::to_vec(&request).map_err(|e| e.to_string())?;
        let len = (data.len() as u32).to_le_bytes();
        stream.write_all(&len).map_err(|e| e.to_string())?;
        stream.write_all(&data).map_err(|e| e.to_string())?;

        let mut len_buf = [0u8; 4];
        stream.read_exact(&mut len_buf).map_err(|e| e.to_string())?;
        let resp_len = u32::from_le_bytes(len_buf) as usize;
        let mut resp_buf = vec![0u8; resp_len];
        stream.read_exact(&mut resp_buf).map_err(|e| e.to_string())?;
        serde_json::from_slice(&resp_buf).map_err(|e| e.to_string())
    }
}
