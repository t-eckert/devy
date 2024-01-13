use crate::entities::error::Result;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FeedConfig {
    pub id: Uuid,
    pub name: String,
}

impl FeedConfig {
    pub fn new(id: Uuid, name: String) -> Self {
        Self { id, name }
    }
}
