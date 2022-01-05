use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::path::Path;

impl ConfigManager {
    pub fn new() -> Result<ConfigManager> {
        let config = ConfigManager::load_config()?;
        return Ok(Self { config });
    }

    pub fn save_config(config: Config) -> Result<()> {
        let file = File::create("config.json")?;
        serde_json::to_writer(file, &config)?;
        return Ok(());
    }

    /// Loads the config file from disk or creates a new one if it doesn't exist.
    pub fn load_config() -> Result<Config> {
        if !Path::new("config.json").exists() {
            return Ok(ConfigManager::create_config()?);
        } else {
            let file = File::open("config.json")?;
            return Ok(serde_json::from_reader(file)?);
        }
    }

    pub fn get_config(&mut self) -> Result<Config> {
        if self.config.is_empty() {
            self.config = ConfigManager::load_config()?;
        }
        return Ok(self.config.clone());
    }

    /// Creates a new config file with an empty access token and default backend address.
    pub fn create_config() -> Result<Config> {
        let config = Config {
            minecraft_installation_path: String::new(),
            mod_download_path: String::new(),
        };
        ConfigManager::save_config(config.clone())?;
        return Ok(config);
    }
}
