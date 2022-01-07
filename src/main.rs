use crate::args_handler::ArgsHandler;
const VERSION: &'static str = env!("CARGO_PKG_VERSION");

mod api;
mod args_handler;
mod config_manager;
mod mod_manager;
mod prefix_manager;
mod structs;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Minecraft Mod Manager v{}", VERSION);
    let mut args_handler = ArgsHandler::new()?;
    while args_handler.has_next() {
        args_handler.execute_next().await?;
    }
    // println!("Searching for mod with ID \"{}\"", "32358");
    // println!("{:?}", api::search_addon(32358).await?);
    Ok(())
}
