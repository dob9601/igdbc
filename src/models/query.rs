use chrono::Utc;
use sea_orm::{prelude::*, Set};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "queries")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub query: String,

    pub queried_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::model::Entity")]
    Game,
}

impl Related<super::model::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Game.def()
    }
}

impl ActiveModel {
    pub async fn create(db: &DatabaseConnection, query: String) -> Result<Model, DbErr> {
        let active_model = Self {
            query: Set(query),
            queried_at: Set(Utc::now()),
        };
        let model = active_model.insert(db).await?;
        Ok(model)
    }
}

impl ActiveModelBehavior for ActiveModel {}
