use crate::config_manager::ConfigManager;
use crate::mod_manager::ModManager;
use crate::util::read_line;
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

    /// Loads all prefixes from disk into array and returns it.
    pub fn load_all_prefixes() -> Result<Vec<Prefix>> {
        let mut prefixes: Vec<Prefix> = Vec::new();
        // TODO: Specify the path to folder that holds prefixes in config
        let prefix_files = std::fs::read_dir(".")?;
        for prefix_file in prefix_files {
            let prefix_file = prefix_file?;
            let prefix_file_name = prefix_file.file_name();
            let prefix_file_name = prefix_file_name.to_str().unwrap();
            if prefix_file_name.starts_with("prefix-") && prefix_file_name.ends_with(".json") {
                let prefix_file_name = prefix_file_name.replace("prefix-", "").replace(".json", "");
                match PrefixManager::load_prefix(&prefix_file_name) {
                    Ok(prefix) => {
                        prefixes.push(prefix);
                    }
                    Err(e) => {
                        println!("Error loading prefix {}: {}", &prefix_file_name, e);
                    }
                }
            }
        }
        return Ok(prefixes);
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
        let addon = mod_manager.get_mod(id, &version)?;
        if self.prefix.mod_list.contains(&addon.id) {
            return Err(anyhow::anyhow!("Mod already exists in prefix"));
        }
        self.prefix.mod_list.push(addon.id);
        // Todo: Implement copying the mod file into the minecraft mods folder if this is the active prefix
        PrefixManager::save_prefix(self.prefix.clone())?;
        return Ok(());
    }

    pub fn remove_mod_from_prefix(&mut self, id: i32) -> Result<()> {
        // TODO: Implement function that removes the mod from the prefix and from the minecraft mods folder if this is the active prefix
        PrefixManager::save_prefix(self.prefix.clone())?;
        return Ok(());
    }

    pub fn set_prefix(name: &str) -> Result<Prefix> {
        let prefix = PrefixManager::load_prefix(name)?;
        let mut config = ConfigManager::load_config()?;
        config.active_prefix = name.to_string();
        ConfigManager::save_config(&config)?;
        println!("Prefix set to {}", name);
        return Ok(prefix);
    }

    pub fn new_prefix_form(name: &str) -> Result<Prefix> {
        print!("{}", "Enter the author of the prefix (default: 'author'): ");
        let author = read_line().unwrap_or("author".to_string());
        print!("{}", "Enter the description of the prefix (default: ''): ");
        let description = read_line().unwrap_or("".to_string());
        print!("{}", "Enter the version of the prefix (default: '0.0.1'): ");
        let version = read_line().unwrap_or("0.0.1".to_string());
        print!(
            "{}",
            "Enter the game version of the prefix (default: '1.18.1'): "
        );
        let game_version = read_line().unwrap_or("1.18.1".to_string());
        print!(
            "{}",
            "Enter the mod loader of the prefix (default: 'Forge'): "
        );
        let mod_loader = read_line().unwrap_or("Forge".to_string());
        let prefix = Prefix {
            name: name.to_string(),
            description,
            author,
            version,
            game_version,
            mod_list: Vec::new(),
            mod_loader,
        };
        println!("{}", "Prefix created!");
        PrefixManager::save_prefix(prefix.clone())?;
        return Ok(prefix);
    }
}
