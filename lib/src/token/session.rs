use crate::entities::{Profile, User};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Session {
    username: String,
    role: String,
    status: String,

    display_name: Option<String>,
    avatar_url: Option<String>,
}

impl Session {
    pub fn new(
        username: String,
        role: String,
        status: String,
        display_name: Option<String>,
        avatar_url: Option<String>,
    ) -> Self {
        Self {
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
