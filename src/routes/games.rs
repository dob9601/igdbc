use axum::extract::{Path, Query, State};
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::{Json, Router};
use chrono::{Duration, Utc};
use reqwest::StatusCode;
use sea_orm::EntityTrait;
use serde::Deserialize;
use serde_json::json;
use shared::models::GameJson;
use thiserror::Error;
use tracing::{info, warn};

use crate::error::IgdbcError;
use crate::models::{self, Game, QueryActive};
use crate::{search_igdb, AppState};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(query_games))
        .route("/:id", get(get_game))
}

#[derive(Error, Debug, Clone)]
#[repr(u8)]
pub enum GameFetchError {
    #[error("Populating game cache for query, please try again later.")]
    RepopulatingCache = 0,

    #[error("The query you provided is too long")]
    QueryTooLong = 1,

    #[error("Could not find a game with ID '{0}'")]
    IdNotFound(u32) = 2,
}

impl GameFetchError {
    pub fn code(&self) -> u8 {
        match self {
            GameFetchError::RepopulatingCache => 0,
            GameFetchError::QueryTooLong => 1,
            GameFetchError::IdNotFound(_) => 2,
        }
    }
}

impl IntoResponse for GameFetchError {
    fn into_response(self) -> Response {
        let status_code = match &self {
            GameFetchError::IdNotFound(_) => StatusCode::NOT_FOUND,
            _ => StatusCode::BAD_REQUEST,
        };

        let json = Json(json!({
            "message": self.to_string(),
            "code": self.code()
        }));

        (status_code, json).into_response()
    }
}

#[derive(Clone, Deserialize)]
pub struct GameQueryParams {
    query: String,
}

async fn query_games(
    State(state): State<AppState>,
    Query(params): Query<GameQueryParams>,
) -> Result<Json<Vec<GameJson>>, IgdbcError> {
    let query = params.query;

    // game name length for 2018 ranged up to around 28. Add a bit of padding.
    if query.len() > 30 {
        return Err(GameFetchError::QueryTooLong.into());
    }

    info!("Querying internal database for {query}");
    let games: Vec<GameJson> = Game::find_by_query(&state.db, &query)
        .await?
        .into_iter()
        .map(|game| game.to_json())
        .collect();

    if games.len() < 10 {
        let mut should_requery = true;
        warn!("Query \"{query}\" returned a low number of results! Attempting to scrape IGDB for more matches");
        if let Some(query_model) = models::Query::find_by_id(query.to_string())
            .one(&state.db)
            .await?
        {
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
            if models::Query::find_by_id(query.to_string())
                .one(&state.db)
                .await?
                .is_none()
            {
                QueryActive::create(&state.db, query.to_string()).await?;
            }

            search_igdb(&state.db, &cloned_query).await?;

            return Ok(Json(
                Game::find_by_query(&state.db, &query)
                    .await?
                    .into_iter()
                    .map(|game| game.to_json())
                    .collect(),
            ));
        }
    }
    Ok(Json(games))
}

async fn get_game(
    State(state): State<AppState>,
    Path(id): Path<u32>,
) -> Result<Json<GameJson>, IgdbcError> {
    let game = Game::find_by_id(id)
        .one(&state.db)
        .await?
        .ok_or_else(|| GameFetchError::IdNotFound(id))?;

    Ok(Json(game.to_json()))
}
