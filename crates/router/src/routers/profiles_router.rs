use crate::error::Result;
use axum::{
    extract::{Path, State},
    routing::get,
    Json, Router,
};
use db::{blog, like, post, profile};
use entities::{Blog, Like, Post, Profile};
use store::Store;
use uuid::Uuid;

pub struct ProfilesRouter;

impl ProfilesRouter {
    pub fn create(store: Store) -> Router<Store> {
        Router::new()
            .route("/profiles/:username", get(get_profile_by_username))
            .route("/profiles/:username/blogs", get(get_blogs_by_username))
            .route("/profiles/:username/posts", get(get_posts_by_username))
            .with_state(store)
    }
}

/// Get a post
async fn get_by_post_id(
    State(store): State<Store>,
    Path(post_id): Path<Uuid>,
) -> Result<Json<Post>> {
    Ok(Json(post::get_by_id(&store.db, post_id).await?))
}

async fn get_by_blog_slug(
    State(store): State<Store>,
    Path(blog_slug): Path<String>,
) -> Result<Json<Vec<Post>>> {
    Ok(Json(post::get_by_blog_slug(&store.db, &blog_slug).await?))
}

async fn get_posts_by_username(
    State(store): State<Store>,
    Path(username): Path<String>,
) -> Result<Json<Vec<Post>>> {
    Ok(Json(post::get_by_username(&store.db, &username).await?))
}

/// Get a profile by username.
async fn get_profile_by_username(
    State(store): State<Store>,
    Path(username): Path<String>,
) -> Result<Json<Profile>> {
    Ok(Json(profile::get_by_username(&store.db, username).await?))
}

/// GET /profiles/:username/blogs
///
/// Get all blogs for a profile by username.
async fn get_blogs_by_username(
    State(store): State<Store>,
    Path(username): Path<String>,
) -> Result<Json<Vec<Blog>>> {
    Ok(Json(blog::get_by_username(&store.db, username).await?))
}

async fn get_ids_by_username(
    State(store): State<Store>,
    Path(username): Path<String>,
) -> Result<Json<Vec<Like>>> {
    Ok(Json(like::get_by_username(&store.db, username).await?))
}
