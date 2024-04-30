use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Blog {
    pub id: Uuid,
    pub profile_id: Uuid,

    pub name: String,
    pub slug: String,
    pub description: Option<String>,

    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}
