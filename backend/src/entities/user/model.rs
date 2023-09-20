use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: Option<String>,

    pub created_at: Option<String>,
    pub updated_at: Option<String>,

    pub username: String,
    pub email: String,
    pub github_username: Option<String>,
    pub role: String,
    pub status: String,
}

impl User {
    pub fn new(username: String, email: String, github_username: String) -> Self {
        User {
            id: None,
            created_at: None,
            updated_at: None,
            username,
            email,
            github_username: Some(github_username),
            role: "user".to_string(),
            status: "active".to_string(),
        }
    }
}
