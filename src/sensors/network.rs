use crate::utils::fs::read_sysfs;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct NetworkInfo {
    pub interface: String,
    pub rx_bytes: u64,
    pub tx_bytes: u64,
    pub rx_packets: u64,
    pub tx_packets: u64,
    pub rx_errors: u64,
    pub tx_errors: u64,
    pub operstate: String,
    pub speed: u64,
    pub mtu: u64,
}

impl Default for NetworkInfo {
    fn default() -> Self {
        Self {
            interface: String::new(),
            rx_bytes: 0, tx_bytes: 0, rx_packets: 0, tx_packets: 0,
            rx_errors: 0, tx_errors: 0,
            operstate: String::new(), speed: 0, mtu: 0,
        }
    }
}

pub struct NetworkSensor;

impl NetworkSensor {
    pub fn list() -> Vec<NetworkInfo> {
        let mut ifaces = vec![];
        let dir = match std::fs::read_dir("/sys/class/net") {
            Ok(d) => d,
            Err(_) => return ifaces,
        };
        for entry in dir.flatten() {
            let name = entry.file_name().into_string().unwrap_or_default();
            if name == "lo" { continue; }
            let mut info = NetworkInfo::default();
            info.interface = name.clone();
            info.operstate = read_sysfs(&format!("/sys/class/net/{}/operstate", name)).unwrap_or_default();
            info.speed = read_sysfs(&format!("/sys/class/net/{}/speed", name))
                .and_then(|s| s.parse().ok()).unwrap_or(0);
            info.mtu = read_sysfs(&format!("/sys/class/net/{}/mtu", name))
                .and_then(|s| s.parse().ok()).unwrap_or(0);
            ifaces.push(info);
        }
        ifaces
    }

    pub fn read_stats(iface: &str) -> NetworkInfo {
        let mut info = NetworkInfo::default();
        info.interface = iface.into();
        info.rx_bytes = read_sysfs(&format!("/sys/class/net/{}/statistics/rx_bytes", iface))
            .and_then(|s| s.parse().ok()).unwrap_or(0);
        info.tx_bytes = read_sysfs(&format!("/sys/class/net/{}/statistics/tx_bytes", iface))
            .and_then(|s| s.parse().ok()).unwrap_or(0);
        info.rx_packets = read_sysfs(&format!("/sys/class/net/{}/statistics/rx_packets", iface))
            .and_then(|s| s.parse().ok()).unwrap_or(0);
        info.tx_packets = read_sysfs(&format!("/sys/class/net/{}/statistics/tx_packets", iface))
            .and_then(|s| s.parse().ok()).unwrap_or(0);
        info.rx_errors = read_sysfs(&format!("/sys/class/net/{}/statistics/rx_errors", iface))
            .and_then(|s| s.parse().ok()).unwrap_or(0);
        info.tx_errors = read_sysfs(&format!("/sys/class/net/{}/statistics/tx_errors", iface))
            .and_then(|s| s.parse().ok()).unwrap_or(0);
        info.operstate = read_sysfs(&format!("/sys/class/net/{}/operstate", iface)).unwrap_or_default();
        info
    }
}
