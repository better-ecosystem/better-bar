
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Write;
use std::path::{PathBuf};
use toml;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PanelConfig {
    pub position: String,
    pub height: u32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ModulesConfig {
    pub clock: bool,
    pub cpu: bool,
    pub memory: bool,
    pub battery: bool,
    pub network: bool,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Config {
    pub panel: PanelConfig,
    pub modules: ModulesConfig,
}

impl Config {
    pub fn default() -> Self {
        Self {
            panel: PanelConfig {
                position: "top".to_string(),
                height: 30,
            },
            modules: ModulesConfig {
                clock: true,
                cpu: true,
                memory: true,
                battery: true,
                network: true,
            },
        }
    }

    pub fn get_config_path() -> PathBuf {
        let home = dirs::home_dir().expect("Could not find home directory");
        home.join(".config").join("better-bar").join("config.toml")
    }

    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let config_path = Self::get_config_path();

        // Check if the config file exists
        if !config_path.exists() {
            // Create dir if not exists
            if let Some(parent) = config_path.parent() {
                fs::create_dir_all(parent)?;
            }

            // Create a default config
            let default_config = Self::default();

            let toml_content = toml::to_string_pretty(&default_config)?;

            // Write to config
            let mut file = fs::File::create(&config_path)?;
            file.write_all(toml_content.as_bytes())?;

            return Ok(default_config);
        }

        // Read and parse the existing config file
        let content = fs::read_to_string(config_path)?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let config_path = Self::get_config_path();

        // Ensure directory exists
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)?;
        }

        // Serialize to TOML
        let toml_content = toml::to_string_pretty(self)?;

        // Write to config
        let mut file = fs::File::create(&config_path)?;
        file.write_all(toml_content.as_bytes())?;

        Ok(())
    }
}