use std::net::SocketAddr;
use std::str::FromStr;

use axum::Server;
use igdbc::CONFIG;

use igdbc::error::IgdbcError;
use tokio::runtime;
use tracing::Level;

fn main() -> Result<(), IgdbcError> {
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .with_env_filter("sqlx=off,igdbc=trace,axum=trace,hyper=warn,tower_http=trace,sea_orm=info")
        .with_line_number(true)
        .init();

    println!(
        r#"
  _____ _____ _____  ____   _____ 
 |_   _/ ____|  __ \|  _ \ / ____|
   | || |  __| |  | | |_) | |     
   | || | |_ | |  | |  _ <| |     
  _| || |__| | |__| | |_) | |____ 
 |_____\_____|_____/|____/ \_____|"#
    );

    // Don't initialize igdb client immediately so can operate without a twitch connection in some situations
    lazy_static::initialize(&CONFIG);
    lazy_static::initialize(&igdbc::igdb::IGDB_CLIENT);

    runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(run())
}

async fn run() -> Result<(), IgdbcError> {
    let app = igdbc::routes::app(&CONFIG.database_url).await?;
    let addr = SocketAddr::from_str(&CONFIG.address).unwrap();

    Server::bind(&addr).serve(app.into_make_service()).await?;

    Ok(())
}
