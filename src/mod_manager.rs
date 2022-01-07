use crate::api;
use crate::config_manager::ConfigManager;
use crate::structs::{Mod, Prefix};
use anyhow::Result;
use std::fs;
use std::path::{Path, PathBuf};

pub struct ModManager {
    pub active_prefix: Prefix,
    pub available_mods: Vec<Mod>,
    pub mod_download_path: PathBuf,
}

impl ModManager {
    pub fn new() -> Result<ModManager> {
        let config_manager = ConfigManager::new()?;
        let active_prefix = config_manager.get_config()?.active_prefix.clone().unwrap();
        let available_mods = ModManager::get_downloaded_mods()?;
        let mod_download_path = config_manager.get_config()?.mod_download_path.clone();
        return Ok(Self {
            active_prefix,
            available_mods,
            mod_download_path,
        });
    }

    /// Fetches all the files in the directory and subdirectories.
    pub fn get_files(dir: &Path) -> Vec<PathBuf> {
        let mut files: Vec<PathBuf> = Vec::new();

        if dir.is_dir() {
            for entry in fs::read_dir(dir).expect("Unable to read directory") {
                let entry = entry.expect("Unable to read entry");
                let path = entry.path();

                if path.is_file() {
                    files.push(path);
                } else if path.is_dir() {
                    files.append(&mut ModManager::get_files(&path));
                }
            }
        }
        files
    }

    pub fn get_downloaded_mods() -> Result<Vec<Mod>> {
        let file_paths = ModManager::get_files(
            ConfigManager::load_config()?
                .mod_download_path
                .clone()
                .as_path(),
        );
        let mut mods: Vec<Mod> = Vec::new();
        return Ok(mods);
    }

    /// Returns boolean based on whether the mod is already downloaded or not.
    pub fn is_downloaded(&self, id: i32, version: i32) -> bool {
        let mods = self.available_mods.clone();
        for r#mod in mods {
            if r#mod.id == id {
                if r#mod.version == version {
                    return true;
                }
            }
        }
        return false;
    }

    pub async fn download_mod(&self, id: i32, version: i32) -> Result<Mod> {
        // Todo: Download mod from backend and add to available_mods
        let addon = api::download_addon(id, version, self.mod_download_path.clone()).await?;
        return Ok(addon);
    }

    pub async fn update_modlist(&mut self) -> Result<()> {
        // Todo: Update local listing of available mods in api
        Ok(())
    }

    pub fn get_mod(&self, id: i32, version: i32) -> Result<Mod> {
        for r#mod in self.available_mods.clone() {
            if r#mod.id == id {
                if r#mod.version == version {
                    return Ok(r#mod);
                }
            }
        }
        return Err(anyhow::anyhow!("Mod not found"));
    }

    pub async fn search_mod(name: &str) -> Result<Vec<Mod>> {
        // TODO: Search for mods on the local modlist and return them
        let mods = Vec::new();
        return Ok(mods);
    }
}
