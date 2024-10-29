use client::IgdbClient;
use lazy_static::lazy_static;

pub mod apicalypse;
pub mod client;
mod deserializers;
mod game;
pub use game::IgdbGame;
use tokio::sync::Mutex;

use crate::CONFIG;
mod models;

lazy_static! {
    pub static ref IGDB_CLIENT: Mutex<IgdbClient> =
        tokio::runtime::Runtime::new().unwrap().block_on(async {
            Mutex::new(
                IgdbClient::new(
                    CONFIG.twitch.client_id.clone(),
                    CONFIG.twitch.client_secret.clone(),
                )
                .await
                .unwrap(),
            )
        });
}
