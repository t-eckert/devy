use axum::{
    extract::{Form, State},
    http::StatusCode,
};

use super::error::Result;
use crate::forms::NewBlog;
use crate::store::Store;

pub async fn new_blog(
    State(store): State<Store>,
    Form(new_blog): Form<NewBlog>,
) -> Result<StatusCode> {
    let _ = new_blog.process(&store.pool).await?;

    Ok(StatusCode::CREATED)
}
