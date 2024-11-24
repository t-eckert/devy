use crate::date::Date;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Identity {
    pub id: Uuid,
    pub user_id: Uuid,
    pub profile_id: Uuid,
    pub username: String,
    pub email: String,

    pub github_auth_enabled: bool,
    pub github_user_id: Option<i64>,

    pub created_at: Date,
    pub updated_at: Date,
}

impl Identity {
    pub fn new(user_id: Uuid, profile_id: Uuid, username: &str, email: &str) -> Self {
        Self {
            id: Uuid::new_v4(),
            user_id,
            profile_id,
            username: username.to_string(),
            email: email.to_string(),
            github_auth_enabled: false,
            github_user_id: None,
            created_at: Date::now(),
            updated_at: Date::now(),
        }
    }

    pub fn enable_github_auth(&mut self, github_user_id: i64) {
        self.github_auth_enabled = true;
        self.github_user_id = Some(github_user_id);
    }
}
