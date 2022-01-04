use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::path::Path;

use crate::structs::Prefix;

impl PrefixManager {
    pub fn new() -> Result<PrefixManager> {
        let prefix = PrefixManager::load_prefix()?;
        return Ok(Self { prefix });
    }

    pub fn save_prefix(prefix: Prefix) -> Result<()> {
        let file = File::create("prefix.json")?;
        serde_json::to_writer(file, &prefix)?;
        return Ok(());
    }

    /// Loads the prefix file from disk or throws an error if it doesn't exist.
    pub fn load_prefix(name: &str, version: &str) -> Result<Prefix> {
        let prefix_file = format!("{}-{}.json", name, version);
        if !Path::new(prefix_file).exists() {
            return Err("The prefix file does not exist".into());
        } else {
            let file = File::open("./prefixes/prefix.json")?;
            return Ok(serde_json::from_reader(file)?);
        }
    }

    /// Creates a new prefix file with an empty access token and default backend address.
    pub fn create_prefix(name: &str, author: &str) -> Result<Prefix> {
        let prefix = Prefix {
            name: name,
            description: String::new(),
            author: author,
            version: "0.0.1".to_string(),
            mod_list: Vec::new(),
        };
        prefixManager::save_prefix(prefix.clone())?;
        return Ok(prefix);
    }
}
