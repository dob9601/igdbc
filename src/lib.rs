use futures::future::try_join_all;
use lazy_static::lazy_static;
use models::_entities::{games, queries};
use sea_orm::{ConnectionTrait, DatabaseConnection};
use tracing::info;

use crate::configuration::{get_config, Config};
use crate::error::IgdbcError;
use crate::igdb::IGDB_CLIENT;

lazy_static! {
    pub static ref CONFIG: Config = get_config().unwrap();
}

pub mod configuration;
pub mod db;
pub mod error;
pub mod igdb;
pub mod models;
pub mod routes;
pub mod views;

#[derive(Clone, Debug)]
pub struct AppState {
    db: DatabaseConnection,
}

pub async fn search_igdb<C>(db: &C, query: String) -> Result<Vec<games::Model>, IgdbcError>
where
    C: ConnectionTrait,
{
    info!("Refreshing game cache for query {query}");

    let games;
    {
        let mut client = IGDB_CLIENT.lock().await;
        games = client.search(query.clone()).await?;
    }

    info!("IGDB returned {} games!", games.len());

    info!("Recording information about query");

    queries::Entity::find_or_create(db, query).await?;

    let games = try_join_all(
        games
            .iter()
            .map(|game| games::Entity::create_or_update(db, game.clone())),
    )
    .await?;

    Ok(games)
}
