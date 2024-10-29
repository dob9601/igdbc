use futures::future::try_join_all;
use lazy_static::lazy_static;
use sea_orm::{ConnectionTrait, DatabaseConnection, EntityTrait};
use tracing::info;

use crate::configuration::{get_config, Config};
use crate::error::IgdbcError;
use crate::igdb::apicalypse::ApicalypseQuery;
use crate::igdb::IGDB_CLIENT;
use crate::models::GameActive;
use crate::views::GameJson;

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

pub async fn search_igdb<C>(db: &C, query: &str) -> Result<(), IgdbcError>
where
    C: ConnectionTrait,
{
    info!("Refreshing game cache for query {query}");

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

    let games: Vec<GameJson>;
    {
        let mut client = IGDB_CLIENT.lock().await;
        games = client.search(&apicalypse_query).await?;
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
