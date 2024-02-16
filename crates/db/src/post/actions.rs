use crate::{Database, Error, Result};
use entities::Post;
use uuid::{uuid, Uuid};

pub async fn get_by_id(db: &Database, id: Uuid) -> Result<Post> {
    Ok(
        sqlx::query_file_as!(Post, "src/post/queries/get_by_id.sql", id)
            .fetch_one(db)
            .await?,
    )
}

pub async fn get_by_blog_slug(db: &Database, blog_slug: &str) -> Result<Vec<Post>> {
    Ok(
        sqlx::query_file_as!(Post, "src/post/queries/get_by_blog_slug.sql", blog_slug)
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
        "src/post/queries/get_by_blog_slug_and_post_slug.sql",
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
    Ok(
        sqlx::query_file_as!(Post, "src/post/queries/get_by_feed_new.sql", limit, offset)
            .fetch_all(db)
            .await?,
    )
}

pub async fn get_by_feed_popular(pool: &Database, limit: i64, offset: i64) -> Result<Vec<Post>> {
    Ok(sqlx::query_file_as!(
        Post,
        "src/post/queries/get_by_feed_popular.sql",
        limit,
        offset
    )
    .fetch_all(pool)
    .await?)
}

pub async fn get_by_username(pool: &Database, username: &str) -> Result<Vec<Post>> {
    Ok(
        sqlx::query_file_as!(Post, "src/post/queries/get_by_username.sql", username)
            .fetch_all(pool)
            .await?,
    )
}
