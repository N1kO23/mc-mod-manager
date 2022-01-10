use crate::config_manager::ConfigManager;
use crate::mod_manager::ModManager;
use anyhow::Result;
use std::fs::File;
use std::path::Path;

use crate::structs::Prefix;

pub struct PrefixManager {
    pub prefix: Prefix,
}

impl PrefixManager {
    pub fn new() -> Result<PrefixManager> {
        let active_prefix = ConfigManager::load_config()?.active_prefix;
        let prefix = PrefixManager::load_prefix(&active_prefix)?;
        let prefix_manager = PrefixManager { prefix };
        return Ok(prefix_manager);
    }

    /// Saves the prefix to the prefix file
    pub fn save_prefix(prefix: Prefix) -> Result<()> {
        let file = File::create(format!("prefix-{}.json", prefix.name.clone()))?;
        serde_json::to_writer(file, &prefix)?;
        return Ok(());
    }

    /// Loads the prefix file from disk or throws an error if it doesn't exist.
    pub fn load_prefix(name: &str) -> Result<Prefix> {
        let path_string = format!("prefix-{}.json", name);
        let prefix_file = Path::new(&path_string);
        if !prefix_file.exists() {
            return Err(anyhow::anyhow!("Prefix file does not exist"));
        } else {
            let file = File::open(prefix_file)?;
            return Ok(serde_json::from_reader(file)?);
        }
    }

    /// Creates a new prefix file with an empty access token and default backend address.
    pub fn create_prefix(name: &str, author: &str) -> Result<Prefix> {
        let prefix = Prefix {
            name: name.to_string(),
            description: String::new(),
            author: author.to_string(),
            version: "0.0.1".to_string(),
            game_version: "1.18.1".to_string(),
            mod_list: Vec::new(),
            mod_loader: "Forge".to_string(),
        };
        PrefixManager::save_prefix(prefix.clone())?;
        return Ok(prefix);
    }

    pub async fn add_mod_to_prefix(&mut self, id: i32, version: String) -> Result<()> {
        let mod_manager = ModManager::new()?;
        if !mod_manager.is_downloaded(id, version.clone()) {
            mod_manager
                .download_mod(id, version.clone(), self.prefix.mod_loader.clone())
                .await?;
        }
        let addon = mod_manager.get_mod(id, version)?;
        self.prefix.mod_list.push(addon.id);
        PrefixManager::save_prefix(self.prefix.clone())?;
        return Ok(());
    }

    pub fn remove_mod_from_prefix(&mut self, id: i32) -> Result<()> {
        // TODO: Implement function that removes the mod from the prefix
        PrefixManager::save_prefix(self.prefix.clone())?;
        return Ok(());
    }
}
