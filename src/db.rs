use std::env;

use once_cell::sync::OnceCell;
use sea_orm::{ConnectionTrait, Database, DatabaseConnection, DbErr, Schema};

use crate::models::{Game, Query};

pub static DATABASE_CONNECTION: OnceCell<DatabaseConnection> = OnceCell::new();

pub async fn get_database_connection() -> &'static DatabaseConnection {
    if let Some(connection) = DATABASE_CONNECTION.get() {
        connection
    } else {
        let database_url =
            env::var("DATABASE_URL").expect("Missing required environment variable 'DATABASE_URL'");
        let db = Database::connect(database_url)
            .await
            .expect("Failed to connect to database");
        DATABASE_CONNECTION.set(db).unwrap();
        DATABASE_CONNECTION.get().unwrap()
    }
}

pub async fn initialize_database() -> Result<(), DbErr> {
    let db = get_database_connection().await;

    let builder = db.get_database_backend();
    let schema = Schema::new(builder);

    db.execute(builder.build(schema.create_table_from_entity(Game).if_not_exists()))
        .await?;

    db.execute(builder.build(schema.create_table_from_entity(Query).if_not_exists()))
        .await?;
    Ok(())
}
