use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Games::Table)
                    .col(integer(Games::Id).primary_key())
                    .col(string(Games::Name))
                    .col(string(Games::SearchableName))
                    .col(string_null(Games::Summary))
                    .col(float_null(Games::AggregatedRating))
                    .col(string_null(Games::Themes))
                    .col(string(Games::IgdbUrl))
                    .col(timestamp_null(Games::FirstReleaseDate))
                    .col(string_null(Games::Franchise))
                    .col(string_null(Games::Genres))
                    .col(string_null(Games::GameModes))
                    .col(boolean_null(Games::SupportsOnlineMultiplayer))
                    .col(string_null(Games::Platforms))
                    .col(string_null(Games::CoverArtUrl))
                    .col(string_null(Games::ArtworkUrl))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Games::Table).to_owned())
            .await
    }
}

#[allow(clippy::enum_variant_names)]
#[derive(DeriveIden)]
enum Games {
    Table,
    Id,
    Name,
    SearchableName,
    Summary,
    AggregatedRating,
    Themes,
    IgdbUrl,
    FirstReleaseDate,
    Franchise,
    Genres,
    GameModes,
    SupportsOnlineMultiplayer,
    Platforms,
    CoverArtUrl,
    ArtworkUrl,
}
