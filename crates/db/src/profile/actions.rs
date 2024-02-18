use crate::{error::Result, Database};
use entities::Profile;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProfileForUpsert {
    pub id: Option<String>,
    pub user_id: Option<String>,

    pub display_name: Option<String>,
    pub avatar_url: Option<String>,
    pub bio: Option<String>,
    pub website: Option<String>,

    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

pub async fn upsert(
    db: &Database,
    id: Option<Uuid>,
    user_id: Option<Uuid>,
    display_name: String,
) -> Result<Profile> {
    Ok(sqlx::query_file_as!(
        Profile,
        "src/profile/queries/upsert.sql",
        id,
        user_id.map(|u| u.to_string()),
        display_name,
    )
    .fetch_one(db)
    .await?)
}

pub async fn get_by_id(db: &Database, id: String) -> Result<Profile> {
    let uuid = Uuid::parse_str(&id).unwrap();

    Ok(
        sqlx::query_file_as!(Profile, "src/profile/queries/get_by_id.sql", uuid)
            .fetch_one(db)
            .await?,
    )
}

pub async fn get_by_username(db: &Database, username: String) -> Result<Profile> {
    Ok(
        sqlx::query_file_as!(Profile, "src/profile/queries/get_by_username.sql", username)
            .fetch_one(db)
            .await?,
    )
}
