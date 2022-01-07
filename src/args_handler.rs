use crate::mod_manager::ModManager;
use crate::prefix_manager::PrefixManager;
use anyhow::Result;
pub struct ArgsHandler {
    pub args: Vec<String>,
}

impl ArgsHandler {
    pub fn new() -> Result<ArgsHandler> {
        let args = ArgsHandler::get_args();
        return Ok(Self { args });
    }

    pub fn get_args() -> Vec<String> {
        let args: Vec<String> = std::env::args().collect();
        return args;
    }

    pub async fn execute_next(&mut self) -> Result<()> {
        let command = self.args[1].clone();
        self.args.remove(1);
        match command.as_str() {
            "install" => {
                let mod_id = self.args[2].clone();
                let mod_version = self.args[3].clone();
                let mod_id = mod_id.parse::<i32>()?;
                let mod_version = mod_version.parse::<i32>()?;
                let mod_manager = ModManager::new()?;
                if !mod_manager.is_downloaded(mod_id, mod_version) {
                    mod_manager.download_mod(mod_id, mod_version).await?;
                }
                let mut prefix_manager = PrefixManager::new()?;
                prefix_manager
                    .add_mod_to_prefix(mod_id, mod_version)
                    .await?;
            }
            "search" => {
                let mod_name = self.args[2].clone();
                println!("{:?}", ModManager::search_mod(&mod_name).await?);
            }
            "update" => {
                let mut mod_manager = ModManager::new()?;
                mod_manager.update_modlist().await?;
                println!("Modlist updated successfully!");
            }
            "list" => {
                println!("{:?}", ModManager::get_downloaded_mods());
            }
            "help" => {
                self.help();
            }
            _ => {
                println!("{}", "Invalid command");
                self.help();
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
