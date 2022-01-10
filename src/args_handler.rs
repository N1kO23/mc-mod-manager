use crate::mod_manager::ModManager;
use crate::prefix_manager::PrefixManager;
use anyhow::Result;
pub struct ArgsHandler {
    pub args: Vec<String>,
    index: usize,
}

impl ArgsHandler {
    pub fn new() -> Result<ArgsHandler> {
        let args = ArgsHandler::get_args();
        return Ok(Self { args, index: 1 });
    }

    pub fn get_args() -> Vec<String> {
        let args: Vec<String> = std::env::args().collect();
        return args;
    }

    pub async fn execute_next(&mut self) -> Result<()> {
        if self.index >= self.args.len() {
            return Err(anyhow::anyhow!("No more arguments to process"));
        }
        let command = self.args[self.index].clone();
        match command.as_str() {
            "install" => {
                self.index += 1;
                if self.index + 1 >= self.args.len() {
                    println!("Usage: install <mod_id> <game_version>");
                    std::process::exit(0);
                }
                let mod_id = self.args[self.index].clone();
                self.index += 1;
                let game_version = self.args[self.index].clone();
                self.index += 1;
                let mod_id = mod_id.parse::<i32>()?;
                let mod_manager = ModManager::new()?;
                if !mod_manager.is_downloaded(mod_id, game_version.clone()) {
                    mod_manager
                        .download_mod(mod_id, game_version.clone())
                        .await?;
                }
                let mut prefix_manager = PrefixManager::new()?;
                prefix_manager
                    .add_mod_to_prefix(mod_id, game_version)
                    .await?;
            }
            "search" => {
                self.index += 1;
                if self.index >= self.args.len() {
                    println!("{}", "Usage: search <mod_name>");
                    std::process::exit(0);
                } else {
                    let mod_name = self.args[self.index].clone();
                    self.index += 1;
                    println!("{:?}", ModManager::search_mod(&mod_name).await?);
                }
            }
            "update" => {
                self.index += 1;
                let mut mod_manager = ModManager::new()?;
                mod_manager.update_modlist().await?;
                println!("Modlist updated successfully!");
            }
            "list" => {
                self.index += 1;
                println!("{:?}", ModManager::get_downloaded_mods());
            }
            "help" => {
                self.index += 1;
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
        println!("{}", "install <mod_id> <game_version> - Installs the specified version of the specified mod from curseforge");
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

    pub fn get_index(&self) -> usize {
        return self.index;
    }

    pub fn set_index(&mut self, index: usize) {
        self.index = index;
    }

    pub fn has_next(&self) -> bool {
        return self.index < self.args.len();
    }
}
