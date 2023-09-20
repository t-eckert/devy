use rocket_db_pools::Connection;

use crate::db::DB;

use super::model::User;

pub struct UserController {}

impl UserController {
    pub async fn insert_user(conn: &mut Connection<DB>, user: User) -> Option<User> {
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
        .fetch_one(&mut **conn)
        .await
        .ok()
    }

    pub async fn get_by_username(conn: &mut Connection<DB>, username: String) -> Option<User> {
        sqlx::query_file_as!(User, "queries/user_get_by_username.sql", username)
            .fetch_one(&mut **conn)
            .await
            .ok()
    }

    pub async fn get_by_github_username(
        conn: &mut Connection<DB>,
        github_username: String,
    ) -> Option<User> {
        sqlx::query_file_as!(
            User,
            "queries/user_get_by_github_username.sql",
            github_username
        )
        .fetch_one(&mut **conn)
        .await
        .ok()
    }
}
