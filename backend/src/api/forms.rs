use axum::{
    extract::{Form, State},
    Json,
};

use super::error::Result;
use crate::forms::new_blog;
use crate::store::Store;

pub async fn new_blog(
    State(store): State<Store>,
    Form(new_blog): Form<new_blog::NewBlog>,
) -> Result<Json<new_blog::Response>> {
    Ok(Json(new_blog.process(&store.pool).await?))
}
