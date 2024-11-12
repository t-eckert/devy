use crate::db;
use crate::webhooks::{Webhook, WebhookType};
use serde_json::Value;
use uuid::Uuid;

pub struct WebhookRepository;

impl WebhookRepository {
    pub async fn insert(
        db_conn: &db::Conn,
        webhook_type: WebhookType,
        payload: Value,
    ) -> Result<Uuid, anyhow::Error> {
        Ok(sqlx::query_file_as!(
            db::Id,
            "queries/insert_webhook.sql",
            webhook_type.as_str(),
            payload
        )
        .fetch_one(db_conn)
        .await?
        .id)
    }

    pub async fn get(db_conn: &db::Conn, id: Uuid) -> Result<Webhook, anyhow::Error> {
        Ok(
            sqlx::query_as!(Webhook, "SELECT id, type::TEXT as webhook_type, payload, to_char(received_at, 'YYYY-MM-DDThh:mm:ss.ss') AS received_at FROM webhook WHERE id = $1", id)
                .fetch_one(db_conn)
                .await?,
        )
    }

    pub async fn count(db_conn: &db::Conn) -> Result<i64, anyhow::Error> {
        Ok(sqlx::query!("SELECT COUNT(*) FROM webhook")
            .fetch_one(db_conn)
            .await?
            .count
            .unwrap())
    }
}
