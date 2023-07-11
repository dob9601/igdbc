use axum::response::{IntoResponse, Response};
use reqwest::StatusCode;
use thiserror::Error;
use tracing::{error, trace};

use crate::routes::games::GameFetchError;

#[allow(clippy::large_enum_variant)]
#[derive(Error, Debug)]
pub enum IgdbcError {
    #[error("HTTP Error {0:?}")]
    Reqwest(#[from] reqwest::Error),
    #[error("URL Parse Error {0:?}")]
    ParseError(#[from] url::ParseError),
    #[error("Serde Json Error {path:?}")]
    SerdeJson { path: String },
    #[error("SeaOrm Error {0:?}")]
    SeaOrm(#[from] sea_orm::DbErr),
    #[error("Axum Error {0:?}")]
    Axum(#[from] axum::Error),

    #[error("Axum Error {0:?}")]
    Hyper(#[from] hyper::Error),

    #[error("Could not find game with id {0}")]
    Status(StatusCode),

    #[error("Error fetching games: {0}")]
    GameFetch(GameFetchError),

    #[error("{0}")]
    Custom(String),
}

impl From<StatusCode> for IgdbcError {
    fn from(code: StatusCode) -> Self {
        Self::Status(code)
    }
}

impl From<GameFetchError> for IgdbcError {
    fn from(value: GameFetchError) -> Self {
        Self::GameFetch(value)
    }
}

impl IntoResponse for IgdbcError {
    fn into_response(self) -> Response {
        trace!("Route returned error: {self}");

        match self {
            Self::Status(code) => code.into_response(),
            Self::GameFetch(error) => error.into_response(),
            _ => {
                error!("Route returned unhandled error: {self}");
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
        }
    }
}
