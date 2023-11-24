use crate::{
    entities::{Blog, Like, Post, Profile},
    store::Store,
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};

pub async fn get_by_username(
    State(store): State<Store>,
    Path(username): Path<String>,
) -> Result<Json<Profile>, StatusCode> {
    Ok(Json(
        Profile::get_by_username(&store.pool, username)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
    ))
}

/// Get all blogs for a user by their username.
/// `/profiles/:username/blogs`
pub async fn get_blog_by_username(
    State(store): State<Store>,
    Path(username): Path<String>,
) -> Result<Json<Vec<Blog>>, StatusCode> {
    Ok(Json(
        Blog::get_by_username(&store.pool, username)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
    ))
}

pub async fn get_posts_by_username(
    State(store): State<Store>,
    Path(username): Path<String>,
) -> Result<Json<Vec<Post>>, StatusCode> {
    Ok(Json(
        Post::get_by_username(&store.pool, username)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
    ))
}

pub async fn get_liked_posts_by_username(
    State(store): State<Store>,
    Path(username): Path<String>,
) -> Result<Json<Vec<Post>>, StatusCode> {
    Ok(Json(
        Post::get_liked_by_username(&store.pool, username)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
    ))
}

pub async fn get_likes_ids_by_username(
    State(store): State<Store>,
    Path(username): Path<String>,
) -> Result<Json<Vec<String>>, StatusCode> {
    Ok(Json(
        Like::get_post_ids_by_username(&store.pool, username)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
    ))
}
