use crate::entities::{Profile, User};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Session {
    pub id: Uuid,
    pub user_id: Uuid,

    pub metadata: SessionMetadata,

    pub created_at: Option<String>,
    pub last_used_at: Option<String>,
    pub exp: i32,

    #[serde(skip)]
    pub access_token: String,

    #[serde(skip)]
    pub encoding_key: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionMetadata {
    pub user: User,
    pub profile: Profile,
}

impl From<JsonValue> for SessionMetadata {
    fn from(value: JsonValue) -> Self {
        let user = value["user"].clone();
        let profile = value["profile"].clone();

        Self {
            user: serde_json::from_value(user).unwrap(),
            profile: serde_json::from_value(profile).unwrap(),
        }
    }
}
