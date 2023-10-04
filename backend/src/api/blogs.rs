use crate::{entities::Post, store::Store};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};

pub async fn get_post_by_blog_and_post_slug(
    State(store): State<Store>,
    Path((blog_slug, post_slug)): Path<(String, String)>,
) -> Result<Json<Post>, StatusCode> {
    Ok(Json(
        Post::get_by_blog_and_post_slug(store.pool, blog_slug, post_slug)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
    ))
}
