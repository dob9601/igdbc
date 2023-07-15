use super::deserializers::*;
use crate::models::QueryModel;
use schemars::JsonSchema;
use sea_orm::{prelude::*, ConnectionTrait, QuerySelect, Set};
use serde::{Deserialize, Serialize};
use serde_json::json;
use shared::models::GameJson;
use tracing::info;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "games")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: u32,
    pub query_id: String,
    pub name: String,
    #[sea_orm(column_type = "Text", nullable)]
    pub summary: Option<String>,
    pub aggregated_rating: Option<f32>,
    pub themes: Option<Json>,
    pub url: String,
    pub first_release_date: Option<DateTimeUtc>,
    pub franchise: Option<String>,
    pub genres: Option<Json>,
    pub game_modes: Option<Json>,
    pub supports_online_multiplayer: Option<bool>,
    pub platforms: Option<Json>,

    pub cover_art_url: Option<String>,
    pub artwork_url: Option<String>,
}

impl Entity {
    pub async fn find_by_query<C>(db: &C, query: &str) -> Result<Vec<Model>, DbErr>
    where
        C: ConnectionTrait,
    {
        Self::find()
            .filter(Column::Name.like(&format!("{query}%")))
            .limit(10)
            .all(db)
            .await
    }
}

impl Model {
    pub fn to_json(self) -> GameJson {
        GameJson {
            id: self.id,
            name: self.name,
            summary: self.summary,
            aggregated_rating: self.aggregated_rating,
            themes: self.themes.map(|themes| {
                themes
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|value| value.as_str().unwrap().to_string())
                    .collect()
            }),
            url: self.url,
            first_release_date: self.first_release_date,
            franchise: self.franchise,
            genres: self.genres.map(|themes| {
                themes
                    .as_array()
                    .unwrap()
                    .into_iter()
                    .map(|value| value.as_str().unwrap().to_string())
                    .collect()
            }),
            game_modes: self.game_modes.map(|game_mode| {
                game_mode
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|value| value.as_str().unwrap().to_string())
                    .collect()
            }),
            supports_online_multiplayer: self.supports_online_multiplayer,
            platforms: self.platforms.map(|platform| {
                platform
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|value| value.as_str().unwrap().to_string())
                    .collect()
            }),
            artwork_url: self.artwork_url,
            cover_art_url: self.cover_art_url,
        }
    }
}

impl ActiveModel {
    pub async fn create<C>(db: &C, json: GameJson, query: &QueryModel) -> Result<Model, DbErr>
    where
        C: ConnectionTrait,
    {
        let model = Self {
            id: Set(json.id),
            name: Set(json.name),
            summary: Set(json.summary),
            aggregated_rating: Set(json.aggregated_rating),
            themes: Set(json.themes.map(|themes| json!(themes))),
            url: Set(json.url),
            first_release_date: Set(json.first_release_date),
            franchise: Set(json.franchise),
            genres: Set(json.genres.map(|genres| json!(genres))),
            game_modes: Set(json.game_modes.map(|game_modes| json!(game_modes))),
            supports_online_multiplayer: Set(json.supports_online_multiplayer),
            platforms: Set(json.platforms.map(|platforms| json!(platforms))),
            query_id: Set(query.query.clone()),
            cover_art_url: Set(json.cover_art_url),
            artwork_url: Set(json.artwork_url),
        };
        let model = model.insert(db).await?;
        Ok(model)
    }

    pub async fn create_or_update<C>(
        db: &C,
        json: GameJson,
        query: &QueryModel,
    ) -> Result<(), DbErr>
    where
        C: ConnectionTrait,
    {
        info!("Creating/Updating game '{}'", json.name);
        let maybe_model = Entity::find_by_id(json.id).one(db).await?;

        if let Some(model) = maybe_model {
            info!("Game '{}' already exists - updating", json.name);
            let mut active_model: ActiveModel = model.into();
            active_model.id = Set(json.id);
            active_model.name = Set(json.name);
            active_model.summary = Set(json.summary);
            active_model.aggregated_rating = Set(json.aggregated_rating);
            active_model.themes = Set(json.themes.map(|themes| json!(themes)));
            active_model.url = Set(json.url);
            active_model.first_release_date = Set(json.first_release_date);
            active_model.franchise = Set(json.franchise);
            active_model.genres = Set(json.genres.map(|genres| json!(genres)));
            active_model.game_modes = Set(json.game_modes.map(|game_modes| json!(game_modes)));
            active_model.supports_online_multiplayer = Set(json.supports_online_multiplayer);
            active_model.platforms = Set(json.platforms.map(|platforms| json!(platforms)));
            active_model.save(db).await?;
        } else {
            info!("Game '{}' does not exist - creating", json.name);
            Self::create(db, json, query).await?;
        }
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Clone, JsonSchema)]
pub struct IGDBGame {
    pub id: u32,

    pub name: String,
    pub summary: Option<String>,
    pub aggregated_rating: Option<f32>,

    #[serde(deserialize_with = "deserialize_themes", default)]
    pub themes: Option<Vec<String>>,

    pub url: String,

    #[serde(deserialize_with = "deserialize_artworks", default)]
    pub artworks: Option<String>,

    #[serde(deserialize_with = "deserialize_cover", default)]
    pub cover: Option<String>,

    #[serde(deserialize_with = "deserialize_unix_timestamp", default)]
    pub first_release_date: Option<DateTimeUtc>,

    #[serde(deserialize_with = "deserialize_franchise", default)]
    pub franchise: Option<String>,

    #[serde(deserialize_with = "deserialize_genres", default)]
    pub genres: Option<Vec<String>>,

    #[serde(deserialize_with = "deserialize_game_modes", default)]
    pub game_modes: Option<Vec<String>>,

    #[serde(
        deserialize_with = "deserialize_supports_online_multiplayer",
        rename(deserialize = "multiplayer_modes"),
        default
    )]
    pub supports_online_multiplayer: Option<bool>,

    #[serde(deserialize_with = "deserialize_platforms", default)]
    pub platforms: Option<Vec<String>>,
}

impl From<IGDBGame> for GameJson {
    fn from(game: IGDBGame) -> Self {
        Self {
            id: game.id,

            name: game.name,
            summary: game.summary,
            aggregated_rating: game.aggregated_rating,

            themes: game.themes,

            url: game.url,

            first_release_date: game.first_release_date,

            franchise: game.franchise,

            genres: game.genres,

            game_modes: game.game_modes,

            supports_online_multiplayer: game.supports_online_multiplayer,

            platforms: game.platforms,
            artwork_url: game
                .artworks
                .map(|url| format!("https:{}", url.replace("t_thumb", "t_1080p"))),
            cover_art_url: game
                .cover
                .map(|url| format!("https:{}", url.replace("t_thumb", "t_1080p"))),
        }
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::query::Entity",
        from = "Column::QueryId",
        to = "super::query::Column::Query"
    )]
    Query,
}

impl Related<super::query::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Query.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
