extern crate reqwest;
use crate::structs::{Addon, Mod};
use anyhow::Result;
use std::{fs::File, io, path::PathBuf};

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
    // Figure out the actual api endpoint for this.
    let url = CF_BASE_URL;
    let res_bytes = reqwest::get(url).await?.bytes().await?;
    let mut file = File::create(format!(
        "{}/{}-{}.jar",
        download_path.to_string_lossy(),
        addon_id,
        version
    ))?;
    io::copy(&mut res_bytes.as_ref(), &mut file)?;
    let r#mod = Mod {
        id: addon_id,
        name: "".to_string(),
        version,
        file_name: "".to_string(),
        file_path: None,
    };
    Ok(r#mod)
}
