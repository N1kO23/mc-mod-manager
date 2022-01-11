extern crate reqwest;
use crate::structs::{Addon, DownloadableFile, DownloadedMod};
use crate::util::concat_string_array;
use anyhow::Result;
use std::{fs::File, io, path::PathBuf};

const CF_BASE_URL: &str = "https://addons-ecs.forgesvc.net/api/v2/addon/";

pub async fn fetch_addon(addon_id: i32) -> Result<Addon, anyhow::Error> {
    print!("Fetching addon {}... ", addon_id);
    let url = format!("{}{}", CF_BASE_URL, addon_id);
    let client = reqwest::Client::new();
    let response = client.get(&url).send().await?;
    let body = response.text().await?;
    let addon: Addon = serde_json::from_str(&body)?;
    println!("{} found", addon.name);
    Ok(addon)
}

pub async fn fetch_addons() -> Result<Vec<Addon>, anyhow::Error> {
    let mut index = 1;
    let mut addons: Vec<Addon> = Vec::new();
    loop {
        if index > 10 {
            break;
        }
        print!("Fetching page {}... ", index);
        let url = format!(
            "{}search?gameId=432&sectionId=6&index={}&sort=5",
            CF_BASE_URL, index
        );
        let client = reqwest::Client::new();
        let response = client.get(&url).send().await;
        match response {
            Ok(response) => {
                println!("{}", response.status());
                if response.status() != 200 {
                    break;
                }
                let body = response.text().await?;
                let mut temp_addons: Vec<Addon> = serde_json::from_str(&body)?;
                if temp_addons.len() == 0 {
                    break;
                }
                addons.append(&mut temp_addons);
                index += 1;
            }
            Err(e) => {
                println!("{}", e);
            }
        }
    }
    Ok(addons)
}

pub async fn search_addon(keyword: &str) -> Result<Vec<Addon>> {
    print!("Searching addons for keyword '{}'... ", keyword);
    let url = format!(
        "{}search?gameId=432&sectionId=6&searchFilter={}",
        CF_BASE_URL, keyword
    );
    let client = reqwest::Client::new();
    let response = client.get(&url).send().await?;
    println!("{}", response.status());
    let body = response.text().await?;
    let addons: Vec<Addon> = serde_json::from_str(&body)?;
    Ok(addons)
}

/// Download the actual mod from the api
pub async fn download_addon(
    addon: &Addon,
    game_version: String,
    download_path: PathBuf,
    modloader: String,
) -> Result<DownloadedMod> {
    let downloadable = get_downloadable_addon(&addon, &game_version, &modloader);
    match downloadable {
        Some(downloadable) => {
            let url = downloadable.download_url.clone();
            let game_version_str = concat_string_array(downloadable.game_version.clone(), ";");
            let path = format!(
                "{};{};{};{}",
                addon.id, addon.name, downloadable.file_name, game_version_str
            );
            check_download_folder(download_path.clone());
            print!("Downloading to {}... ", path);
            let file_path = download_path.join(path.clone());
            let mut file = File::create(&file_path).unwrap();
            let res_bytes = reqwest::get(url).await?.bytes().await?;

            io::copy(&mut res_bytes.as_ref(), &mut file)?;
            println!("Done!");

            return Ok(DownloadedMod {
                id: addon.id,
                version: downloadable.game_version.clone(),
                name: addon.name.clone(),
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
pub fn get_downloadable_addon(
    addon: &Addon,
    version: &str,
    modloader: &str,
) -> Option<DownloadableFile> {
    for file in &addon.latest_files {
        if file.game_version.contains(&version.to_string())
            && file.game_version.contains(&modloader.to_string())
        {
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
