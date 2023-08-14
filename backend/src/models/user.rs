use rocket::serde::{Deserialize, Serialize};
use rocket_db_pools::Connection;
use sqlx;
use sqlx::types::Uuid;

use crate::db::DB;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
#[serde(crate = "rocket::serde")]
pub struct User {
    pub id: Option<Uuid>,
    pub profile_id: String,
    pub email: Option<String>,
    pub github_username: Option<String>,
}

impl User {
    pub fn new(id: String, profile_id: String, email: String) -> Self {
        Self {
            id: Some(id),
            profile_id,
            email: Some(email),
            github_username: None,
        }
    }

    pub fn with_github_username(mut self, github_username: String) -> Self {
        self.github_username = Some(github_username);
        self
    }

    pub fn get_by_id(mut db: Connection<DB>, id: String) -> Option<Self> {
        println!("id: {}", id);

        let user = sqlx::query_as!(Self, "SELECT * FROM \"user\";").fetch(&mut *db);

        None
    }
}
