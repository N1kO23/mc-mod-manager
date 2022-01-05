impl ModManager {
    pub fn new() -> Result<ModManager> {
        let active_prefix = ConfigManager::get_config()?.active_prefix.clone()?;
        let available_mods = get_files(ConfigManager::get_config()?.mod_download_path.clone())?;
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
                    files.append(&mut get_files(&path));
                }
            }
        }
        files
    }

    pub fn get_downloaded_mods(&self) -> Vec<Mod> {
        // Todo: Implement function that returns a list of all the downloaded mods.
    }

    /// Returns boolean based on whether the mod is already downloaded or not.
    pub fn is_downloaded(&self, id: i32, version: i32) -> bool {
        for r#mod in self.available_mods {
            if r#mod.id == id {
                if r#mod.version == version {
                    return true;
                }
            }
        }
        return false;
    }

    pub async fn download_mod(&self, id: i32, version: i32) -> Result<()> {
        // Todo: Download mod from backend and add to available_mods
    }
}
