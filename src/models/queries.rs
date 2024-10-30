use chrono::{Duration, Utc};
use sea_orm::{prelude::*, Set};

use super::_entities::queries::{ActiveModel, Entity, Model};

const RECENT_THRESHOLD_WEEKS: i64 = 4;

impl Entity {
    pub async fn create<C: ConnectionTrait>(db: &C, query: String) -> Result<Model, DbErr> {
        let active_model = ActiveModel {
            query: Set(query),
            queried_at: Set(Utc::now()),
        };
        let model = active_model.insert(db).await?;
        Ok(model)
    }

    pub async fn find_or_create<C: ConnectionTrait>(db: &C, query: String) -> Result<Model, DbErr> {
        if let Some(query) = Self::find_by_id(&query).one(db).await? {
            Ok(query)
        } else {
            Ok(Self::create(db, query).await?)
        }
    }
}

impl Model {
    pub fn queried_recently(&self) -> bool {
        Utc::now() - self.queried_at < Duration::weeks(RECENT_THRESHOLD_WEEKS)
    }
}
