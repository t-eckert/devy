use crate::date::Date;
use crate::db;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Blog {
    pub id: Uuid,
    pub profile_id: Uuid,
    pub user_id: Uuid,

    pub author_username: String,
    pub author_display_name: Option<String>,

    pub name: String,
    pub slug: String,
    pub description: Option<String>,

    pub created_at: Date,
    pub updated_at: Date,
}
