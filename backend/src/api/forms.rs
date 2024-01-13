use super::error::Result;
use crate::{
    entities::{blog, post, repo, user, Blog, Post, Repo, User},
    store::Store,
};
use axum::{
    extract::{Form, State},
    http::StatusCode,
};
use serde::Deserialize;
use sqlx::PgPool;

#[derive(Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct NewBlog {
    pub username: String,
    pub name: String,
    pub repo_url: String,
}

pub async fn new_blog(
    State(store): State<Store>,
    Form(new_blog_form_submission): Form<NewBlog>,
) -> Result<StatusCode> {
    dbg!(&new_blog_form_submission);

    validate_repo_url(&new_blog_form_submission.repo_url)?;
    validate_user_exists(&store.pool, &new_blog_form_submission.username).await?;

    let blog = create_blog(&store.pool, &new_blog_form_submission).await?;
    let repo = create_repo(&store.pool, &new_blog_form_submission).await?;

    Ok(StatusCode::CREATED)
}

fn validate_repo_url(repo_url: &str) -> Result<()> {
    let url = match url::Url::parse(repo_url) {
        Ok(url) => url,
        Err(_) => return Err(StatusCode::BAD_REQUEST.into()),
    };
    if url.scheme() != "https" {
        return Err(StatusCode::BAD_REQUEST.into());
    }
    if url.host_str() != Some("github.com") {
        return Err(StatusCode::BAD_REQUEST.into());
    }

    Ok(())
}

async fn validate_user_exists(pool: &PgPool, username: &str) -> Result<()> {
    user::get_by_username(pool, username).await?;
    Ok(())
}

async fn create_blog(pool: &PgPool, new_blog_form_submission: &NewBlog) -> Result<Blog> {
    let new_blog = blog::BlogForUpsert::new(
        new_blog_form_submission.name.clone(),
        blog_slug_from_repo(&new_blog_form_submission.repo_url)?,
        new_blog_form_submission.username.clone(),
    );
    dbg!(&new_blog);
    Ok(blog::upsert(pool, new_blog).await?)
}

fn blog_slug_from_repo(repo_url: &str) -> Result<String> {
    let url = match url::Url::parse(repo_url) {
        Ok(url) => url,
        Err(_) => return Err(StatusCode::BAD_REQUEST.into()),
    };
    let path = url.path();
    let path_parts: Vec<&str> = path.split('/').collect();
    let repo_name = path_parts.get(2).ok_or(StatusCode::BAD_REQUEST)?;
    Ok(repo_name.to_string())
}

async fn create_repo(pool: &PgPool, blog: &NewBlog) -> Result<()> {
    // let new_repo = repo::NewRepo {};

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_repo_url_valid() {
        let url = "https://github.com/username/repo";
        assert!(validate_repo_url(url).is_ok());
    }

    #[test]
    fn test_validate_repo_url_invalid_unparsable() {
        let url = "pete's pizza";
        assert!(validate_repo_url(url).is_err());
    }

    #[test]
    fn test_validate_repo_url_invalid_scheme() {
        let url = "http://github.com/username/repo";
        assert!(validate_repo_url(url).is_err());
    }

    #[test]
    fn test_validate_repo_url_invalid_host() {
        let url = "https://goathub.com/username/repo";
        assert!(validate_repo_url(url).is_err());
    }
}
