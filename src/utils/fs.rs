use std::fs;
use std::path::Path;

pub fn read_sysfs(path: &str) -> Option<String> {
    fs::read_to_string(path).ok().map(|s| s.trim().to_string())
}

pub fn write_sysfs(path: &str, value: &str) -> Result<(), String> {
    fs::write(path, value).map_err(|e| format!("Failed to write {}: {}", path, e))
}

pub fn read_int(path: &str) -> Option<i64> {
    read_sysfs(path).and_then(|s| s.parse().ok())
}

pub fn cpu_path(cpu: u32, file: &str) -> String {
    format!("/sys/devices/system/cpu/cpu{}/cpufreq/{}", cpu, file)
}

pub fn cpu_online_count() -> usize {
    match read_sysfs("/sys/devices/system/cpu/present") {
        Some(s) => {
            if s.contains('-') {
                let parts: Vec<&str> = s.split('-').collect();
                if parts.len() == 2 {
                    let end: usize = parts[1].parse().unwrap_or(0);
                    end + 1
                } else {
                    num_cpus()
                }
            } else {
                1
            }
        }
        None => num_cpus(),
    }
}

fn num_cpus() -> usize {
    std::thread::available_parallelism().map(|n| n.get()).unwrap_or(1)
}

pub fn cpu_online_list() -> Vec<u32> {
    (0..cpu_online_count()).collect()
}

pub fn detect_distro() -> String {
    for path in &["/etc/os-release", "/usr/lib/os-release"] {
        if let Ok(content) = fs::read_to_string(path) {
            for line in content.lines() {
                if line.starts_with("ID=") {
                    return line[3..].trim_matches('"').to_string();
                }
            }
        }
    }
    "linux".into()
}

pub fn has_lm_sensors() -> bool {
    Path::new("/usr/bin/sensors").exists() || Path::new("/usr/sbin/sensors").exists()
}

pub fn has_nvidia_smi() -> bool {
    Path::new("/usr/bin/nvidia-smi").exists()
}

pub fn has_gamemode() -> bool {
    Path::new("/usr/bin/gamemoderun").exists()
}
