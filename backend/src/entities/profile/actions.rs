use super::*;
use crate::auth::GitHubUser;
use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;
use sqlx::PgPool;

pub async fn upsert(pool: &PgPool, profile: ProfileForUpsert) -> Result<Profile> {
    Ok(sqlx::query_file_as!(
        Profile,
        "src/entities/profile/queries/upsert.sql",
        Uuid::parse_str(&profile.user_id.unwrap()).ok(),
        profile.display_name.unwrap(),
        profile.avatar_url
    )
    .fetch_one(pool)
    .await?)
}

pub async fn get_by_id(pool: &PgPool, id: String) -> Result<Profile> {
    let uuid = Uuid::parse_str(&id).unwrap();

    Ok(
        sqlx::query_file_as!(Profile, "src/entities/profile/queries/get_by_id.sql", uuid)
            .fetch_one(pool)
            .await?,
    )
}

pub async fn get_by_username(pool: &PgPool, username: String) -> Result<Profile> {
    Ok(sqlx::query_file_as!(
        Profile,
        "src/entities/profile/queries/get_by_username.sql",
        username
    )
    .fetch_one(pool)
    .await?)
}

pub async fn upsert_from_github_user(
    pool: &PgPool,
    user_id: String,
    github_user: GitHubUser,
) -> Result<Profile> {
    upsert(
        pool,
        ProfileForUpsert::new(
            user_id,
            github_user
                .name
                .ok_or(Error::Malformed("GitHub user missing name".to_string()))?,
            github_user.avatar_url,
        ),
    )
    .await
}
