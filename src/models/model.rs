use super::deserializers::*;
use crate::models::QueryModel;
use log::info;
use sea_orm::{prelude::*, Set};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "games")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: u32,

    pub query_id: String,

    pub name: String,
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
}

impl Model {
    pub fn to_json(&self) -> GameJson {
        GameJson {
            id: self.id,
            name: self.name.clone(),
            summary: self.summary.clone(),
            aggregated_rating: self.aggregated_rating,
            themes: self.themes.clone().map(|themes| {
                themes
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|value| value.as_str().unwrap().to_string())
                    .collect()
            }),
            url: self.url.clone(),
            first_release_date: self.first_release_date,
            franchise: self.franchise.clone(),
            genres: self.genres.clone().map(|themes| {
                themes
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|value| value.as_str().unwrap().to_string())
                    .collect()
            }),
            game_modes: self.game_modes.clone().map(|game_mode| {
                game_mode
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|value| value.as_str().unwrap().to_string())
                    .collect()
            }),
            supports_online_multiplayer: self.supports_online_multiplayer,
            platforms: self.platforms.clone().map(|platform| {
                platform
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|value| value.as_str().unwrap().to_string())
                    .collect()
            }),
        }
    }
}

impl ActiveModel {
    pub async fn create(
        db: &DatabaseConnection,
        json: GameJson,
        query: &QueryModel,
    ) -> Result<Model, DbErr> {

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
        };
        let model = model.insert(db).await?;
        Ok(model)
    }

    pub async fn create_or_update(
        db: &DatabaseConnection,
        json: GameJson,
        query: &QueryModel,
    ) -> Result<(), DbErr> {
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

#[derive(Serialize, Deserialize, Clone)]
pub struct GameJson {
    pub id: u32,

    pub name: String,
    pub summary: Option<String>,
    pub aggregated_rating: Option<f32>,

    #[serde(deserialize_with = "deserialize_themes", default)]
    pub themes: Option<Vec<String>>,

    pub url: String,

    //pub artwork: Vec<u8>, Requires a CDN, defer
    //pub cover_art: Vec<u8>,
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
