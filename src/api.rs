extern crate reqwest;
use crate::structs::{Addon, DownloadableFile, DownloadedMod};
use anyhow::Result;
use std::{fs::File, io, path::PathBuf};

const CF_BASE_URL: &str = "https://addons-ecs.forgesvc.net/api/v2/addon/";

pub async fn fetch_addon(addon_id: i32) -> Result<Addon, anyhow::Error> {
    let url = format!("{}{}", CF_BASE_URL, addon_id);
    let client = reqwest::Client::new();
    let response = client.get(&url).send().await?;
    let body = response.text().await?;
    let addon: Addon = serde_json::from_str(&body)?;
    Ok(addon)
}

pub async fn fetch_addons() -> Result<Vec<Addon>, anyhow::Error> {
    let url = format!("{}search?gameId=432&sectionId=6", CF_BASE_URL);
    let client = reqwest::Client::new();
    let response = client.get(&url).send().await?;
    let body = response.text().await?;
    let addons: Vec<Addon> = serde_json::from_str(&body)?;
    Ok(addons)
}

pub async fn download_addon(
    addon: Addon,
    game_version: String,
    download_path: PathBuf,
) -> Result<DownloadedMod> {
    // Figure out the actual api endpoint for this.
    let downloadable = get_downloadable_addon(&addon, &game_version);
    match downloadable {
        Some(downloadable) => {
            let url = downloadable.download_url.clone();
            let path = format!(
                "{}/{}-{}.jar",
                download_path.to_string_lossy(),
                addon.id,
                downloadable.game_version[0].clone(),
            );
            check_download_folder(download_path.clone());
            let file_path = download_path.join(downloadable.file_name.clone());
            let mut file = File::create(&file_path).unwrap();
            let res_bytes = reqwest::get(url).await?.bytes().await?;

            io::copy(&mut res_bytes.as_ref(), &mut file)?;

            return Ok(DownloadedMod {
                id: addon.id,
                version: downloadable.game_version[0].clone(),
                name: addon.name,
                file_name: downloadable.file_name,
                file_path: Some(file_path),
            });
        }
        None => {
            return Err(anyhow::anyhow!("No downloadable file found"));
        }
    }
}

/// Get the downloadable file struct if there is any for the given game version.
fn get_downloadable_addon(addon: &Addon, version: &str) -> Option<DownloadableFile> {
    for file in &addon.latest_files {
        if file.game_version[0] == version {
            return Some(file.clone());
        }
    }
    return None;
}

fn check_download_folder(path: PathBuf) {
    if !path.exists() {
        println!("Download folder does not exist, creating...");
        std::fs::create_dir_all(path).unwrap();
    }
}
