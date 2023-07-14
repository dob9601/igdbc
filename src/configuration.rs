use reqwest::Url;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub database_url: String,
    pub address: String,
    pub allowed_origins: Vec<String>,
    pub igdb: IGDB,
    pub twitch: Twitch,
    pub cloudinary: Cloudinary,
}

#[derive(Serialize, Deserialize)]
pub struct IGDB {
    pub api_endpoint: Url,
}

#[derive(Serialize, Deserialize)]
pub struct Cloudinary {
    pub api_endpoint: Url,
    pub cloud_name: String,
    pub api_key: String,
    pub api_secret: String,
}

#[derive(Serialize, Deserialize)]
pub struct Twitch {
    pub client_id: String,
    pub client_secret: String,
    pub oauth2_endpoint: Url,
}

pub fn get_config() -> Result<Config, config::ConfigError> {
    config::Config::builder()
        .add_source(config::File::with_name("Config.toml").required(false))
        .add_source(
            config::Environment::with_prefix("IGDBC")
                .try_parsing(true)
                .separator("__")
                .list_separator(",")
                .with_list_parse_key("allowed_origins"),
        )
        .build()?
        .try_deserialize()
}
