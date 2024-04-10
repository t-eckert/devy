use crate::error::Result;
use entities::{Webhook, WebhookType};
use serde_json::Value;

use crate::Database;

pub async fn insert(db: &Database, webhook_type: WebhookType, payload: Value) -> Result<Webhook> {
    Ok(sqlx::query_file_as!(
        Webhook,
        "src/webhook/queries/insert.sql",
        webhook_type.as_str(),
        payload
    )
    .fetch_one(db)
    .await?)
}
