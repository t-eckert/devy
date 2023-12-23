use super::error::Result;
use crate::{
    entities::{Blog, BlogRepository, NewBlog, Post, PostRepository},
    store::Store,
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};

/// Get a blog from the database given a blog slug.
/// GET /blogs/:blog_slug
pub async fn get_blog_by_blog_slug(
    State(store): State<Store>,
    Path(blog_slug): Path<String>,
) -> Result<Json<Blog>> {
    Ok(Json(
        BlogRepository::get_by_slug(&store.pool, blog_slug).await?,
    ))
}

/// Get a post from the database given a blog slug and post slug.
/// GET /blogs/:blog_slug/posts/:post_slug
pub async fn get_post_by_blog_and_post_slug(
    State(store): State<Store>,
    Path((blog_slug, post_slug)): Path<(String, String)>,
) -> Result<Json<Post>> {
    Ok(Json(
        PostRepository::get_by_blog_slug_and_post_slug(&store.pool, &blog_slug, &post_slug).await?,
    ))
}

/// Create a new blog in the database.
/// POST /blogs
pub async fn create_new_blog(
    State(store): State<Store>,
    Json(blog_creation_req): Json<BlogCreationRequest>,
) -> Result<StatusCode> {
    dbg!(&blog_creation_req);

    Ok(StatusCode::CREATED)
}

/// Delete a blog from the database given a blog slug.
/// DELETE /blogs/:blog_slug
pub async fn delete(
    State(store): State<Store>,
    Path(blog_slug): Path<String>,
) -> Result<StatusCode> {
    BlogRepository::delete_by_slug(&store.pool, blog_slug).await?;

    Ok(StatusCode::OK)
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlogCreationRequest {
    pub repo_url: String,
}
