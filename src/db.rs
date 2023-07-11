use sea_orm::{ConnectionTrait, DbErr, Schema};

use crate::models::{Game, Query};

pub async fn init_database<C>(db: &C) -> Result<(), DbErr>
where
    C: ConnectionTrait,
{
    let builder = db.get_database_backend();
    let schema = Schema::new(builder);

    db.execute(builder.build(schema.create_table_from_entity(Query).if_not_exists()))
        .await?;

    db.execute(builder.build(schema.create_table_from_entity(Game).if_not_exists()))
        .await?;
    Ok(())
}
