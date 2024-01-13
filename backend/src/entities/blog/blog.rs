use crate::entities::error::Result;
use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;
use sqlx::PgPool;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Blog {
    pub id: Uuid,

    pub name: String,
    pub slug: String,

    pub username: String,
    pub display_name: Option<String>,
    pub description: Option<String>,

    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlogForUpsert {
    pub name: String,
    pub slug: String,

    pub username: String,
    pub display_name: Option<String>,
    pub description: Option<String>,

    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

impl BlogForUpsert {
    pub fn new(name: String, slug: String, username: String) -> Self {
        Self {
            name,
            slug,
            username,
            display_name: None,
            description: None,
            created_at: None,
            updated_at: None,
        }
    }
}
