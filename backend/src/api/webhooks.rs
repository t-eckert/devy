use super::error::Result;
use crate::{
    entities::{Upload, Webhook, WebhookType},
    store::Store,
};
use axum::{extract::State, Json};
use http::{HeaderMap, StatusCode};
use serde_json::Value;

pub async fn handle_webhook(
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
        WebhookType::GitHubPushEvent => {
            let upload = Upload::try_from(webhook)?.insert(&store.pool).await?;
            let _ = store.uploader.upload(upload, &store.pool).await?;
        }
        WebhookType::Generic => {}
    }

    Ok(StatusCode::OK)
}
