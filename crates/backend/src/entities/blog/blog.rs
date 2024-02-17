use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Blog {
    pub id: Uuid,

    /// The profile_id of the user who owns this blog.
    pub profile_id: Uuid,

    pub name: String,
    pub slug: String,
    pub description: Option<String>,

    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct BlogForUpsert {
    pub profile_id: Uuid,

    pub name: String,
    pub slug: String,
    pub description: Option<String>,
}

impl BlogForUpsert {
    pub fn new(name: String, slug: String, profile_id: Uuid) -> Self {
        Self {
            name,
            slug,
            profile_id,
            description: None,
        }
    }

    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }
}
