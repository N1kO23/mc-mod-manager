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

                let mod_id_vec = self.args_into_i32()?;
                self.index += 1;
                let game_version = prefix_manager.prefix.game_version.clone();
                let mut mod_manager = ModManager::new()?;
                for mod_id in mod_id_vec {
                    if !mod_manager.is_downloaded(mod_id, game_version.clone()) {
                        mod_manager
                            .download_mod(
                                mod_id,
                                game_version.clone(),
                                prefix_manager.prefix.mod_loader.clone(),
                            )
                            .await?;
                    } else {
                        println!("Mod already downloaded");
                    }
                    prefix_manager
                        .add_mod_to_prefix(mod_id, game_version.clone())
                        .await?;
                }
            }
            "search" => {
                self.index += 1;
                if self.index >= self.args.len() {
                    println!("{}", "Usage: search <mod_name>");
                    std::process::exit(0);
                } else {
                    let mod_name = self.join_args_string();
                    self.index += 1;
                    let found_addons = ModManager::search_mod(&mod_name).await?;
                    for addon in found_addons {
                        println!(
                            "{}\t| {} ({} total downloads)",
                            addon.id, addon.name, addon.download_count as i32
                        );
                        println!("\t└{}", addon.summary);
                    }
                }
            }
            "upgrade" => {
                self.index += 1;
                // ModManager::upgrade_mod().await?;
                // println!("Modlist updated successfully!");
            }
            "list" => {
                self.index += 1;
                print!("{}", "Fetching all downloaded mods... ");
                let installed_addons = ModManager::get_downloaded_mods()?;
                println!("Done!");
                if installed_addons.len() == 0 {
                    println!("No downloaded mods found!");
                } else if installed_addons.len() == 1 {
                    println!("1 downloaded mod found:");
                } else {
                    println!("{} downloaded mods found:", installed_addons.len());
                }
                for addon in installed_addons {
                    println!("{}\t| {} {:?}", addon.id, addon.name, addon.version);
                }
            }
            "help" => {
                self.index += 1;
                self.help();
            }
            "prefix" => {
                self.index += 1;
                match self.args[self.index].clone().as_str() {
                    "new" => {
                        self.index += 1;
                        if self.index >= self.args.len() {
                            println!("Usage: prefix new <prefix_name>");
                            std::process::exit(0);
                        }
                        let prefix_name = self.join_args_string();
                        PrefixManager::new_prefix_form(&prefix_name)?;
                    }
                    "set" => {
                        self.index += 1;
                        if self.index >= self.args.len() {
                            println!("Usage: prefix set <prefix_name>");
                            std::process::exit(0);
                        }
                        let prefix_name = self.join_args_string();
                        // PrefixManager::set_prefix(prefix_name, prefix_path)?;
                    }
                    "list" => {
                        self.index += 1;
                        let prefix_manager = PrefixManager::new()?;
                        // let prefixes = prefix_manager.get_prefixes()?;
                        // for prefix in prefixes {
                        //     println!("{}", prefix.name);
                        // }
                    }
                    "delete" => {
                        self.index += 1;
                        if self.index >= self.args.len() {
                            println!("Usage: prefix delete <prefix_name>");
                            std::process::exit(0);
                        }
                        let prefix_name = self.join_args_string();
                        // PrefixManager::delete_prefix(prefix_name)?;
                    }
                    _ => {
                        println!("Usage: prefix <set|list|delete> <prefix_name>");
                        std::process::exit(0);
                    }
                }
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

    fn join_args_string(&mut self) -> String {
        let mut joined_args = String::new();
        while self.has_next() {
            joined_args.push_str(&self.args[self.index]);
            self.index += 1;
            if self.has_next() {
                joined_args.push_str(" ");
            }
        }
        return joined_args;
    }

    fn args_into_i32(&mut self) -> Result<Vec<i32>> {
        let mut args_i32 = Vec::new();
        while self.has_next() {
            let num = self.args[self.index].clone().parse::<i32>()?;
            args_i32.push(num);
            self.index += 1;
        }
        return Ok(args_i32);
    }
}
