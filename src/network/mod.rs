use crate::utils::fs;

pub struct NetworkTuner;

impl NetworkTuner {
    pub fn set_tcp_buffers(rmem: &str, wmem: &str) -> Result<(), String> {
        fs::write_sysfs("/proc/sys/net/ipv4/tcp_rmem", rmem)?;
        fs::write_sysfs("/proc/sys/net/ipv4/tcp_wmem", wmem)
    }

    pub fn set_congestion_control(cc: &str) -> Result<(), String> {
        fs::write_sysfs("/proc/sys/net/ipv4/tcp_congestion_control", cc)
    }

    pub fn enable_bbr() -> Result<(), String> {
        fs::write_sysfs("/proc/sys/net/ipv4/tcp_congestion_control", "bbr")?;
        fs::write_sysfs("/proc/sys/net/core/default_qdisc", "fq")
    }

    pub fn set_txqueuelen(iface: &str, len: u32) -> Result<(), String> {
        let _ = std::process::Command::new("ip")
            .args(["link", "set", "dev", iface, "txqueuelen", &len.to_string()])
            .output()
            .map_err(|e| e.to_string())?;
        Ok(())
    }
}
