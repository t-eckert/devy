use rocket::serde::{Deserialize, Serialize};
use rocket_db_pools::Connection;
use sqlx;

use crate::db::DB;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
#[serde(crate = "rocket::serde")]
pub struct User {
    #[serde(skip)]
    pub id: i32,
    pub profile_id: Option<i32>,
    pub username: String,
    pub email: Option<String>,
    pub github_username: Option<String>,
}

impl User {
    // pub async fn upsert(mut db: Connection<DB>, user: Self) -> Result<User, sqlx::Error> {
    //     let user = sqlx::query_as!(
    //         User,
    //         r#"
    //         INSERT INTO "user" (username, email, github_username)
    //         VALUES ($1, $2, $3)
    //         ON CONFLICT (username) DO UPDATE SET
    //             email = $2,
    //             github_username = $3
    //         RETURNING *;
    //         "#,
    //         user.username,
    //         user.email,
    //         user.github_username
    //     )
    //     .fetch_one(&mut *db)
    //     .await?;

    //     return Ok(user);
    // }

    // pub async fn get_by_username(
    //     mut db: Connection<DB>,
    //     username: String,
    // ) -> Result<Self, sqlx::Error> {
    //     let user = sqlx::query_as!(
    //         Self,
    //         r#"SELECT * FROM "user" WHERE username = $1;"#,
    //         username
    //     )
    //     .fetch_one(&mut *db)
    //     .await?;

    //     return Ok(user);
    // }
}
