use chrono::{Duration, Utc};
use lazy_static::lazy_static;
use log::{info, warn};
use rocket::futures::future::try_join_all;
use rocket::serde::json::Json;
use rocket::tokio::runtime;
use rocket::{get, routes};
use rocket::{Ignite, Rocket};
use sea_orm::{ColumnTrait, Database, EntityTrait, QueryFilter, QuerySelect};
use std::env;
use tokio::sync::Mutex;

mod models;

mod igdb;

mod db;
pub mod error;
use db::DATABASE_CONNECTION;

use crate::error::Error;
use crate::models::{Game, GameActive, GameColumn, GameJson, Query, QueryActive};

use self::db::{get_database_connection, initialize_database};
use self::igdb::IGDBClient;

// Could solve this by making the new() method sync? Only called once so might make sense
// Would have to create a non-async client and then throw it away. Big brain me is already doing
// this with 2 async clients accidentally
lazy_static! {
    static ref IGDB_CLIENT: Mutex<IGDBClient> = Mutex::new(IGDBClient::new().unwrap());
}

fn main() -> Result<(), Error> {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .init();

    lazy_static::initialize(&IGDB_CLIENT);

    let database_url = env::var("IGDBC_DATABASE_URL")
        .or(Err(
            "Missing required environment variable 'IGDBC_DATABASE_URL'",
        ))
        .unwrap();

    let db = runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(Database::connect(database_url))
        .unwrap();
    DATABASE_CONNECTION.set(db).unwrap();

    runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(run())
        .map(|_| ())
}

async fn run() -> Result<Rocket<Ignite>, Error> {
    initialize_database().await?;

    Ok(rocket::build()
        .mount("/", routes![query_games, get_game])
        .launch()
        .await?)
}

#[get("/games?<query>")]
async fn query_games(query: &str) -> Result<Json<Vec<GameJson>>, Error> {
    let db = get_database_connection().await;

    info!("Querying internal database for {query}");
    let games = Game::find()
        .filter(GameColumn::Name.like(&format!("{query}%")))
        .limit(10) // TODO: Could make this customisable
        .all(db)
        .await?
        .iter()
        .map(|game| game.to_json())
        .collect::<Vec<GameJson>>();

    if games.len() < 10 {
        let mut should_requery = true;
        warn!("Query \"{query}\" returned a low number of results! Attempting to scrape IGDB for more matches");
        if let Some(query_model) = Query::find_by_id(query.to_string()).one(db).await? {
            if Utc::now() - query_model.queried_at < Duration::weeks(4) {
                info!("Query is known to return a low number of results recently (or is in the queue to be queried). Not requerying.");
                should_requery = false;
            } else {
                info!("Query has been attempted before, but not in the last 4 weeks. Retrying");
            }
        } else {
            info!("Query has not been attempted before, proceeding");
        }

        if should_requery {
            let cloned_query = query.to_string();
            if Query::find_by_id(query.to_string())
                .one(db)
                .await?
                .is_none()
            {
                QueryActive::create(db, query.to_string()).await?;
            }
            tokio::spawn(async move {
                if let Err(err) = query_igdb(&cloned_query).await {
                    warn!("Failed to update game cache: {err:?}")
                }
            });
        }
    }
    Ok(Json(games))
}

async fn query_igdb(query: &str) -> Result<(), Error> {
    info!("Refreshing game cache for query {query}");
    let db = get_database_connection().await;

    let games;
    {
        let mut client = IGDB_CLIENT.lock().await;
        games = client.find_games(query).await?;
    }

    info!("IGDB returned {} games!", games.len());

    info!("Recording information about query");

    let query = Query::find_by_id(query.to_string())
        .one(db)
        .await?
        .ok_or(Error::Custom {
            message: "Query doesn't exist in database".to_string(),
        })?;

    try_join_all(
        games
            .iter()
            .map(|game| GameActive::create_or_update(db, game.clone(), &query)),
    )
    .await?;
    Ok(())
}

#[get("/games/<game_id>")]
async fn get_game(game_id: u32) -> Result<Json<GameJson>, Error> {
    let db = get_database_connection().await;
    let game = Game::find_by_id(game_id)
        .one(db)
        .await?
        .ok_or(Error::NotFound)?;
    Ok(Json(game.to_json()))
}
