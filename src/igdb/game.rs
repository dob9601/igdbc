use chrono::{DateTime, Utc};
use serde::Deserialize;

use super::deserializers::*;

#[derive(Deserialize, Clone)]
pub struct IgdbGame {
    pub id: i32,

    pub name: String,
    pub summary: Option<String>,
    pub aggregated_rating: Option<f32>,

    #[serde(deserialize_with = "deserialize_themes", default)]
    pub themes: Option<Vec<String>>,

    pub url: String,

    #[serde(deserialize_with = "deserialize_artworks", default)]
    pub artworks: Option<String>,

    #[serde(deserialize_with = "deserialize_cover", default)]
    pub cover: Option<String>,

    #[serde(deserialize_with = "deserialize_unix_timestamp", default)]
    pub first_release_date: Option<DateTime<Utc>>,

    #[serde(deserialize_with = "deserialize_franchise", default)]
    pub franchise: Option<String>,

    #[serde(deserialize_with = "deserialize_genres", default)]
    pub genres: Option<Vec<String>>,

    #[serde(deserialize_with = "deserialize_game_modes", default)]
    pub game_modes: Option<Vec<String>>,

    #[serde(
        deserialize_with = "deserialize_supports_online_multiplayer",
        rename(deserialize = "multiplayer_modes"),
        default
    )]
    pub supports_online_multiplayer: Option<bool>,

    #[serde(deserialize_with = "deserialize_platforms", default)]
    pub platforms: Option<Vec<String>>,
}
