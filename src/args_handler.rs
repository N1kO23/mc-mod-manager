impl ArgsHandler {
    pub fn new() -> Result<ArgsHandler> {
        let args = ArgsHandler::get_args();
        return Ok(ArgsHandler { args });
    }

    pub fn get_args() -> Vec<String> {
        let args: Vec<String> = std::env::args().collect();
        return args;
    }

    pub fn execute_next(&self) -> Result<()> {
        let command = self.args[1].clone();
        self.args.remove(1);
        match command.as_str() {
            "install" => {
                let mod_id = self.args[2].clone();
                let mod_version = self.args[3].clone();
                let mod_id = mod_id.parse::<i32>()?;
                let mod_version = mod_version.parse::<i32>()?;
                let mod_manager = ModManager::new();
                if !mod_manager.is_downloaded(mod_id, mod_version)? {
                    mod_manager.download_mod(mod_id, mod_version).await?;
                }
                let prefix_manager = PrefixManager::new();
                prefix_manager.add_mod_to_prefix(mod_id, mod_version)?;
            }
            "search" => {
                let mod_id = self.args[2].clone();
                let mod_id = mod_id.parse::<i32>()?;
                let mod_manager = ModManager::new();
                println!("{:?}", mod_manager.search_mod(mod_id).await?);
            }
            "update" => {
                let mod_manager = ModManager::new();
                mod_manager.update_modlist().await?;
                println!("Modlist updated successfully!");
            }
            "list" => {
                let mod_manager = ModManager::new();
                println!("{:?}", mod_manager.get_downloaded_mods());
            }
            "help" => {
                help();
            }
            _ => {
                println!("{}", "Invalid command");
                help();
            }
        }
        return Ok(());
    }

    fn help(&self) {
        println!("{}", "Help");
        println!("{}", "install <mod_id> <mod_version> - Installs the specified version of the specified mod from curseforge");
        println!("{}", "update - Updates the local list of available mods");
        println!(
            "{}",
            "list - Lists all the mods that are currently downloaded and available"
        );
        println!(
            "{}",
            "search <mod_id> - Searches for the specified mod on curseforge"
        );
        println!("{}", "help - Displays this help message and exits program");
        std::process::exit(0);
    }
}
