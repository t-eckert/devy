use super::error::Result;
use crate::{
    entities::{user, User},
    store::Store,
};
use axum::Router;
use axum::{
    extract::{Path, State},
    routing::get,
    Json,
};

pub fn make_router(store: Store) -> Router<Store> {
    Router::new()
        .route("/users/:username", get(get_by_username))
        .with_state(store)
}

pub async fn get_by_username(
    State(store): State<Store>,
    Path(username): Path<String>,
) -> Result<Json<User>> {
    Ok(Json(user::get_by_username(&store.pool, &username).await?))
}
