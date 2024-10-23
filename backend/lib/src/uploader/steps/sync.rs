use crate::{
    blogs::Blog,
    db::Conn,
    markdown::parse_markdown,
    posts::PostRepository,
    repositories::RepoRepository,
    uploader::{Error, Result},
    uploads::{Status, Upload},
};
use slugify::slugify;

pub async fn sync(db_conn: &Conn, mut upload: Upload) -> Upload {
    tracing::info!("Syncing upload {}", upload.id);

    if upload.status != Status::DIFFED {
        upload.set_status(Status::REJECTED);
        upload.append_log("ERROR: Upload is not diffed");
        return upload;
    }

    upload.set_status(Status::SYNCED);
    upload.append_log("INFO: Upload synced");
    tracing::info!("Upload synced {}", upload.id);

    upload
}

// -------------------------------------------------------------------------------------------------------------------

async fn add_post(db: &Conn, blog: &Blog, dir: &str, path: String) -> Result<()> {
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

async fn modify_post(db: &Conn, blog: &Blog, dir: &str, path: String) -> Result<()> {
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

async fn rename_post(db: &Conn, blog: &Blog, dir: &str, from: String, to: String) -> Result<()> {
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

async fn delete_post(db: &Conn, blog: &Blog, path: String) -> Result<()> {
    let slug = slugify!(&path.replace(".md", ""));
    let post = match PostRepository::get_by_blog_slug_and_post_slug(db, &blog.slug, &slug).await {
        Ok(post) => post,
        Err(_) => return Ok(()),
    };
    PostRepository::delete(db, post.id).await?;
    Ok(())
}
