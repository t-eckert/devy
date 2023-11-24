use super::error::Result;
use crate::{entities::Like, store::Store};
use axum::{
    extract::{Json as ExtractJson, Path, State},
    Json,
};

pub async fn get_ids_by_username(
    State(store): State<Store>,
    Path(username): Path<String>,
) -> Result<Json<Vec<String>>> {
    Ok(Json(
        Like::get_post_ids_by_username(&store.pool, username).await?,
    ))
}

pub async fn post_like(
    State(store): State<Store>,
    ExtractJson(like): ExtractJson<Like>,
) -> Result<Json<Like>> {
    Ok(Json(like.upsert(&store.pool).await?))
}

pub async fn delete_like(
    State(store): State<Store>,
    Path((post_id, profile_id)): Path<(String, String)>,
) -> Result<Json<Like>> {
    let like = Like::new(profile_id, post_id);
    Ok(Json(like.delete(&store.pool).await?))
}
