use chrono::{Duration, NaiveDateTime, TimeDelta, Utc};
use reqwest::Client;
use tokio::time::sleep;

use super::{apicalypse::ApicalypseQuery, models::TwitchAuthResponse, IgdbGame};

const TWITCH_OAUTH2_ENDPOINT: &str = "https://id.twitch.tv/oauth2/token?client_id={client_id}&client_secret={client_secret}&grant_type=client_credentials";
const IGDB_GAMES_URL: &str = "https://api.igdb.com/v4/games";
const REQUEST_DELAY_MS: i64 = 260;

pub struct IgdbClient {
    client: Client,
    client_id: String,
    client_secret: String,
    access_token: String,
    token_expiry: NaiveDateTime,
    next_request: NaiveDateTime,
}

type IgdbResult<T> = Result<T, reqwest::Error>;

impl IgdbClient {
    pub async fn new(client_id: String, client_secret: String) -> IgdbResult<Self> {
        let client = Client::new();

        let response = Self::refresh_access_token(&client, &client_id, &client_secret).await?;

        Ok(Self {
            client,
            client_id,
            client_secret,
            access_token: response.access_token,
            token_expiry: Utc::now().naive_utc() + Duration::seconds(response.expires_in.into()),
            next_request: Utc::now().naive_utc(),
        })
    }

    pub async fn search(&mut self, query: String) -> IgdbResult<Vec<IgdbGame>> {
        let apicalypse_query = ApicalypseQuery::builder()
            .search(query)
            .fields(vec![
                "id",
                "name",
                "url",
                "summary",
                "cover.url",
                "artworks.url",
                "multiplayer_modes.onlinecoop",
                "first_release_date",
            ])
            // Only main-games (exclude DLCs etc.)
            .r#where("category = 0")
            // As above, in case of upstream incorrect metadata
            .and_where("parent_game = null")
            // Exclude versions of games
            .and_where("version_parent = null")
            .limit(500);

        if self.token_expiry < Utc::now().naive_utc() {
            let auth_response =
                Self::refresh_access_token(&self.client, &self.client_id, &self.client_secret)
                    .await?;

            self.access_token = auth_response.access_token;
            self.token_expiry =
                Utc::now().naive_utc() + Duration::seconds(auth_response.expires_in.into());
        }

        let request_delay = self.next_request - Utc::now().naive_utc();

        if request_delay.num_seconds() > 0 {
            // FIXME(Dan): Could remove this unwrap, but if this error happens then the world might be collapsing
            sleep(request_delay.to_std().unwrap()).await;
        }

        let response = self
            .client
            .post(IGDB_GAMES_URL)
            .body(apicalypse_query.to_string())
            .header("Client-ID", &self.client_id)
            .bearer_auth(&self.access_token)
            .send()
            .await?
            .error_for_status()?
            .json::<Vec<IgdbGame>>()
            .await?;

        self.next_request += TimeDelta::milliseconds(REQUEST_DELAY_MS);

        Ok(response)
    }

    async fn refresh_access_token(
        client: &Client,
        client_id: &str,
        client_secret: &str,
    ) -> IgdbResult<TwitchAuthResponse> {
        client
            .post(
                TWITCH_OAUTH2_ENDPOINT
                    .replace("{client_id}", client_id)
                    .replace("{client_secret}", client_secret),
            )
            .send()
            .await?
            .error_for_status()?
            .json::<TwitchAuthResponse>()
            .await
    }
}
