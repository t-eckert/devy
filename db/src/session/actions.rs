use crate::{error::Result, Database};
use entities::SessionMetadata;
use uuid::Uuid;

pub async fn insert(
    db: &Database,
    user_id: Uuid,
    metadata: SessionMetadata,
    token: String,
    exp: i32,
    access_token: String,
    encoding_key: String,
) -> Result<()> {
    sqlx::query_file!(
        "src/session/queries/insert.sql",
        user_id,
        serde_json::to_value(metadata).unwrap_or_default(),
        token,
        exp,
        access_token,
        encoding_key
    )
    .execute(db)
    .await?;

    Ok(())
}
