use crate::router::{error::Result, middleware::auth};
use axum::{
    extract::{Json as ExtractJson, State},
    middleware,
    routing::post,
    Json, Router,
};
use lib::{
    forms::new_blog::{NewBlog, NewBlogResponse},
    store::Store,
};

/// Create a new router for the forms endpoints.
pub fn router(store: Store) -> Router<Store> {
    Router::new()
        .layer(middleware::from_fn_with_state(store.clone(), auth))
        .route("/forms/new-blog", post(new_blog))
        .with_state(store)
}

/// `POST /forms/new-blog`
///
/// Creates a new blog.
async fn new_blog(
    State(store): State<Store>,
    ExtractJson(new_blog): ExtractJson<NewBlog>,
) -> Result<Json<NewBlogResponse>> {
    Ok(Json(new_blog.process(&store.db).await?))
}
