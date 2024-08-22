use crate::router::{error::Result, middleware::auth};
use axum::{
    extract::{Json as ExtractJson, State},
    http::StatusCode,
    middleware,
    routing::post,
    Json, Router,
    Extension,
};
use lib::{
    forms::new_blog::{NewBlog, NewBlogResponse},
    store::Store, token::Session,
};
use serde_json::Value;

/// Create a new router for the forms endpoints.
pub fn router(store: Store) -> Router<Store> {
    Router::new()
        .route("/forms/new-blog", post(new_blog))
        .layer(middleware::from_fn_with_state(store.clone(), auth))
        .with_state(store)
}

/// `POST /forms/new-blog`
///
/// Creates a new blog.
async fn new_blog(
    Extension(session): Extension<Session>,
    State(store): State<Store>,
    ExtractJson(new_blog): ExtractJson<NewBlog>,
) -> Result<Json<NewBlogResponse>> {
    if session.user_id != new_blog.user_id {
        return Err(StatusCode::FORBIDDEN.into());
    }

    Ok(Json(new_blog.process(&store.db_conn).await?))
}
