use crate::{
    blogs::Blog,
    db::Conn,
    markdown::parse_markdown,
    posts::PostRepository,
    repositories::RepoRepository,
    uploader::{Error, Result},
    uploads::{
        changeset::{Changeset, PostAddition, PostChange, PostDeletion, PostModification},
        Status, Upload,
    },
};
use slugify::slugify;
use uuid::Uuid;

pub async fn sync(db_conn: &Conn, mut upload: Upload) -> Upload {
    tracing::info!("Syncing upload {}", upload.id);

    if upload.status != Status::COMMITTED || upload.changeset.is_none() {
        upload.set_status(Status::REJECTED);
        upload.append_log("ERROR: Upload is not committed");
        return upload;
    }

    let changeset = upload.clone().changeset.unwrap();
    for post_change in changeset.post_changeset.post_changes {
        match post_change {
            PostChange::Addition(PostAddition { path }) => {
                match add_post(db_conn, changeset.blog_id, &changeset.dir, &path).await {
                    Ok(id) => {
                        dbg!(id);
                    }
                    Err(e) => {
                        dbg!(e);
                    }
                }
            }
            PostChange::Modification(PostModification { path, from_path }) => {
                modify_post(
                    db_conn,
                    &changeset.blog_slug,
                    &changeset.dir,
                    &path,
                    from_path.as_deref(),
                )
                .await;
            }
            PostChange::Deletion(PostDeletion { path }) => {
                delete_post(db_conn, &changeset.blog_slug, path).await;
            }
        }
    }

    upload.set_status(Status::SYNCED);
    upload.append_log("INFO: Upload synced");
    tracing::info!("Upload synced {}", upload.id);

    upload
}

async fn add_post(db: &Conn, blog_id: Uuid, dir: &str, path: &str) -> Result<Uuid> {
    let slug = slugify!(&path.replace(".md", ""));

    let raw = std::fs::read_to_string(format!("{}/{}", dir, path))
        .map_err(|e| Error::FileParseError(e.to_string()))?;
    let markdown = parse_markdown(&raw);

    let id = PostRepository::insert(
        db,
        blog_id,
        &markdown.title,
        &slug,
        &markdown.body,
        markdown.is_draft(),
    )
    .await?;

    Ok(id)
}

async fn modify_post(
    db: &Conn,
    blog_slug: &str,
    dir: &str,
    path: &str,
    from_path: Option<&str>,
) -> Result<()> {
    if from_path.is_some() {
        rename_post(
            db,
            blog_slug,
            dir,
            from_path.unwrap().to_string(),
            path.to_string(),
        )
        .await?;
        return Ok(());
    }

    let slug = slugify!(&path.replace(".md", ""));
    let mut post = PostRepository::get_by_blog_slug_and_post_slug(db, blog_slug, &slug).await?;

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
    db: &Conn,
    blog_slug: &str,
    dir: &str,
    from: String,
    to: String,
) -> Result<()> {
    let from_slug = slugify!(&from.replace(".md", ""));
    let mut post =
        PostRepository::get_by_blog_slug_and_post_slug(db, blog_slug, &from_slug).await?;

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

async fn delete_post(db: &Conn, blog_slug: &str, path: String) -> Result<()> {
    let slug = slugify!(&path.replace(".md", ""));
    let post = match PostRepository::get_by_blog_slug_and_post_slug(db, blog_slug, &slug).await {
        Ok(post) => post,
        Err(_) => return Ok(()),
    };
    PostRepository::delete(db, post.id).await?;
    Ok(())
}
