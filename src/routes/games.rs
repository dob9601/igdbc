use axum::extract::{Path, Query, State};
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::{Json, Router};
use reqwest::StatusCode;
use sea_orm::EntityTrait;
use serde::Deserialize;
use serde_json::json;
use thiserror::Error;
use tracing::info;
use views::GameDTO;

use crate::error::IgdbcError;
use crate::models::_entities::games::{self, Entity};
use crate::models::_entities::queries;
use crate::{search_igdb, AppState};

const MAX_GAME_QUERY_LENGTH: usize = 32;
const MAX_RESULTS: usize = 10;

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
    IdNotFound(i32) = 2,
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
) -> Result<Json<Vec<GameDTO>>, IgdbcError> {
    let query = params.query;

    // game name length for 2018 ranged up to around 28. Add a bit of padding by doubling
    if query.len() > MAX_GAME_QUERY_LENGTH {
        return Err(GameFetchError::QueryTooLong.into());
    }

    info!("Querying internal database for {query}");
    let games: Vec<GameDTO> = games::Entity::find_by_query(&state.db, query.clone(), MAX_RESULTS)
        .await?
        .into_iter()
        .map(|game| game.to_json())
        .collect();

    if games.len() >= MAX_RESULTS {
        return Ok(Json(games));
    }

    let maybe_query = queries::Entity::find_by_id(query.to_string())
        .one(&state.db)
        .await?;

    if let Some(ref query_model) = maybe_query {
        if query_model.queried_recently() {
            info!("Not requerying - already queried recently.");
            return Ok(Json(games));
        }
    }

    let games = search_igdb(&state.db, query.clone()).await?;

    Ok(Json(
        games
            .into_iter()
            .take(MAX_RESULTS)
            .map(|game| game.to_json())
            .collect(),
    ))
}

async fn get_game(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<GameDTO>, IgdbcError> {
    let game = games::Entity::find_by_id(id)
        .one(&state.db)
        .await?
        .ok_or(GameFetchError::IdNotFound(id))?;

    Ok(Json(game.to_json()))
}
