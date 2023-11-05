use crate::{
    entities::{Like, Post},
    store::Store,
};
use axum::{
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    Json,
};

pub async fn get_post_by_post_id(
    State(store): State<Store>,
    Path(post_id): Path<String>,
) -> Result<Json<Post>, StatusCode> {
    Ok(Json(
        Post::get_by_id(&store.pool, post_id)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
    ))
}
