use super::{markdown, Diff};
use crate::error::{Error, Result};
use db::{blog, post, repo, upload, Database};
use entities::Blog;
use slugify::slugify;
use uuid::Uuid;

pub async fn sync_posts(db: &Database, id: Uuid, dir: &str, diffs: Vec<Diff>) -> Result<()> {
    let upload = upload::get_by_id(db, id).await?;
    let repo = repo::get_by_url(db, &upload.repo)
        .await
        .map_err(|e| match e {
            db::Error::EntityNotFound => Error::RepositoryNotFound(upload.repo.clone()),
            _ => Error::DependencyError(e.to_string()),
        })?;

    let blog = blog::get_by_id(db, repo.blog_id).await?;
    for diff in diffs {
        dbg!(&diff);
        match diff {
            Diff::Added(file) => {
                if markdown::file_is_markdown(&file) {
                    match add_post(db, &blog, dir, file).await {
                        Ok(_) => {}
                        Err(e) => {
                            println!("Error: {:?}", e);
                            return Ok(());
                        }
                    };
                }
            }
            Diff::Modified(file) => {
                if markdown::file_is_markdown(&file) {
                    match modify_post(db, &blog, dir, file).await {
                        Ok(_) => {}
                        Err(e) => {
                            println!("Error: {:?}", e);
                            return Ok(());
                        }
                    };
                }
            }
            Diff::Renamed(_, from, to) => {
                if markdown::file_is_markdown(&to) {
                    match rename_post(db, &blog, dir, from, to).await {
                        Ok(_) => {}
                        Err(e) => {
                            println!("Error: {:?}", e);
                            return Ok(());
                        }
                    };
                }
            }
            Diff::Deleted(file) => {
                if markdown::file_is_markdown(&file) {
                    match delete_post(db, &blog, file).await {
                        Ok(_) => {}
                        Err(e) => {
                            println!("Error: {:?}", e);
                            return Ok(());
                        }
                    };
                }
            }
        }
    }
    Ok(())
}

async fn add_post(db: &Database, blog: &Blog, dir: &str, path: String) -> Result<()> {
    let slug = slugify!(&path.replace(".md", ""));
    let markdown = markdown::Markdown::from_file(format!("{}/{}", dir, path))?;

    post::insert(db, blog.id, markdown.title, slug, markdown.body).await?;

    Ok(())
}

async fn modify_post(db: &Database, blog: &Blog, dir: &str, path: String) -> Result<()> {
    let slug = slugify!(&path.replace(".md", ""));
    let mut post = post::get_by_blog_slug_and_post_slug(db, &blog.slug, &slug).await?;
    let markdown = markdown::Markdown::from_file(format!("{}/{}", dir, path))?;

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
    let markdown = markdown::Markdown::from_file(format!("{}/{}", dir, to))?;

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
