use crate::{entities::Blog, store::Store};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};

pub async fn get_blog_by_username(
    State(store): State<Store>,
    Path(username): Path<String>,
) -> Result<Json<Vec<Blog>>, StatusCode> {
    Ok(Json(
        Blog::get_by_username(&store.pool, username)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
    ))
}
