pub struct PluginManager;

impl PluginManager {
    pub fn load_wasm(path: &str) -> Result<(), String> {
        Err("WASM plugin support not yet implemented".into())
    }

    pub fn load_native(path: &str) -> Result<(), String> {
        Err("Native plugin support not yet implemented".into())
    }
}
