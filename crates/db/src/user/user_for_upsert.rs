use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserForUpsert {
    pub username: String,
    pub email: Option<String>,
    pub github_username: Option<String>,
    pub role: Option<String>,
    pub status: Option<String>,
}

impl UserForUpsert {
    pub fn new(username: String, email: Option<String>) -> Self {
        Self {
            username,
            email,
            github_username: None,
            role: None,
            status: None,
        }
    }

    pub fn with_github_username(mut self, github_username: String) -> Self {
        self.github_username = Some(github_username);
        self
    }
}
