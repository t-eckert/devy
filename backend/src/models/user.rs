use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct User {
    pub id: String,
    pub profile_id: String,
    pub email: String,
    pub github_username: Option<String>,
}

impl User {
    pub fn new(id: String, profile_id: String, email: String) -> Self {
        Self {
            id,
            profile_id,
            email,
            github_username: None,
        }
    }

    pub fn with_github_username(mut self, github_username: String) -> Self {
        self.github_username = Some(github_username);
        self
    }
}
