use log::error;
use rocket::http::Status;
use rocket::response::Responder;
use rocket::{response, Request};
use thiserror::Error;

#[allow(clippy::large_enum_variant)]
#[derive(Error, Debug)]
pub enum Error {
    #[error("HTTP Error {source:?}")]
    Reqwest {
        #[from]
        source: reqwest::Error,
    },
    #[error("URL Parse Error {source:?}")]
    ParseError {
        #[from]
        source: url::ParseError,
    },
    #[error("Serde Json Error {path:?}")]
    SerdeJson { path: String },
    #[error("SeaOrm Error {source:?}")]
    SeaOrm {
        #[from]
        source: sea_orm::DbErr,
    },
    #[error("Rocket Error {source:?}")]
    Rocket {
        #[from]
        source: rocket::Error,
    },

    #[error("Could not find game with id {0}")]
    NotFound(u32),

    #[error("{message}")]
    Custom { message: String },
}

impl<'r, 'o: 'r> Responder<'r, 'o> for Error {
    fn respond_to(self, req: &'r Request<'_>) -> response::Result<'o> {
        // log `self` to your favored error tracker, e.g.
        // sentry::capture_error(&self);
        error!("{self}");

        match self {
            Self::NotFound(_) => Status::NotFound.respond_to(req),
            _ => Status::InternalServerError.respond_to(req),
        }
    }
}
