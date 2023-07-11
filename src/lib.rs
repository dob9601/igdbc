use futures::future::try_join_all;
use lazy_static::lazy_static;
use sea_orm::{ConnectionTrait, DatabaseConnection, EntityTrait};
use tokio::sync::Mutex;
use tracing::info;

use crate::configuration::{get_config, Config};
use crate::error::IgdbcError;
use crate::igdb::IGDBClient;
use crate::models::GameActive;

lazy_static! {
    pub static ref CONFIG: Config = get_config().unwrap();
    pub static ref IGDB_CLIENT: Mutex<IGDBClient> = Mutex::new(IGDBClient::new().unwrap());
}

pub mod configuration;
pub mod db;
pub mod error;

pub mod igdb;

pub mod models;

pub mod routes;

#[derive(Clone, Debug)]
pub struct AppState {
    db: DatabaseConnection,
}

pub async fn query_igdb<C>(db: &C, query: &str) -> Result<(), IgdbcError>
where
    C: ConnectionTrait,
{
    info!("Refreshing game cache for query {query}");

    let games;
    {
        let mut client = IGDB_CLIENT.lock().await;
        games = client.find_games(query).await?;
    }

    info!("IGDB returned {} games!", games.len());

    info!("Recording information about query");

    let query = models::Query::find_by_id(query.to_string())
        .one(db)
        .await?
        .ok_or(IgdbcError::Custom(
            "Query doesn't exist in database".to_string(),
        ))?;

    try_join_all(
        games
            .iter()
            .map(|game| GameActive::create_or_update(db, game.clone(), &query)),
    )
    .await?;
    Ok(())
}
