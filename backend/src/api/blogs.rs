use crate::entities::Post;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use sqlx::PgPool;

pub async fn get_post_by_blog_and_post_slug(
    State(pool): State<PgPool>,
    Path((blog_slug, post_slug)): Path<(String, String)>,
) -> Result<Json<Post>, StatusCode> {
    Ok(Json(
        Post::get_by_blog_and_post_slug(pool, blog_slug, post_slug)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
    ))
}
