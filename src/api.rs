use anyhow::Result;

use crate::structs::Addon;

const CF_BASE_URL: &str = "https://addons-ecs.forgesvc.net/api/v2/addon/";

pub async fn search_addon(addon_id: i32) -> Result<Addon, anyhow::Error> {
    let url = format!("{}{}", CF_BASE_URL, addon_id);
    let client = reqwest::Client::new();
    let response = client.get(&url).send().await?;
    let body = response.text().await?;
    let addon: Addon = serde_json::from_str(&body)?;
    Ok(addon)
}
