use std::path::PathBuf;
use std::env;

use lazy_static::lazy_static;

use crate::ui::logger::{LogLevel, Logger};

lazy_static! {
    static ref LOG: Logger = Logger::new(LogLevel::Debug);
}

pub fn get_primary_css_path() -> PathBuf {
    if let Ok(home_dir) = env::var("HOME") {
        let mut user_css_path = PathBuf::from(home_dir);
        user_css_path.push(".config/better-bar/style.css");
        
        if user_css_path.exists() {
            LOG.debug("config -> found custom users config at ~/.config/better-bar/style.css");
            return user_css_path;
        }
    }
    
    // Fallback to default css
    let mut default_css_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    LOG.debug("config -> using default css");
    default_css_path.push("src/ui/style/default.css");
    default_css_path
}

// pub fn get_css_paths() -> Vec<PathBuf> {
    // let mut css_paths = Vec::new();
    // 
    // let mut default_css_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    // default_css_path.push("src/ui/style/default.css");
    // css_paths.push(default_css_path);
    // 
    // if let Ok(home_dir) = env::var("HOME") {
        // let mut user_css_path = PathBuf::from(home_dir);
        // user_css_path.push(".config/better-bar/style.css");
        // 
        // if user_css_path.exists() {
            // css_paths.push(user_css_path);
        // }
    // }
    // 
    // css_paths
// }