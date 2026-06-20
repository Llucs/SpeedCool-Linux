pub struct DesktopIntegration;

impl DesktopIntegration {
    pub fn notify(title: &str, body: &str) -> Result<(), String> {
        let _ = std::process::Command::new("notify-send")
            .args([title, body])
            .output();
        Ok(())
    }
}
