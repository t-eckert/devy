use crate::date::Date;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Bookmark {
    pub profile_id: Uuid,
    pub post_id: Uuid,
    pub created_at: Date,
}

impl Bookmark {
    pub fn new(profile_id: Uuid, post_id: Uuid) -> Self {
        Self {
            profile_id,
            post_id,
            created_at: Date::now(),
        }
    }
}
