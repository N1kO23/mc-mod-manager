const VERSION: &'static str = env!("CARGO_PKG_VERSION");

mod api;
mod structs;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Minecraft Mod Manager v{}", VERSION);
    println!("Searching for mod with ID \"{}\"", "32358");
    println!("{:?}", api::search_addon(32358).await?);
    Ok(())
}
