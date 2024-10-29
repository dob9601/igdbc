use migration::MigratorTrait;
use sea_orm::{DatabaseConnection, DbErr};

pub async fn init_database(db: &DatabaseConnection) -> Result<(), DbErr> {
    migration::Migrator::up(db, None).await?;
    Ok(())
}
