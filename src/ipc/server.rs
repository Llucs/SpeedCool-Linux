use crate::ipc::protocol::{IpcRequest, IpcResponse};
use crate::config::paths;
use std::io::{Read, Write};
use std::os::unix::net::{UnixListener, UnixStream};
use std::path::Path;
use std::sync::mpsc;

pub type RequestHandler = Box<dyn Fn(IpcRequest) -> IpcResponse + Send + Sync>;

pub struct IpcServer {
    listener: UnixListener,
    handler: RequestHandler,
}

impl IpcServer {
    pub fn start(handler: RequestHandler) -> Result<Self, String> {
        let socket_path = paths::runtime_dir().join("speedcool.sock");
        if socket_path.exists() {
            std::fs::remove_file(&socket_path).map_err(|e| e.to_string())?;
        }
        std::fs::create_dir_all(paths::runtime_dir()).map_err(|e| e.to_string())?;

        let listener = UnixListener::bind(&socket_path).map_err(|e| e.to_string())?;
        Ok(Self { listener, handler })
    }

    pub fn run(&self, cancel: mpsc::Receiver<()>) {
        for stream in self.listener.incoming() {
            if cancel.try_recv().is_ok() {
                break;
            }
            match stream {
                Ok(stream) => self.handle_connection(stream),
                Err(e) => tracing::error!("IPC connection error: {}", e),
            }
        }
    }

    fn handle_connection(&self, mut stream: UnixStream) {
        let mut len_buf = [0u8; 4];
        if stream.read_exact(&mut len_buf).is_err() {
            return;
        }
        let req_len = u32::from_le_bytes(len_buf) as usize;
        let mut req_buf = vec![0u8; req_len];
        if stream.read_exact(&mut req_buf).is_err() {
            return;
        }
        let request: IpcRequest = match serde_json::from_slice(&req_buf) {
            Ok(r) => r,
            Err(e) => {
                let resp = IpcResponse::err(&format!("Invalid request: {}", e));
                let _ = self.send_response(&mut stream, resp);
                return;
            }
        };
        let response = (self.handler)(request);
        let _ = self.send_response(&mut stream, response);
    }

    fn send_response(&self, stream: &mut UnixStream, response: IpcResponse) -> Result<(), String> {
        let data = serde_json::to_vec(&response).map_err(|e| e.to_string())?;
        let len = (data.len() as u32).to_le_bytes();
        stream.write_all(&len).map_err(|e| e.to_string())?;
        stream.write_all(&data).map_err(|e| e.to_string())
    }
}
