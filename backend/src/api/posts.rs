use super::error::Result;
use crate::{entities::Post, store::Store};
use axum::{
    extract::{Path, State},
    Json,
};

pub async fn get_post_by_post_id(
    State(store): State<Store>,
    Path(post_id): Path<String>,
) -> Result<Json<Post>> {
    Ok(Json(Post::get_by_id(&store.pool, post_id).await?))
}
