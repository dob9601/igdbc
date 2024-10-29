pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_games;
mod m20241029_230517_create_queries;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_games::Migration),
            Box::new(m20241029_230517_create_queries::Migration),
        ]
    }
}
