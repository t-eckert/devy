use crate::entities::{Profile, User};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Session {
    pub user_id: Uuid,
    pub profile_id: Uuid,
    pub username: String,
    role: String,
    status: String,

    display_name: Option<String>,
    avatar_url: Option<String>,
}

impl Session {
    pub fn new(
        user_id: Uuid,
        profile_id: Uuid,
        username: String,
        role: String,
        status: String,
        display_name: Option<String>,
        avatar_url: Option<String>,
    ) -> Self {
        Self {
            user_id,
            profile_id,
            username,
            role,
            status,
            display_name,
            avatar_url,
        }
    }
}

impl From<Value> for Session {
    fn from(value: Value) -> Self {
        Self {
            user_id: Uuid::parse_str(value["user_id"].as_str().unwrap()).unwrap(),
            profile_id: Uuid::parse_str(value["profile_id"].as_str().unwrap()).unwrap(),
            username: value["username"].as_str().unwrap().to_string(),
            role: value["role"].as_str().unwrap().to_string(),
            status: value["status"].as_str().unwrap().to_string(),
            display_name: match value["display_name"] {
                Value::Null => None,
                _ => value["display_name"].as_str().map(|s| s.to_string()),
            },
            avatar_url: match value["avatar_url"] {
                Value::Null => None,
                _ => value["avatar_url"].as_str().map(|s| s.to_string()),
            },
        }
    }
}
