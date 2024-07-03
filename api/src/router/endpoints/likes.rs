use crate::router::{error::Result, middleware::auth};
use axum::{
    extract::{Json as ExtractJson, Path, State},
    middleware,
    routing::{delete, get, post},
    Json, Router,
};
use lib::{db::like, entities::Like, store::Store};
use uuid::Uuid;

/// Create a new router for the likes endpoints.
pub fn router(store: Store) -> Router<Store> {
    Router::new()
        .layer(middleware::from_fn_with_state(store.clone(), auth))
        .route("/likes", post(post_like))
        .route("/likes/:username", get(get_by_username))
        .route("/likes/:profile_id/:post_id", delete(delete_like))
        .with_state(store)
}

/// `GET /likes/:username`
async fn get_by_username(
    State(store): State<Store>,
    Path(username): Path<String>,
) -> Result<Json<Vec<Like>>> {
    Ok(Json(like::get_by_username(&store.db, username).await?))
}

/// `POST /likes`
async fn post_like(
    State(store): State<Store>,
    ExtractJson(like): ExtractJson<Like>,
) -> Result<Json<Like>> {
    Ok(Json(
        like::upsert(&store.db, like.profile_id, like.post_id).await?,
    ))
}

/// `DELETE /likes/:profile_id/:post_id`
async fn delete_like(
    State(store): State<Store>,
    Path((profile_id, post_id)): Path<(Uuid, Uuid)>,
) -> Result<()> {
    Ok(like::delete(&store.db, profile_id, post_id).await?)
}
