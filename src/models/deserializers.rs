use chrono::Utc;
use sea_orm::prelude::DateTimeUtc;
use serde::{Deserialize, Deserializer, Serialize};

pub fn deserialize_artworks<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Serialize, Deserialize)]
    struct ImageData {
        url: String,
    }

    let url = <Vec<ImageData>>::deserialize(deserializer)?
        .first()
        .map(|data| data.url.clone());

    Ok(url)
}

pub fn deserialize_cover<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Serialize, Deserialize)]
    struct ImageData {
        url: Option<String>,
    }

    let url = <ImageData>::deserialize(deserializer)?.url;

    Ok(url)
}

pub fn deserialize_unix_timestamp<'de, D>(deserializer: D) -> Result<Option<DateTimeUtc>, D::Error>
where
    D: Deserializer<'de>,
{
    let unix_timestamp = <Option<u32>>::deserialize(deserializer)?;

    Ok(unix_timestamp.map(|unix_timestamp| {
        let naive = chrono::NaiveDateTime::from_timestamp_opt(unix_timestamp.into(), 0).unwrap();
        DateTimeUtc::from_utc(naive, Utc)
    }))
}

pub fn deserialize_franchise<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Serialize, Deserialize)]
    struct Franchise {
        name: String,
    }
    let franchise = <Option<Franchise>>::deserialize(deserializer)?;
    Ok(franchise.map(|franchise| franchise.name))
}

pub fn deserialize_platforms<'de, D>(deserializer: D) -> Result<Option<Vec<String>>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Serialize, Deserialize)]
    struct Platform {
        name: String,
    }
    let platforms = <Option<Vec<Platform>>>::deserialize(deserializer)?;
    Ok(platforms.map(|platforms| {
        platforms
            .into_iter()
            .map(|item| item.name)
            .collect::<Vec<String>>()
    }))
}

pub fn deserialize_genres<'de, D>(deserializer: D) -> Result<Option<Vec<String>>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Serialize, Deserialize)]
    struct Genre {
        name: String,
    }
    let genres = <Option<Vec<Genre>>>::deserialize(deserializer)?;
    Ok(genres.map(|genres| {
        genres
            .into_iter()
            .map(|item| item.name)
            .collect::<Vec<String>>()
    }))
}

pub fn deserialize_game_modes<'de, D>(deserializer: D) -> Result<Option<Vec<String>>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Serialize, Deserialize)]
    struct GameMode {
        name: String,
    }
    let game_modes = <Option<Vec<GameMode>>>::deserialize(deserializer)?;
    Ok(game_modes.map(|game_modes| {
        game_modes
            .into_iter()
            .map(|item| item.name)
            .collect::<Vec<String>>()
    }))
}

pub fn deserialize_themes<'de, D>(deserializer: D) -> Result<Option<Vec<String>>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Serialize, Deserialize)]
    struct Theme {
        name: String,
    }
    let themes = <Option<Vec<Theme>>>::deserialize(deserializer)?;
    Ok(themes.map(|themes| {
        themes
            .into_iter()
            .map(|item| item.name)
            .collect::<Vec<String>>()
    }))
}

pub fn deserialize_supports_online_multiplayer<'de, D>(
    deserializer: D,
) -> Result<Option<bool>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Serialize, Deserialize)]
    struct MultiplayerMetadata {
        onlinecoop: bool,
    }
    let multiplayer_metadata = <Option<Vec<MultiplayerMetadata>>>::deserialize(deserializer)?;
    Ok(multiplayer_metadata.map(|multiplayer_metadata| {
        multiplayer_metadata
            .into_iter()
            .map(|item| item.onlinecoop)
            .any(|next| next)
    }))
}
