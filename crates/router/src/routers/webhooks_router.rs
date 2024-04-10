use crate::error::Result;
use axum::Router;
use axum::{extract::State, routing::post, Json};
use db::{upload, webhook};
use entities::WebhookType;
use http::{HeaderMap, StatusCode};
use serde_json::Value;
use store::Store;

pub struct WebhooksRouter;

impl WebhooksRouter {
    pub fn create(store: Store) -> Router<Store> {
        Router::new()
            .route("/webhooks", post(receive))
            .with_state(store)
    }
}

async fn receive(
    State(store): State<Store>,
    headers: HeaderMap,
    Json(payload): Json<Value>,
) -> Result<StatusCode> {
    dbg!(&payload);

    // This will only vibe with GitHub headers for now, but that's future problem.
    // TODO break this out into a separate crate called `webhooks`.
    let event = format!(
        "webhook.github.{}",
        headers
            .get("x-github-event")
            .map(|header_value| header_value.to_str().unwrap_or_default())
            .unwrap_or_default()
    );
    dbg!(&event);

    let webhook_type = WebhookType::from(event);
    dbg!(&webhook_type);
    let webhook = webhook::insert(&store.db, webhook_type, payload).await?;
    dbg!(&webhook);

    match webhook.webhook_type {
        WebhookType::GitHubPushEvent => {
            let repo = format!(
                "https://api.github.com/repos/{}",
                webhook.payload["repository"]["full_name"]
                    .as_str()
                    .unwrap_or_default()
                    .replace(r#"""#, "")
            );
            dbg!(&repo);
            let upload = upload::insert(&store.db, None, repo).await?;
            dbg!(&upload);
            let upload = store.uploader.upload(&store.db, upload).await?;
            dbg!(&upload);
        }
        _ => {}
    }

    Ok(StatusCode::OK)
}
