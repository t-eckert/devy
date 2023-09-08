use super::model::User;
use crate::db::DB;
use rocket_db_pools::Connection;

pub struct UserController {}

impl UserController {
    pub async fn create_user(mut db: Connection<DB>, user: User) -> Option<User> {
        sqlx::query_as!(
            User,
            r#"
            INSERT INTO "user" (username, email, github_username)
            VALUES ($1, $2, $3)
            RETURNING 
                id::TEXT,
                to_char("user".created_at, 'YYYY-MM-DDThh:mm:ss.ss') AS created_at,
                to_char("user".updated_at, 'YYYY-MM-DDThh:mm:ss.ss') AS updated_at,
                username, email, github_username, role, status
            ;
            "#,
            user.username,
            user.email,
            user.github_username.unwrap(),
        )
        .fetch_one(&mut *db)
        .await
        .ok()
    }

    pub async fn get_by_github_username(
        mut db: Connection<DB>,
        github_username: String,
    ) -> Option<User> {
        sqlx::query_file_as!(
            User,
            "queries/user_get_by_github_username.sql",
            github_username
        )
        .fetch_one(&mut *db)
        .await
        .ok()
    }
}
