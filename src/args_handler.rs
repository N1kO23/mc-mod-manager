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
                if self.index >= self.args.len() {
                    println!("Usage: install <mod_id>");
                    std::process::exit(0);
                }
                let mut prefix_manager = PrefixManager::new()?;

                let mod_id = self.args[self.index].clone();
                self.index += 1;
                let game_version = prefix_manager.prefix.game_version.clone();
                let mod_id = mod_id.parse::<i32>()?;
                let mod_manager = ModManager::new()?;
                if !mod_manager.is_downloaded(mod_id, game_version.clone()) {
                    mod_manager
                        .download_mod(mod_id, game_version.clone())
                        .await?;
                }
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
                    let found_addons = ModManager::search_mod(&mod_name).await?;
                    for addon in found_addons {
                        println!("{}\t | {}", addon.id, addon.name);
                        println!("{}", addon.summary);
                    }
                }
            }
            "update" => {
                self.index += 1;
                ModManager::update_modlist().await?;
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
        println!(
            "{}",
            "install <mod_id> - Installs the specified mod into prefix"
        );
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

    pub fn has_next(&self) -> bool {
        return self.index < self.args.len();
    }
}
