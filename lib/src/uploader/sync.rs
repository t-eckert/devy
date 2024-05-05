use crate::db;
use crate::db::Database;
use crate::db::{blog, post, repo};
use crate::entities::{Blog, Upload};
use crate::uploader::{diff::Diff, error::Result, Error};
use regex::Regex;
use slugify::slugify;

pub async fn sync(db: &Database, upload: &mut Upload, diffs: Vec<Diff>) -> Result<()> {
    dbg!("------------------------\nSyncing...");
    let repo = repo::get_by_url(db, &upload.repo)
        .await
        .map_err(|e| match e {
            db::Error::EntityNotFound => Error::RepositoryNotFound(upload.repo.clone()),
            _ => Error::DependencyError(e.to_string()),
        })?;

    dbg!("------------------------\nGetting blog...");
    let blog = blog::get_by_id(db, repo.blog_id).await?;
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

pub struct Markdown {
    pub title: String,
    pub body: String,
}

impl Markdown {
    pub fn from_file(path: String) -> Result<Self> {
        let (title, body) = Self::title(
            std::fs::read_to_string(&path).map_err(|e| Error::FileParseError(e.to_string()))?,
        );

        Ok(Self { title, body })
    }

    fn title(body: String) -> (String, String) {
        let re = Regex::new(r"^# (.*)").unwrap();
        match re.captures(&body) {
            Some(caps) => {
                let title = caps.get(1).unwrap().as_str().to_owned();
                let body = re.replace(&body, "").to_string();

                (title, body)
            }
            None => ("Untitled".to_owned(), body),
        }
    }
}

async fn add_post(db: &Database, blog: &Blog, dir: &str, path: String) -> Result<()> {
    let slug = slugify!(&path.replace(".md", ""));

    let markdown = Markdown::from_file(format!("{}/{}", dir, path))?;

    post::insert(db, blog.id, markdown.title, slug, markdown.body).await?;

    Ok(())
}

async fn modify_post(db: &Database, blog: &Blog, dir: &str, path: String) -> Result<()> {
    let slug = slugify!(&path.replace(".md", ""));
    let mut post = post::get_by_blog_slug_and_post_slug(db, &blog.slug, &slug).await?;
    let markdown = Markdown::from_file(format!("{}/{}", dir, path))?;

    post.title = markdown.title;
    post.slug = slug;
    post.body = markdown.body;

    post::update(db, post).await?;

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
    let mut post = post::get_by_blog_slug_and_post_slug(db, &blog.slug, &from_slug).await?;

    let slug = slugify!(&to.replace(".md", ""));
    let markdown = Markdown::from_file(format!("{}/{}", dir, to))?;

    post.title = markdown.title;
    post.slug = slug;
    post.body = markdown.body;

    post::update(db, post).await?;

    Ok(())
}

async fn delete_post(db: &Database, blog: &Blog, path: String) -> Result<()> {
    let slug = slugify!(&path.replace(".md", ""));
    let post = match post::get_by_blog_slug_and_post_slug(db, &blog.slug, &slug).await {
        Ok(post) => post,
        Err(_) => return Ok(()),
    };
    post::delete(db, post.id).await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_sync() {}
}
