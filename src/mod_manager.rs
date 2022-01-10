use crate::api;
use crate::config_manager::ConfigManager;
use crate::structs::{DownloadedMod, Prefix};
use anyhow::Result;
use std::fs;
use std::path::{Path, PathBuf};

pub struct ModManager {
    pub active_prefix: Option<Prefix>,
    pub downloaded_mods: Vec<DownloadedMod>,
    pub mod_download_path: PathBuf,
}

impl ModManager {
    pub fn new() -> Result<ModManager> {
        let config_manager = ConfigManager::new()?;
        let active_prefix = config_manager.get_config()?.active_prefix.clone();
        let downloaded_mods = ModManager::get_downloaded_mods()?;
        let mod_download_path = config_manager.get_config()?.mod_download_path.clone();
        return Ok(Self {
            active_prefix,
            downloaded_mods,
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

    pub fn get_downloaded_mods() -> Result<Vec<DownloadedMod>> {
        let file_paths = ModManager::get_files(
            ConfigManager::load_config()?
                .mod_download_path
                .clone()
                .as_path(),
        );
        let mut mods: Vec<DownloadedMod> = Vec::new();
        for file in file_paths {
            match ModManager::handle_file(file.clone()) {
                Ok(addon) => {
                    mods.push(addon);
                }
                Err(e) => {
                    println!("{}", e);
                }
            }
        }
        return Ok(mods);
    }

    /// Returns boolean based on whether the mod is already downloaded or not.
    pub fn is_downloaded(&self, id: i32, version: String) -> bool {
        let mods = self.downloaded_mods.clone();
        for r#mod in mods {
            if r#mod.id == id {
                if r#mod.version == version {
                    return true;
                }
            }
        }
        return false;
    }

    pub async fn download_mod(&self, id: i32, version: String) -> Result<DownloadedMod> {
        // Todo: Download mod from backend and add to downloaded_mods
        let mod_info = api::fetch_addon(id).await?;
        let addon = api::download_addon(mod_info, version, self.mod_download_path.clone()).await?;
        return Ok(addon);
    }

    pub async fn update_modlist(&mut self) -> Result<()> {
        // Todo: Update local listing of available mods in api
        let addons = api::fetch_addons().await?;
        println!("{:?}", addons);
        Ok(())
    }

    pub fn get_mod(&self, id: i32, version: String) -> Result<DownloadedMod> {
        for r#mod in self.downloaded_mods.clone() {
            if r#mod.id == id {
                if r#mod.version == version {
                    return Ok(r#mod);
                }
            }
        }
        return Err(anyhow::anyhow!("DownloadedMod not found"));
    }

    pub async fn search_mod(name: &str) -> Result<Vec<DownloadedMod>> {
        // TODO: Search for mods on the local modlist and return them
        let mods = Vec::new();
        return Ok(mods);
    }

    pub fn fetch_mod(id: i32, version: String) -> Result<DownloadedMod> {
        // Todo: Check local modlist and return the mod if it exists
        Ok(DownloadedMod {
            id,
            version,
            name: "".to_string(),
            file_name: "".to_string(),
            file_path: None,
        })
    }

    fn handle_file(file: PathBuf) -> Result<DownloadedMod> {
        let name = file.file_name().unwrap().to_str().unwrap();
        let mod_id = name.split("-").collect::<Vec<&str>>()[0].parse::<i32>()?;
        let mod_version = name.split("-").collect::<Vec<&str>>()[1];
        Ok(ModManager::fetch_mod(mod_id, mod_version.to_string())?)
    }
}
