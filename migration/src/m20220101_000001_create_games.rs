use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Game::Table)
                    .col(pk_auto(Game::Id))
                    .col(string(Game::Name))
                    .col(string_null(Game::Summary))
                    .col(float_null(Game::AggregatedRating))
                    .col(string_null(Game::Themes))
                    .col(string(Game::IgdbcUrl))
                    .col(string_null(Game::FirstReleaseDate))
                    .col(string_null(Game::Franchise))
                    .col(string_null(Game::Genres))
                    .col(string_null(Game::GameModes))
                    .col(string_null(Game::SupportsOnlineMultiplayer))
                    .col(string_null(Game::Platforms))
                    .col(string_null(Game::CoverArtUrl))
                    .col(string_null(Game::ArtworkUrl))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Game::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Game {
    Table,
    Id,
    Name,
    Summary,
    AggregatedRating,
    Themes,
    IgdbcUrl,
    FirstReleaseDate,
    Franchise,
    Genres,
    GameModes,
    SupportsOnlineMultiplayer,
    Platforms,
    CoverArtUrl,
    ArtworkUrl,
}
