use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
    pub minecraft_installation_path: PathBuf,
    pub mod_download_path: PathBuf,
    pub active_prefix: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Addon {
    pub id: i32,
    pub name: String,
    pub slug: String,
    pub authors: Vec<Author>,
    pub website_url: String,
    pub summary: String,
    pub status: i32,
    pub game_id: i32,
    pub primary_category_id: i32,
    pub download_count: f32,
    pub popularity_score: f32,
    pub date_created: String,
    pub date_modified: String,
    pub date_released: String,
    pub is_available: bool,
    pub is_experiemental: bool,
    pub is_featured: bool,
    pub latest_files: Vec<DownloadableFile>,
    pub mod_loaders: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DownloadableFile {
    pub id: i32,
    pub file_name: String,
    pub file_date: String,
    pub download_url: String,
    pub game_version: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Author {
    pub name: String,
    pub url: String,
    pub id: i32,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Prefix {
    pub name: String,
    pub description: String,
    pub author: String,
    pub version: String,
    pub mod_list: Vec<i32>,
    pub game_version: String,
    pub mod_loader: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DownloadedMod {
    pub id: i32,
    pub name: String,
    pub version: String,
    pub file_name: String,
    pub file_path: Option<PathBuf>,
}
