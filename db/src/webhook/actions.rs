use crate::{Database, Result};
use entities::Webhook;
use serde_json::Value;

pub async fn insert(db: &Database, webhook_type: &str, payload: Value) -> Result<Webhook> {
    Ok(sqlx::query_as!(
        Webhook,
        r#"
        INSERT INTO "webhook" (type, payload)
        VALUES ($1, $2)
        RETURNING
            id::TEXT,
            type::TEXT as webhook_type,
            payload,
            to_char(received_at, 'YYYY-MM-DDThh:mm:ss.ss') AS received_at;
        "#,
        webhook_type,
        payload
    )
    .fetch_one(db)
    .await?)
}
