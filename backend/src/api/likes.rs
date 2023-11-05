use crate::{entities::Like, store::Store};
use axum::{
    extract::{Json as ExtractJson, Path, State},
    http::StatusCode,
    Json,
};

pub async fn post_like(
    State(store): State<Store>,
    ExtractJson(like): ExtractJson<Like>,
) -> Result<Json<Like>, StatusCode> {
    Ok(Json(
        like.upsert(&store.pool)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
    ))
}

pub async fn delete_like(
    State(store): State<Store>,
    Path((post_id, profile_id)): Path<(String, String)>,
) -> Result<Json<Like>, StatusCode> {
    let like = Like::new(profile_id, post_id);
    Ok(Json(
        like.delete(&store.pool)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
    ))
}
