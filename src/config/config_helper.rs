use once_cell::sync::OnceCell;
use std::sync::RwLock;
use super::config::Config;

// Global static config
static CONFIG: OnceCell<RwLock<Config>> = OnceCell::new();

/// Init the global config
pub fn init_config() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::load()?;

    // Set the global config
    CONFIG.set(RwLock::new(config))
        .map_err(|_| "config_helper --> Config already initialized".into())
}

pub fn get_config() -> Result<impl std::ops::Deref<Target = Config>, Box<dyn std::error::Error>> {
    let lock = CONFIG.get()
        .ok_or_else(|| "config_helper --> Config not initialized".to_string())?
        .read()
        .map_err(|_| "config_helper --> Failed to acquire read lock on config".to_string())?;

    Ok(lock)
}

pub fn get_config_mut() -> Result<impl std::ops::DerefMut<Target = Config>, Box<dyn std::error::Error>> {
    let lock = CONFIG.get()
        .ok_or_else(|| "config_helper --> Config not initialized".to_string())?
        .write()
        .map_err(|_| "config_helper --> Failed to acquire write lock on config".to_string())?;

    Ok(lock)
}

/// Save the current config
pub fn save_config() -> Result<(), Box<dyn std::error::Error>> {
    match get_config_mut() {
        Ok(config) => {
            if let Err(e) = config.save() {
                eprintln!("Failed to save config: {e}");
                return Err(e);
            }
        }
        Err(e) => {
            eprintln!("Failed to lock config: {e}");
            return Err(e);
        }
    }
    Ok(())
}
