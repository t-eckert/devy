use super::*;
use crate::entities::{upload, webhook, Webhook};
use crate::store::Store;
use axum::{extract::State, Json};
use http::{HeaderMap, StatusCode};
use serde_json::Value;

pub async fn insert(
    State(store): State<Store>,
    headers: HeaderMap,
    Json(payload): Json<Value>,
) -> Result<http::StatusCode> {
    // This will only vibe with GitHub headers for now, but that's future problem.
    let event = format!(
        "webhook.github.{}",
        headers
            .get("x-github-event")
            .map(|header_value| header_value.to_str().unwrap_or_default())
            .unwrap_or_default()
    );

    let webhook = match Webhook::new(event, payload).insert(&store.pool).await {
        Ok(webhook) => webhook,
        Err(e) => return Err(e.into()),
    };

    match webhook.webhook_type {
        webhook::WebhookType::GitHubPushEvent => {
            let upload =
                upload::insert(&store.pool, upload::UploadForUpsert::try_from(webhook)?).await?;
            let _ = store.uploader.upload(upload, &store.pool).await?;
        }
        webhook::WebhookType::Generic => {}
    }

    Ok(StatusCode::OK)
}
