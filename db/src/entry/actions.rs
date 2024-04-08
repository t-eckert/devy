use crate::{error::Result, Database};
use entities::Entry;

pub async fn get_by_blog_slug(db: &Database, blog_slug: &str) -> Result<Vec<Entry>> {
    Ok(
        sqlx::query_file_as!(Entry, "src/entry/queries/get_by_blog_slug.sql", blog_slug)
            .fetch_all(db)
            .await?,
    )
}

pub async fn get_by_blog_slug_and_post_slug(
    db: &Database,
    blog_slug: &str,
    post_slug: &str,
) -> Result<Entry> {
    Ok(sqlx::query_file_as!(
        Entry,
        "src/entry/queries/get_by_blog_slug_and_post_slug.sql",
        blog_slug,
        post_slug
    )
    .fetch_one(db)
    .await?)
}
