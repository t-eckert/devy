use crate::db::{Database, Result};
use crate::entities::{Webhook,WebhookType};
use serde_json::Value;

pub async fn insert(db: &Database, webhook_type: WebhookType, payload: Value) -> Result<Webhook> {
    Ok(sqlx::query_as!(
        Webhook,
        r#"
        INSERT INTO "webhook" (type, payload)
        VALUES ($1, $2)
        RETURNING
            id,
            type::TEXT as webhook_type,
            payload,
            to_char(received_at, 'YYYY-MM-DDThh:mm:ss.ss') AS received_at;
        "#,
        webhook_type.as_str(),
        payload
    )
    .fetch_one(db)
    .await?)
}

pub async fn count(db: &Database) -> Result<i64> {
    Ok(sqlx::query!(r#"select COUNT(*) from "webhook";"#)
        .fetch_one(db)
        .await?
        .count
        .unwrap())
}
