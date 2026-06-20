use crate::core::profile::Profile;
use std::collections::HashMap;

pub struct AppMonitor {
    rules: HashMap<String, String>,
}

impl AppMonitor {
    pub fn new() -> Self {
        Self { rules: HashMap::new() }
    }

    pub fn set_rule(&mut self, app: &str, profile: &str) {
        self.rules.insert(app.to_lowercase(), profile.to_lowercase());
    }

    pub fn detect_foreground_app() -> Option<String> {
        if let Ok(out) = std::process::Command::new("xdotool")
            .args(["getactivewindow", "getwindowname"])
            .output()
        {
            let name = String::from_utf8_lossy(&out.stdout).trim().to_string();
            if !name.is_empty() {
                return Some(name);
            }
        }

        let env_vars = vec!["XDG_SESSION_DESKTOP", "GDMSESSION", "DESKTOP_SESSION"];
        for var in &env_vars {
            if let Ok(val) = std::env::var(var) {
                return Some(val);
            }
        }

        if let Ok(out) = std::process::Command::new("ps")
            .args(["-eo", "pid,comm", "--sort=-%cpu", "--no-headers"])
            .output()
        {
            let s = String::from_utf8_lossy(&out.stdout);
            if let Some(line) = s.lines().next() {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 2 {
                    return Some(parts[1].to_string());
                }
            }
        }

        None
    }

    pub fn resolve_profile(&self, app_name: &str) -> Option<Profile> {
        let lower = app_name.to_lowercase();
        for (pattern, profile_name) in &self.rules {
            if lower.contains(pattern) {
                return Profile::from_str(profile_name);
            }
        }
        None
    }
}
