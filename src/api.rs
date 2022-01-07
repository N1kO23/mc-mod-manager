use std::path::PathBuf;

use anyhow::Result;

use crate::structs::{Addon, Mod};

const CF_BASE_URL: &str = "https://addons-ecs.forgesvc.net/api/v2/addon/";

pub async fn search_addon(addon_id: i32) -> Result<Addon, anyhow::Error> {
    let url = format!("{}{}", CF_BASE_URL, addon_id);
    let client = reqwest::Client::new();
    let response = client.get(&url).send().await?;
    let body = response.text().await?;
    let addon: Addon = serde_json::from_str(&body)?;
    Ok(addon)
}

pub async fn download_addon(addon_id: i32, version: i32, download_path: PathBuf) -> Result<Mod> {
    // Todo: Implement function that downloads the addon from the curseforge server.
    let r#mod = Mod {
        id: addon_id,
        name: "".to_string(),
        version,
        file_name: "".to_string(),
        file_path: None,
    };
    Ok(r#mod)
}
