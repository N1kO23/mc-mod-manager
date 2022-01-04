use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Addon {
    pub id: i32,
    pub name: String,
    pub slug: String,
    pub authors: Vec<Author>,
    pub websiteUrl: String,
    pub summary: String,
    pub status: i32,
    pub gameId: i32,
    pub primaryCategoryId: i32,
    pub popularityScore: f32,
    pub gamePopularityRank: i32,
    pub dateCreated: String,
    pub dateModified: String,
    pub dateReleased: String,
    pub isAvailable: bool,
    pub isExperiemental: bool,
    pub isFeatured: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Author {
    pub name: String,
    pub url: String,
    pub id: i32,
}
