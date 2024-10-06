use super::Upload;
use crate::db;
use crate::db::Database;
use crate::git::Diff;
use crate::markdown::parse_markdown;
use crate::posts::PostRepository;
use crate::repositories::RepoRepository;
use crate::uploader::{error::Result, Error};

use crate::blogs::{Blog, BlogRepository};

use regex::Regex;
use slugify::slugify;

pub async fn sync(db: &Database, upload: &mut Upload, diffs: Vec<Diff>) -> Result<()> {
    let repo = RepoRepository::get_by_url(db, &upload.repo)
        .await
        .map_err(|e| match e {
            db::Error::EntityNotFound => Error::RepositoryNotFound(upload.repo.clone()),
            _ => Error::DependencyError(e.to_string()),
        })?;

    let blog = BlogRepository::get(db, repo.blog_id).await?;
    let dir = format!("/tmp/{}", upload.id);
    dbg!(&blog);

    // Errors while syncing are not fatal, so we log them and continue.
    for diff in diffs {
        let result = match diff {
            Diff::Added(file) => add_post(db, &blog, &dir, file).await,
            Diff::Modified(file) => modify_post(db, &blog, &dir, file).await,
            Diff::Renamed(from, to) => rename_post(db, &blog, &dir, from, to).await,
            Diff::Deleted(file) => delete_post(db, &blog, file).await,
        };
        match result {
            Ok(_) => (),
            Err(e) => {
                tracing::error!("Error syncing post: {}", e);
            }
        }
    }

    Ok(())
}

async fn add_post(db: &Database, blog: &Blog, dir: &str, path: String) -> Result<()> {
    let slug = slugify!(&path.replace(".md", ""));

    let raw = std::fs::read_to_string(format!("{}/{}", dir, path))
        .map_err(|e| Error::FileParseError(e.to_string()))?;
    let markdown = parse_markdown(&raw);

    PostRepository::insert(
        db,
        blog.id,
        &markdown.title,
        &slug,
        &markdown.body,
        markdown.is_draft(),
    )
    .await?;

    Ok(())
}

async fn modify_post(db: &Database, blog: &Blog, dir: &str, path: String) -> Result<()> {
    let slug = slugify!(&path.replace(".md", ""));
    let mut post = PostRepository::get_by_blog_slug_and_post_slug(db, &blog.slug, &slug).await?;

    let raw = std::fs::read_to_string(format!("{}/{}", dir, path))
        .map_err(|e| Error::FileParseError(e.to_string()))?;
    let markdown = parse_markdown(&raw);

    post.title = markdown.title.clone();
    post.is_draft = markdown.is_draft();
    post.slug = slug;
    post.body = markdown.body;

    PostRepository::update(db, &post).await?;

    Ok(())
}

async fn rename_post(
    db: &Database,
    blog: &Blog,
    dir: &str,
    from: String,
    to: String,
) -> Result<()> {
    let from_slug = slugify!(&from.replace(".md", ""));
    let mut post =
        PostRepository::get_by_blog_slug_and_post_slug(db, &blog.slug, &from_slug).await?;

    let slug = slugify!(&to.replace(".md", ""));
    let raw = std::fs::read_to_string(format!("{}/{}", dir, to))
        .map_err(|e| Error::FileParseError(e.to_string()))?;
    let markdown = parse_markdown(&raw);

    post.title = markdown.title.clone();
    post.is_draft = markdown.is_draft();
    post.slug = slug;
    post.body = markdown.body;

    PostRepository::update(db, &post).await?;

    Ok(())
}

async fn delete_post(db: &Database, blog: &Blog, path: String) -> Result<()> {
    let slug = slugify!(&path.replace(".md", ""));
    let post = match PostRepository::get_by_blog_slug_and_post_slug(db, &blog.slug, &slug).await {
        Ok(post) => post,
        Err(_) => return Ok(()),
    };
    PostRepository::delete(db, post.id).await?;
    Ok(())
}

#[cfg(test)]
mod tests {

    #[tokio::test]
    async fn test_sync() {}
}
