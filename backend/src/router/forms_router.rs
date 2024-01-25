use axum::{
    extract::{Form, State},
    routing::post,
    Json,
};

use super::error::Result;
use crate::forms::new_blog;
use crate::store::Store;

pub fn make_router(store: Store) -> axum::Router<Store> {
    axum::Router::new()
        .route("/forms/new-blog", post(new_blog))
        .with_state(store)
}

pub async fn new_blog(
    State(store): State<Store>,
    Form(new_blog): Form<new_blog::NewBlog>,
) -> Result<Json<new_blog::Response>> {
    Ok(Json(new_blog.process(&store.pool).await?))
}
