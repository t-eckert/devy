use super::error::Result;
use crate::{entities::Blog, store::Store};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};

pub async fn get_by_username(
    State(store): State<Store>,
    Path(username): Path<String>,
) -> Result<Json<Vec<Blog>>> {
    Ok(Json(Blog::get_by_username(&store.pool, username).await?))
}

pub async fn get_by_blog_slug(
    State(store): State<Store>,
    Path(blog_slug): Path<String>,
) -> Result<Json<Blog>> {
    Ok(Json(Blog::get_by_slug(&store.pool, blog_slug).await?))
}

pub async fn upsert(State(store): State<Store>, Json(blog): Json<Blog>) -> Result<Json<Blog>> {
    Ok(Json(blog.upsert(&store.pool).await?))
}

pub async fn delete(
    State(store): State<Store>,
    Path(blog_slug): Path<String>,
) -> Result<StatusCode> {
    Blog::delete_by_slug(&store.pool, blog_slug).await?;

    Ok(StatusCode::OK)
}
