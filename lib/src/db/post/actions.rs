use crate::db::{Database, Error, Result};
use crate::entities::Post;
use uuid::{uuid, Uuid};

pub async fn insert(
    db: &Database,
    blog_id: Uuid,
    title: String,
    slug: String,
    body: String,
) -> Result<Post> {
    Ok(sqlx::query_file_as!(
        Post,
        "src/db/post/queries/insert.sql",
        blog_id,
        title,
        slug,
        body
    )
    .fetch_one(db)
    .await?)
}

pub async fn update(db: &Database, post: Post) -> Result<Post> {
    Ok(sqlx::query_file_as!(
        Post,
        "src/db/post/queries/update.sql",
        post.id,
        post.blog_id,
        post.title,
        post.slug,
        post.body
    )
    .fetch_one(db)
    .await?)
}

pub async fn get(db: &Database, id: Uuid) -> Result<Post> {
    Ok(
        sqlx::query_file_as!(Post, "src/db/post/queries/get.sql", id)
            .fetch_one(db)
            .await?,
    )
}

/// Returns all posts for a given blog, identified by its slug.
pub async fn get_by_blog_slug(db: &Database, blog_slug: String) -> Result<Vec<Post>> {
    Ok(
        sqlx::query_file_as!(Post, "src/db/post/queries/get_by_blog_slug.sql", blog_slug)
            .fetch_all(db)
            .await?,
    )
}

pub async fn get_by_blog_slug_and_post_slug(
    db: &Database,
    blog_slug: &str,
    post_slug: &str,
) -> Result<Post> {
    Ok(sqlx::query_file_as!(
        Post,
        "src/db/post/queries/get_by_blog_slug_and_post_slug.sql",
        blog_slug,
        post_slug
    )
    .fetch_one(db)
    .await?)
}

pub async fn get_by_feed(
    db: &Database,
    feed_id: Uuid,
    limit: i64,
    offset: i64,
) -> Result<Vec<Post>> {
    let new = uuid!("5941b29d-246d-4897-a69e-3201f6ad8715");
    let popular = uuid!("e9173695-1b31-465f-9e79-a80224be44ad");

    match feed_id {
        id if id == new => get_by_feed_new(db, limit, offset).await,
        id if id == popular => get_by_feed_popular(db, limit, offset).await,
        _ => Err(Error::EntityNotFound),
    }
}

pub async fn get_by_feed_new(db: &Database, limit: i64, offset: i64) -> Result<Vec<Post>> {
    Ok(sqlx::query_file_as!(
        Post,
        "src/db/post/queries/get_by_feed_new.sql",
        limit,
        offset
    )
    .fetch_all(db)
    .await?)
}

pub async fn get_by_feed_popular(pool: &Database, limit: i64, offset: i64) -> Result<Vec<Post>> {
    Ok(sqlx::query_file_as!(
        Post,
        "src/db/post/queries/get_by_feed_popular.sql",
        limit,
        offset
    )
    .fetch_all(pool)
    .await?)
}

pub async fn get_by_username(db: &Database, username: &str) -> Result<Vec<Post>> {
    Ok(
        sqlx::query_file_as!(Post, "src/db/post/queries/get_by_username.sql", username)
            .fetch_all(db)
            .await?,
    )
}

pub async fn delete(db: &Database, id: Uuid) -> Result<()> {
    sqlx::query_file_as!(Post, "src/db/post/queries/delete.sql", id)
        .execute(db)
        .await?;
    Ok(())
}

#[cfg(test)]
mod test {}
