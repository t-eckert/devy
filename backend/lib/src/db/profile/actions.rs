use crate::db::{error::Result, Database};
use crate::entities::Profile;
use uuid::Uuid;

async fn upsert(
    db: &Database,
    user_id: Uuid,
    display_name: String,
    avatar_url: Option<String>,
) -> Result<Profile> {
    Ok(sqlx::query_file_as!(
        Profile,
        "src/db/profile/queries/upsert.sql",
        user_id,
        display_name,
        avatar_url
    )
    .fetch_one(db)
    .await?)
}

pub async fn get_by_username(db: &Database, username: String) -> Result<Profile> {
    Ok(sqlx::query_file_as!(
        Profile,
        "src/db/profile/queries/get_by_username.sql",
        username
    )
    .fetch_one(db)
    .await?)
}
