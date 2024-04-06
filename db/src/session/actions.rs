use crate::{error::Result, Database};
use entities::{Session, SessionMetadata};
use uuid::Uuid;

pub async fn insert(
    db: &Database,
    user_id: Uuid,
    metadata: SessionMetadata,
    token: String,
    access_token: String,
    encoding_key: String,
) -> Result<Session> {
    Ok(sqlx::query_file_as!(
        Session,
        "src/session/queries/insert.sql",
        user_id,
        serde_json::to_value(metadata).unwrap_or_default(),
        token,
        access_token,
        encoding_key
    )
    .fetch_one(db)
    .await?)
}