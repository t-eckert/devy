use axum::Router;

use crate::store::Store;

use super::error::Result;
use crate::entities::{blog, like, profile, Blog, Like, Profile};
use crate::entities::{post, Post};
use axum::{
    extract::{Path, State},
    routing::get,
    Json,
};
use uuid::Uuid;

pub fn make_router(store: Store) -> Router<Store> {
    Router::new()
        .route("/profiles/:username", get(get_profile_by_username))
        .route("/profiles/:username/blogs", get(get_blogs_by_username))
        .route("/profiles/:username/posts", get(get_posts_by_username))
        .route("/profiles/:username/likes", get(get_liked_by_username))
        .route("/profiles/:username/likes/ids", get(get_ids_by_username))
        .with_state(store)
}

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

/// Get a profile by username.
pub async fn get_profile_by_username(
    State(store): State<Store>,
    Path(username): Path<String>,
) -> Result<Json<Profile>> {
    Ok(Json(profile::get_by_username(&store.pool, username).await?))
}

/// Get all blogs for a profile by username.
/// GET /profiles/:username/blogs
pub async fn get_blogs_by_username(
    State(store): State<Store>,
    Path(username): Path<String>,
) -> Result<Json<Vec<Blog>>> {
    Ok(Json(blog::get_by_username(&store.pool, username).await?))
}

pub async fn get_ids_by_username(
    State(store): State<Store>,
    Path(username): Path<String>,
) -> Result<Json<Vec<Like>>> {
    Ok(Json(like::get_by_username(&store.pool, username).await?))
}
