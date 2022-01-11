use crate::api;
use crate::config_manager::ConfigManager;
use crate::prefix_manager::PrefixManager;
use crate::structs::{Addon, DownloadedMod, Prefix};
use crate::util::{string_to_array, subvec};
use anyhow::Result;
use std::fs::{self, File};
use std::path::{Path, PathBuf};

pub struct ModManager {
    pub active_prefix: Prefix,
    pub downloaded_mods: Vec<DownloadedMod>,
    pub mod_download_path: PathBuf,
}

impl ModManager {
    pub fn new() -> Result<ModManager> {
        let config_manager = ConfigManager::new()?;
        let config = config_manager.get_config()?;
        let active_prefix = PrefixManager::load_prefix(&config.active_prefix.clone())?;
        let downloaded_mods = ModManager::get_downloaded_mods()?;
        let mod_download_path = config.mod_download_path.clone();
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
                Err(_) => {}
            }
        }
        return Ok(mods);
    }

    /// Returns boolean based on whether the mod is already downloaded or not.
    pub fn is_downloaded(&self, id: i32, version: String) -> bool {
        let mods = self.downloaded_mods.clone();
        for r#mod in mods {
            if r#mod.id == id {
                if r#mod.version.contains(&version) {
                    return true;
                }
            }
        }
        return false;
    }

    pub async fn download_mod(
        &mut self,
        id: i32,
        version: String,
        modloader: String,
    ) -> Result<DownloadedMod> {
        // Todo: Download mod from backend and add to downloaded_mods
        let mod_info = api::fetch_addon(id).await?;
        let addon = api::download_addon(
            &mod_info,
            version,
            self.mod_download_path.clone(),
            modloader,
        )
        .await?;
        self.downloaded_mods = ModManager::get_downloaded_mods()?;
        return Ok(addon);
    }

    pub fn get_mod(&self, id: i32, version: &String) -> Result<DownloadedMod> {
        for r#mod in self.downloaded_mods.clone() {
            if r#mod.id == id {
                if r#mod.version.contains(version) {
                    return Ok(r#mod);
                }
            }
        }
        return Err(anyhow::anyhow!("DownloadedMod not found"));
    }

    pub async fn search_mod(name: &str) -> Result<Vec<Addon>> {
        // TODO: Search for mods on the local modlist and return them
        // let modlist = ModManager::load_modlist()?;
        let modlist = api::search_addon(name).await?;
        let mut mods = Vec::new();
        let name = name.to_lowercase();
        for r#mod in modlist {
            if r#mod.name.to_lowercase().contains(&name) {
                mods.push(r#mod);
            }
        }
        mods.reverse();
        return Ok(mods);
    }

    pub fn fetch_mod(id: i32) -> Result<Addon> {
        return Ok(futures::executor::block_on(api::fetch_addon(id))?);
    }

    fn handle_file(file: PathBuf) -> Result<DownloadedMod> {
        let name = file.file_name().unwrap().to_str().unwrap();
        let filename_array = string_to_array(name, ";");
        if filename_array.len() < 4 {
            return Err(anyhow::anyhow!("Invalid filename"));
        }
        let mod_id = filename_array[0].parse::<i32>()?;
        let mod_name = filename_array[1].clone();
        let file_name = filename_array[2].clone();
        let mc_version = subvec(&filename_array, 3, filename_array.len());
        let downloaded_mod = DownloadedMod {
            id: mod_id,
            name: mod_name.to_string(),
            version: mc_version,
            file_path: Some(file.clone()),
            file_name: file_name.to_string(),
        };
        Ok(downloaded_mod)
    }
}
