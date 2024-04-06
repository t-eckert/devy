use crate::error::Result;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{delete, get},
    Json, Router,
};
use db::{blog, post};
use entities::{Blog, Post};
use store::Store;

pub struct BlogsRouter;

impl BlogsRouter {
    pub fn create(store: Store) -> Router<Store> {
        Router::new()
            .route("/blogs/:blog_slug", get(get_blog_by_blog_slug))
            .route("/blogs/:blog_slug/posts", get(get_posts_by_blog_slug))
            .route(
                "/blogs/:blog_slug/posts/:post_slug",
                get(get_post_by_blog_and_post_slug),
            )
            .route("/blogs/:blog_slug", delete(delete_blog_by_blog_slug))
            .with_state(store)
    }
}

/// GET /blogs/:blog_slug
///
/// Get a blog from the database given a blog slug.
async fn get_blog_by_blog_slug(
    State(store): State<Store>,
    Path(blog_slug): Path<String>,
) -> Result<Json<Blog>> {
    Ok(Json(blog::get_by_slug(&store.db, blog_slug).await?))
}

/// GET /blogs/:blog_slug/posts
///
/// Get posts from the database given a blog slug.
async fn get_posts_by_blog_slug(
    State(store): State<Store>,
    Path(blog_slug): Path<String>,
) -> Result<Json<Vec<Post>>> {
    Ok(Json(post::get_by_blog_slug(&store.db, blog_slug).await?))
}

/// GET /blogs/:blog_slug/posts/:post_slug
///
/// Get a post from the database given a blog slug and post slug.
async fn get_post_by_blog_and_post_slug(
    State(store): State<Store>,
    Path((blog_slug, post_slug)): Path<(String, String)>,
) -> Result<Json<Post>> {
    Ok(Json(
        post::get_by_blog_slug_and_post_slug(&store.db, &blog_slug, &post_slug).await?,
    ))
}

/// DELETE /blogs/:blog_slug
///
/// Delete a blog from the database given a blog slug.
async fn delete_blog_by_blog_slug(
    State(store): State<Store>,
    Path(blog_slug): Path<String>,
) -> Result<StatusCode> {
    blog::delete_by_slug(&store.db, blog_slug).await?;

    Ok(StatusCode::OK)
}
