use serde::Deserialize;

// #[derive(Serialize)]
// struct TwitchAuthPayload {
//     client_id: String,
//     client_secret: String,
//     grant_type: String,
// }

#[derive(Deserialize)]
pub struct TwitchAuthResponse {
    pub access_token: String,
    pub expires_in: u32,
    pub token_type: String,
}
