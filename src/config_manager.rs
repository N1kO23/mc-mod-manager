use crate::prefix_manager::PrefixManager;
use crate::structs::Config;
use anyhow::Result;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::str::FromStr;

pub struct ConfigManager {
    pub config: Config,
}

impl ConfigManager {
    pub fn new() -> Result<ConfigManager> {
        let config = ConfigManager::load_config()?;
        return Ok(Self { config });
    }

    pub fn save_config(config: &Config) -> Result<()> {
        let file = File::create("config.json")?;
        serde_json::to_writer(file, config)?;
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

    pub fn get_config(&self) -> Result<Config> {
        return Ok(self.config.clone());
    }

    /// Creates a new config file with an empty access token and default backend address.
    pub fn create_config() -> Result<Config> {
        PrefixManager::create_prefix("default", "Default Author")?;
        let config = Config {
            minecraft_installation_path: PathBuf::new(),
            mod_download_path: PathBuf::from_str("./downloads")?,
            active_prefix: "default".to_string(),
        };
        ConfigManager::save_config(&config)?;
        return Ok(config);
    }
}
