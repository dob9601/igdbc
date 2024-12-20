use super::_entities::games::{ActiveModel, Column, Entity, Model};
use crate::igdb::IgdbGame;
use migration::extension::postgres::PgExpr;
use sea_orm::{prelude::*, ConnectionTrait, QuerySelect, Set, TryIntoModel};
use tracing::trace;
use views::GameDTO;

impl Entity {
    const UNSEARCHABLE_CHARS: [char; 21] = [
        '[', ']', '"', '\'', ':', ';', '!', '@', '#', '$', '%', '^', '&', '*', '(', ')', '-', '+',
        '=', '~', ' ',
    ];

    pub async fn find_by_query<C>(db: &C, query: String, limit: usize) -> Result<Vec<Model>, DbErr>
    where
        C: ConnectionTrait,
    {
        Self::find()
            // FIXME(Dan): Use gamename entity, serialise without special characters to make searching better.
            .filter(
                Expr::col(Column::SearchableName)
                    .ilike(format!("{}%", Self::make_searchable_name(query))),
            )
            .limit(limit as u64)
            .all(db)
            .await
    }

    pub async fn create_or_update<C>(db: &C, json: IgdbGame) -> Result<Model, DbErr>
    where
        C: ConnectionTrait,
    {
        trace!("Creating/Updating game '{}'", json.name);
        let maybe_model = Entity::find_by_id(json.id).one(db).await?;

        if let Some(model) = maybe_model {
            trace!("Game '{}' already exists - updating", json.name);
            let mut active_model: ActiveModel = model.into();
            active_model.id = Set(json.id);
            active_model.name = Set(json.name);
            active_model.summary = Set(json.summary);
            active_model.aggregated_rating = Set(json.aggregated_rating);
            active_model.themes = Set(json.themes.map(|themes| themes.join(",")));
            active_model.igdb_url = Set(json.url);
            active_model.first_release_date = Set(json.first_release_date);
            active_model.franchise = Set(json.franchise);
            active_model.genres = Set(json.genres.map(|genres| genres.join(",")));
            active_model.game_modes = Set(json.game_modes.map(|game_modes| game_modes.join(",")));
            active_model.supports_online_multiplayer = Set(json.supports_online_multiplayer);
            active_model.platforms = Set(json.platforms.map(|platforms| platforms.join(",")));
            active_model.save(db).await?.try_into_model()
        } else {
            trace!("Game '{}' does not exist - creating", json.name);
            Self::create(db, json).await
        }
    }

    pub async fn create<C>(db: &C, json: IgdbGame) -> Result<Model, DbErr>
    where
        C: ConnectionTrait,
    {
        let model = ActiveModel {
            id: Set(json.id),
            name: Set(json.name.clone()),
            searchable_name: Set(Self::make_searchable_name(json.name)),
            summary: Set(json.summary),
            aggregated_rating: Set(json.aggregated_rating),
            themes: Set(json.themes.map(|themes| themes.join(","))),
            igdb_url: Set(json.url),
            first_release_date: Set(json.first_release_date),
            franchise: Set(json.franchise),
            genres: Set(json.genres.map(|genres| genres.join(","))),
            game_modes: Set(json.game_modes.map(|game_modes| game_modes.join(","))),
            supports_online_multiplayer: Set(json.supports_online_multiplayer),
            platforms: Set(json.platforms.map(|platforms| platforms.join(","))),
            // FIXME(Dan): Make not thumbnail
            cover_art_url: Set(json.cover),
            artwork_url: Set(json.artworks),
        };
        let model = model.insert(db).await?;
        Ok(model)
    }

    pub fn make_searchable_name(name: String) -> String {
        name.chars()
            .filter_map(|char| {
                if Self::UNSEARCHABLE_CHARS.contains(&char) {
                    None
                } else {
                    Some(char.to_ascii_lowercase())
                }
            })
            .collect::<String>()
    }
}

impl Model {
    pub fn to_json(self) -> GameDTO {
        GameDTO {
            id: self.id,
            name: self.name,
            summary: self.summary,
            aggregated_rating: self.aggregated_rating,
            themes: self
                .themes
                .map(|themes| themes.split(",").map(String::from).collect()),
            igdb_url: self.igdb_url,
            first_release_date: self.first_release_date,
            franchise: self.franchise,
            genres: self
                .genres
                .map(|themes| themes.split(",").map(String::from).collect()),
            game_modes: self
                .game_modes
                .map(|game_mode| game_mode.split(",").map(String::from).collect()),
            supports_online_multiplayer: self.supports_online_multiplayer,
            platforms: self
                .platforms
                .map(|platform| platform.split(",").map(String::from).collect()),
            artwork_url: self.artwork_url,
            cover_art_url: self.cover_art_url,
        }
    }
}
