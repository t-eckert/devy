use super::error::Result;
use crate::{
    entities::{Blog, BlogRepository, Post, PostRepository, Profile},
    store::Store,
};
use axum::{
    extract::{Path, State},
    Json,
};

/// Get a profile by username.
pub async fn get_profile_by_username(
    State(store): State<Store>,
    Path(username): Path<String>,
) -> Result<Json<Profile>> {
    Ok(Json(Profile::get_by_username(&store.pool, username).await?))
}

/// Get all blogs for a profile by username.
/// GET /profiles/:username/blogs
pub async fn get_blogs_by_username(
    State(store): State<Store>,
    Path(username): Path<String>,
) -> Result<Json<Vec<Blog>>> {
    Ok(Json(
        BlogRepository::get_by_username(&store.pool, username).await?,
    ))
}

/// Get all posts for a profile by username.
/// GET /profiles/:username/posts
pub async fn get_posts_by_username(
    State(store): State<Store>,
    Path(username): Path<String>,
) -> Result<Json<Vec<Post>>> {
    Ok(Json(
        PostRepository::get_by_username(&store.pool, &username).await?,
    ))
}
