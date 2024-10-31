use axum::Router;
use sea_orm::Database;
use tower_http::trace::{self, TraceLayer};
use tracing::Level;

use crate::{db::init_database, error::IgdbcError, AppState};

pub mod games;

pub async fn app(db_url: &str) -> Result<Router, IgdbcError> {
    let db = Database::connect(db_url).await?;
    init_database(&db).await?;

    let state = AppState { db };

    let router = Router::new()
        .nest("/games", games::router())
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO))
                .on_failure(trace::DefaultOnFailure::new().level(Level::INFO)),
        )
        .with_state(state);

    Ok(router)
}
