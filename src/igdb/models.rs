use serde::Deserialize;

#[derive(Deserialize)]
pub struct TwitchAuthResponse {
    pub access_token: String,
    pub expires_in: u32,
}
