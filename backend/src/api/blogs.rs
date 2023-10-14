use crate::{
    entities::{error::EntityError, Blog, Post},
    store::Store,
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};

/// Get a blog by its slug.
/// `/blogs/:blog_slug`
pub async fn get_blog_by_blog_slug(
    State(store): State<Store>,
    Path(blog_slug): Path<String>,
) -> Result<Json<Blog>, StatusCode> {
    Ok(Json(
        Blog::get_by_slug(&store.pool, blog_slug)
            .await
            .map_err(map_error)?,
    ))
}

/// Get all posts by a blog slug.
/// `/blogs/:blog_slug/posts`
pub async fn get_blog_posts_by_blog_slug(
    State(store): State<Store>,
    Path(blog_slug): Path<String>,
) -> Result<Json<Vec<Post>>, StatusCode> {
    Ok(Json(
        Post::get_by_blog_slug(&store.pool, blog_slug)
            .await
            .map_err(map_error)?,
    ))
}

/// Get a post by its blog slug and post slug.
/// `/blogs/:blog_slug/posts/:post_slug`
pub async fn get_post_by_blog_and_post_slug(
    State(store): State<Store>,
    Path((blog_slug, post_slug)): Path<(String, String)>,
) -> Result<Json<Post>, StatusCode> {
    Ok(Json(
        Post::get_by_blog_and_post_slug(&store.pool, blog_slug, post_slug)
            .await
            .map_err(map_error)?,
    ))
}

pub async fn upsert_blog(
    State(store): State<Store>,
    Json(blog): Json<Blog>,
) -> Result<Json<Blog>, StatusCode> {
    Ok(Json(blog.upsert(&store.pool).await.map_err(map_error)?))
}

pub async fn delete_blog(
    State(store): State<Store>,
    Path(blog_slug): Path<String>,
) -> Result<StatusCode, StatusCode> {
    Blog::delete_by_slug(&store.pool, blog_slug)
        .await
        .map_err(map_error)?;

    Ok(StatusCode::OK)
}

fn map_error(e: EntityError) -> StatusCode {
    match e {
        EntityError::NotFound => StatusCode::NOT_FOUND,
        EntityError::Malformed { .. } => StatusCode::BAD_REQUEST,
        EntityError::Sqlx(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
