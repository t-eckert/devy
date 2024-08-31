use crate::controllers::WebhooksController;
use crate::router::error::Result;
use crate::store::Store;
use axum::Router;
use axum::{extract::State, routing::post, Json};
use http::{HeaderMap, StatusCode};
use serde_json::Value;
use std::collections::HashMap;

pub fn router(store: Store) -> Router<Store> {
    Router::new()
        .route("/webhooks", post(receive))
        .with_state(store)
}

async fn receive(
    State(store): State<Store>,
    headers: HeaderMap,
    Json(payload): Json<Value>,
) -> Result<StatusCode> {
    let header_hashmap: HashMap<String, String> = headers
        .iter()
        .map(|(key, value)| {
            (
                key.to_string(),
                value.to_str().unwrap_or_default().to_string(),
            )
        })
        .collect();

    let _ = WebhooksController::handle(store, header_hashmap, payload).await?;

    Ok(StatusCode::ACCEPTED)
}
