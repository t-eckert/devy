use super::error::Result;
use crate::{
    entities::{like, Like},
    store::Store,
};
use axum::{
    extract::{Json as ExtractJson, Path, State},
    routing::{delete, post},
    Json,
};
use uuid::Uuid;

pub fn make_router(store: Store) -> axum::Router<Store> {
    axum::Router::new()
        .route("/likes", post(post_like))
        .route("/likes/:post_id/:profile_id", delete(delete_like))
        .with_state(store)
}

async fn get_ids_by_username(
    State(store): State<Store>,
    Path(username): Path<String>,
) -> Result<Json<Vec<Like>>> {
    Ok(Json(like::get_by_username(&store.pool, username).await?))
}

async fn post_like(
    State(store): State<Store>,
    ExtractJson(like): ExtractJson<like::LikeForUpsert>,
) -> Result<Json<Like>> {
    Ok(Json(like::upsert(&store.pool, like).await?))
}

async fn delete_like(
    State(store): State<Store>,
    Path((post_id, profile_id)): Path<(Uuid, Uuid)>,
) -> Result<()> {
    Ok(like::delete(&store.pool, profile_id, post_id).await?)
}
