use crate::date::Date;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: Uuid,

    pub username: String,
    pub email: Option<String>,
    pub github_username: Option<String>,
    pub role: String,
    pub status: String,

    pub created_at: Date,
    pub updated_at: Date,
}

impl User {
    pub fn set_role(self, role: String) -> Self {
        Self { role, ..self }
    }

    pub fn set_status(self, status: String) -> Self {
        Self { status, ..self }
    }
}

pub struct UserRepo;

impl UserRepo {
    pub async fn insert() {}

    pub async fn update() {}

    pub async fn get() {}

    pub async fn get_by_username() {}

    pub async fn get_all() {}

    pub async fn count() {}

    pub async fn delete() {}
}
