use super::error::Result;
use crate::entities::{post, Post};
use crate::store::Store;
use axum::{
    extract::{Path, State},
    Json,
};
use uuid::Uuid;

/// Get a post
pub async fn get_by_post_id(
    State(store): State<Store>,
    Path(post_id): Path<Uuid>,
) -> Result<Json<Post>> {
    Ok(Json(post::get_by_id(&store.pool, post_id).await?))
}

pub async fn get_by_blog_slug(
    State(store): State<Store>,
    Path(blog_slug): Path<String>,
) -> Result<Json<Vec<Post>>> {
    Ok(Json(post::get_by_blog_slug(&store.pool, &blog_slug).await?))
}

pub async fn get_posts_by_username(
    State(store): State<Store>,
    Path(username): Path<String>,
) -> Result<Json<Vec<Post>>> {
    Ok(Json(post::get_by_username(&store.pool, &username).await?))
}

pub async fn get_liked_by_username(
    State(store): State<Store>,
    Path(username): Path<String>,
) -> Result<Json<Vec<Post>>> {
    Ok(Json(
        post::get_liked_by_username(&store.pool, &username).await?,
    ))
}
