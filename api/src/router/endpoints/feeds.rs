use crate::router::error::Result;
use axum::{extract::State, routing::get, Json};
use lib::{db::feed, entities::Feed, store::Store};

pub fn router(store: Store) -> axum::Router<Store> {
    axum::Router::new()
        .route("/feeds/recent", get(get_recent))
        .route("/feeds/popular", get(get_popular))
        .with_state(store)
}

// GET /feeds/recent
async fn get_recent(State(store): State<Store>) -> Result<Json<Feed>> {
    Ok(Json(feed::get_recent(&store.db).await?))
}

// GET /feeds/popular
async fn get_popular(State(store): State<Store>) -> Result<Json<Feed>> {
    Ok(Json(feed::get_popular(&store.db).await?))
}
