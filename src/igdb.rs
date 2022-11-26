use std::env;

use chrono::{Duration, NaiveDateTime, Utc};
use lazy_static::lazy_static;
use log::info;
use reqwest::{blocking::Client as BlockingClient, Client, Url};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use shared::models::GameJson;
use tokio::time::sleep;

use crate::error::Error;
use crate::models::IGDBGame;

lazy_static! {
    static ref TWITCH_CLIENT_ID: String = env::var("TWITCH_CLIENT_ID").unwrap();
    static ref TWITCH_CLIENT_SECRET: String = env::var("TWITCH_CLIENT_SECRET").unwrap();
    static ref TWITCH_OAUTH2_ENDPOINT: Url =
        Url::parse(&env::var("TWITCH_OAUTH2_ENDPOINT").unwrap()).unwrap();
    static ref IGDB_API_ENDPOINT: Url =
        Url::parse(&env::var("IGDB_API_ENDPOINT").unwrap()).unwrap();
}

pub struct IGDBClient {
    client: Client,
    access_token: String,
    // token_type: String,
    token_expiry: NaiveDateTime,
    next_request: NaiveDateTime,
}

#[derive(Serialize)]
struct TwitchAuthPayload {
    client_id: String,
    client_secret: String,
    grant_type: String,
}

#[derive(Deserialize)]
struct TwitchAuthResponse {
    access_token: String,
    expires_in: u32,
    // token_type: String,
}

impl IGDBClient {
    pub fn new() -> Result<Self, Error> {
        let client = BlockingClient::new();

        let payload = TwitchAuthPayload {
            client_id: TWITCH_CLIENT_ID.to_string(),
            client_secret: TWITCH_CLIENT_SECRET.to_string(),
            grant_type: "client_credentials".to_string(),
        };

        let response: TwitchAuthResponse = client
            .post(TWITCH_OAUTH2_ENDPOINT.as_str())
            .form(&payload)
            .send()?
            .json()?;

        Ok(IGDBClient {
            client: Client::new(),
            access_token: response.access_token,
            // token_type: response.token_type,
            token_expiry: Utc::now().naive_utc() + Duration::seconds(response.expires_in.into()),
            next_request: NaiveDateTime::MIN,
        })
    }
}

impl IGDBClient {
    pub async fn find_games(&mut self, game_name: &str) -> Result<Vec<GameJson>, Error> {
        let games: Vec<IGDBGame> = self
            .request(
                "games",
                format!(
                    "\
search \"{game_name}\";
limit 500;
fields
    id,
    name,
    summary,
    aggregated_rating,
    themes.name,
    url,
    artworks.url,
    cover.url,
    first_release_date, 
    franchise.name,
    genres.name,
    game_modes.name,
    multiplayer_modes.onlinecoop,
    platforms.name;
"
                ),
            )
            .await?;
        Ok(games.into_iter().map(|game| game.into()).collect())
    }
    pub async fn request<T>(&mut self, endpoint: &str, body: String) -> Result<T, Error>
    where
        Self: Sync + Send,
        T: DeserializeOwned,
    {
        info!("Making request to IGDB API endpoint \"{endpoint}\"");

        let sleep_time = self.next_request - Utc::now().naive_utc();
        if sleep_time > Duration::seconds(0) {
            info!(
                "IGDB Client on cooldown! Sleeping for {} milliseconds",
                sleep_time.num_milliseconds()
            );
            sleep(sleep_time.to_std().unwrap()).await;
        }

        if Utc::now().naive_utc() > self.token_expiry {
            info!("access_token has expired! Refreshing");
            self.refresh_access_token().await?;
        }

        let target_url = IGDB_API_ENDPOINT.join(endpoint)?;
        let response = {
            self.client
                .post(target_url)
                .body(body)
                .header("Authorization", format!("Bearer {}", self.access_token))
                .header("Client-ID", TWITCH_CLIENT_ID.as_str())
                .send()
                .await?
        };

        let response_body = response.text().await?;

        let mut truncated_body = response_body.clone();
        truncated_body.truncate(200);

        info!("Received response from API: \"{}\" ...", truncated_body);

        self.next_request = Utc::now().naive_utc() + Duration::milliseconds(250);

        let mut deserializer = serde_json::Deserializer::from_str(&response_body);
        serde_path_to_error::deserialize(&mut deserializer).map_err(|err| Error::SerdeJson {
            path: err.path().to_string(),
        })
    }

    async fn refresh_access_token(&mut self) -> Result<(), Error> {
        *self = IGDBClient::new()?;
        Ok(())
    }
}
