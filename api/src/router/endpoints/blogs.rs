use crate::router::{error::Result, middleware::auth};
use axum::{
    extract::{Path, State}, http::StatusCode, middleware, routing::{delete, get}, Extension, Json, Router
};
use lib::{
    db::{blog, entry, post},
    entities::{Blog, Entry, Post},
    store::Store, token::Session,
};

/// Create a new router for the blogs endpoints.
pub fn router(store: Store) -> Router<Store> {
    let open = Router::new()
        .route("/blogs/:blog_slug", get(get_blog_by_blog_slug))
        .route("/blogs/:blog_slug/posts", get(get_blog_posts_by_blog_slug))
        .route(
            "/blogs/:blog_slug/entries",
            get(get_blog_entries_by_blog_slug),
        )
        .route(
            "/blogs/:blog_slug/posts/:post_slug",
            get(get_blog_post_by_blog_slug_and_post_slug),
        )
        .route(
            "/blogs/:blog_slug/entries/:post_slug",
            get(get_blog_entry_by_blog_slug_and_post_slug),
        )
        .with_state(store.clone());

    let protected = Router::new()
        .route("/blogs/:blog_slug", delete(delete_blog_by_blog_slug))
        .layer(middleware::from_fn_with_state(store.clone(), auth))
        .with_state(store);

    Router::new().merge(open).merge(protected)
}

// GET /blogs/:blog_slug
async fn get_blog_by_blog_slug(
    State(store): State<Store>,
    Path(blog_slug): Path<String>,
) -> Result<Json<Blog>> {
    Ok(Json(blog::get_by_slug(&store.db, blog_slug).await?))
}

// GET /blogs/:blog_slug/posts
async fn get_blog_posts_by_blog_slug(
    State(store): State<Store>,
    Path(blog_slug): Path<String>,
) -> Result<Json<Vec<Post>>> {
    Ok(Json(post::get_by_blog_slug(&store.db, blog_slug).await?))
}

// GET /blogs/:blog_slug/entries
async fn get_blog_entries_by_blog_slug(
    State(store): State<Store>,
    Path(blog_slug): Path<String>,
) -> Result<Json<Vec<Entry>>> {
    Ok(Json(entry::get_by_blog_slug(&store.db, &blog_slug).await?))
}

// GET /blogs/:blog_slug/posts/:post_slug
async fn get_blog_post_by_blog_slug_and_post_slug(
    State(store): State<Store>,
    Path((blog_slug, post_slug)): Path<(String, String)>,
) -> Result<Json<Post>> {
    Ok(Json(
        post::get_by_blog_slug_and_post_slug(&store.db, &blog_slug, &post_slug).await?,
    ))
}

// GET /blogs/:blog_slug/entries/post_slug
async fn get_blog_entry_by_blog_slug_and_post_slug(
    State(store): State<Store>,
    Path((blog_slug, post_slug)): Path<(String, String)>,
) -> Result<Json<Entry>> {
    Ok(Json(
        entry::get_by_blog_slug_and_post_slug(&store.db, &blog_slug, &post_slug).await?,
    ))
}

// DELETE /blogs/:blog_slug
async fn delete_blog_by_blog_slug(
    Extension(session): Extension<Session>,
    State(store): State<Store>,
    Path(blog_slug): Path<String>,
) -> Result<StatusCode> {
    if session.user_id != blog::get_by_slug(&store.db, blog_slug.clone()).await?.user_id {
        return Ok(StatusCode::FORBIDDEN);
    }

    blog::delete_by_slug(&store.db, blog_slug).await?;

    Ok(StatusCode::OK)
}
