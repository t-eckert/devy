use crate::router::error::Result;
use crate::store::Store;
use axum::{
    extract::{Query, State},
    routing::get,
    Router,
};
use serde::Deserialize;

pub fn router(store: Store) -> Router<Store> {
    return Router::new()
        .route("/search", get(search))
        .with_state(store);
}

#[derive(Deserialize)]
struct SearchQuery {
    query: String,
}

async fn search(State(store): State<Store>, searchQuery: Query<SearchQuery>) -> Result<String> {
    Ok(searchQuery.query.clone())
}
