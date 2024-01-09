use super::error::Result;
use crate::{
    entities::{blog, repo, Blog, BlogRepository, Post, PostRepository},
    store::Store,
};
use axum::{
    extract::{Form, State},
    http::StatusCode,
};
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct NewBlog {
    pub username: String,
    pub name: String,
    pub repo_url: String,
}

pub async fn new_blog(State(store): State<Store>, Form(blog): Form<NewBlog>) -> Result<StatusCode> {
    dbg!(&blog);
    Ok(StatusCode::CREATED)
}
