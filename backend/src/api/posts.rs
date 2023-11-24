use super::error::Result;
use crate::{entities::Post, store::Store};
use axum::{
    extract::{Path, State},
    Json,
};

pub async fn get_by_post_id(
    State(store): State<Store>,
    Path(post_id): Path<String>,
) -> Result<Json<Post>> {
    Ok(Json(Post::get_by_id(&store.pool, post_id).await?))
}

pub async fn get_by_blog_slug(
    State(store): State<Store>,
    Path(blog_slug): Path<String>,
) -> Result<Json<Vec<Post>>> {
    Ok(Json(Post::get_by_blog_slug(&store.pool, blog_slug).await?))
}

pub async fn get_by_blog_and_post_slug(
    State(store): State<Store>,
    Path((blog_slug, post_slug)): Path<(String, String)>,
) -> Result<Json<Post>> {
    Ok(Json(
        Post::get_by_blog_and_post_slug(&store.pool, blog_slug, post_slug).await?,
    ))
}

pub async fn get_by_username(
    State(store): State<Store>,
    Path(username): Path<String>,
) -> Result<Json<Vec<Post>>> {
    Ok(Json(Post::get_by_username(&store.pool, username).await?))
}

pub async fn get_liked_by_username(
    State(store): State<Store>,
    Path(username): Path<String>,
) -> Result<Json<Vec<Post>>> {
    Ok(Json(
        Post::get_liked_by_username(&store.pool, username).await?,
    ))
}
