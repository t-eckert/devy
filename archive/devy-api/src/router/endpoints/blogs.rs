use crate::{
    controllers::{BlogsController, EntriesController, PostsController},
    router::{error::Result, middleware::auth},
    store::Store,
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    middleware,
    routing::{delete, get},
    Extension, Json, Router,
};
use lib::{
    auth::token::Session, blogs::Blog, posts::Entry, posts::Post, repositories::BlogRepository,
};

/// Create a new router for blogs.
pub fn router(store: Store) -> Router<Store> {
    let open = Router::new()
        .route("/blogs/:blog_slug", get(get_blog))
        .route("/blogs/:blog_slug/posts", get(get_blog_posts))
        .route("/blogs/:blog_slug/entries", get(get_blog_entries))
        .route("/blogs/:blog_slug/posts/:post_slug", get(get_blog_post))
        .route("/blogs/:blog_slug/entries/:post_slug", get(get_blog_entry))
        .with_state(store.clone());

    let protected = Router::new()
        .route("/blogs/:blog_slug", delete(delete_blog))
        .layer(middleware::from_fn_with_state(store.clone(), auth))
        .with_state(store);

    Router::new().merge(open).merge(protected)
}

// GET /blogs/:blog_slug
async fn get_blog(State(store): State<Store>, Path(blog_slug): Path<String>) -> Result<Json<Blog>> {
    Ok(Json(
        BlogsController::get_by_slug(&store, &blog_slug).await?,
    ))
}

// GET /blogs/:blog_slug/posts
async fn get_blog_posts(
    State(store): State<Store>,
    Path(blog_slug): Path<String>,
) -> Result<Json<Vec<lib::posts::Post>>> {
    Ok(Json(
        PostsController::get_by_blog_slug(&store, &blog_slug).await?,
    ))
}

// GET /blogs/:blog_slug/entries
async fn get_blog_entries(
    State(store): State<Store>,
    Path(blog_slug): Path<String>,
) -> Result<Json<Vec<Entry>>> {
    Ok(Json(
        EntriesController::get_by_blog_slug(&store, &blog_slug).await?,
    ))
}

// GET /blogs/:blog_slug/posts/:post_slug
async fn get_blog_post(
    State(store): State<Store>,
    Path((blog_slug, post_slug)): Path<(String, String)>,
) -> Result<Json<Post>> {
    Ok(Json(
        PostsController::get_by_blog_slug_and_post_slug(&store, &blog_slug, &post_slug).await?,
    ))
}

// GET /blogs/:blog_slug/entries/:post_slug
async fn get_blog_entry(
    State(store): State<Store>,
    Path((blog_slug, post_slug)): Path<(String, String)>,
) -> Result<Json<Entry>> {
    Ok(Json(
        EntriesController::get_by_blog_slug_and_post_slug(&store, &blog_slug, &post_slug).await?,
    ))
}

// DELETE /blogs/:blog_slug
async fn delete_blog(
    Extension(session): Extension<Session>,
    State(store): State<Store>,
    Path(blog_slug): Path<String>,
) -> Result<StatusCode> {
    if session.user_id
        != BlogRepository::get_by_slug(&store.db_conn, &blog_slug.clone())
            .await?
            .user_id
    {
        return Err(StatusCode::FORBIDDEN.into());
    }

    BlogsController::delete_by_slug(&store, &blog_slug).await?;

    Ok(StatusCode::OK)
}
