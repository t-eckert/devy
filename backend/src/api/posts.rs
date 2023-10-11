use crate::{entities::Like, store::Store};
use axum::{
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    Json,
};

pub async fn post_like_to_post(
    headers: HeaderMap,
    State(store): State<Store>,
    Path(post_id): Path<String>,
) -> Result<Json<Like>, StatusCode> {
    let profile_id = headers
        .get("profile_id")
        .ok_or(StatusCode::BAD_REQUEST)?
        .to_str()
        .map_err(|_| StatusCode::BAD_REQUEST)?
        .to_string();

    Ok(Json(
        Like::new(profile_id, post_id)
            .upsert(&store.pool)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
    ))
}
