use sqlx::PgPool;
use uuid::{uuid, Uuid};

use super::*;

pub async fn upsert(_pool: &PgPool, _post: PostForUpsert) -> Result<Post> {
    // TODO
    Ok(Post {
        id: uuid!("5941b29d-246d-4897-a69e-3201f6ad8715"),
        slug: "slug".to_string(),
        blog_slug: Some("blog_slug".to_string()),
        blog_name: None,
        author_name: None,
        author_username: None,
        title: "title".to_string(),
        content: "content".to_string(),
        likes: None,
        created_at: None,
        updated_at: None,
    })
}

pub async fn get_by_id(pool: &PgPool, id: Uuid) -> Result<Post> {
    Ok(
        sqlx::query_file_as!(Post, "src/entities/post/queries/get_by_id.sql", id)
            .fetch_one(pool)
            .await?,
    )
}

pub async fn get_by_blog_slug(pool: &PgPool, blog_slug: &str) -> Result<Vec<Post>> {
    Ok(sqlx::query_file_as!(
        Post,
        "src/entities/post/queries/get_by_blog_slug.sql",
        blog_slug
    )
    .fetch_all(pool)
    .await?)
}

pub async fn get_by_blog_slug_and_post_slug(
    pool: &PgPool,
    blog_slug: &str,
    post_slug: &str,
) -> Result<Post> {
    Ok(sqlx::query_file_as!(
        Post,
        "src/entities/post/queries/get_by_blog_slug_and_post_slug.sql",
        blog_slug,
        post_slug
    )
    .fetch_one(pool)
    .await?)
}

pub async fn get_by_feed(
    pool: &PgPool,
    feed_id: Uuid,
    limit: i64,
    offset: i64,
) -> Result<Vec<Post>> {
    let new = uuid!("5941b29d-246d-4897-a69e-3201f6ad8715");
    let popular = uuid!("e9173695-1b31-465f-9e79-a80224be44ad");

    match feed_id {
        id if id == new => get_by_feed_new(pool, limit, offset).await,
        id if id == popular => get_by_feed_popular(pool, limit, offset).await,
        _ => Err(Error::EntityNotFound),
    }
}

pub async fn get_by_feed_new(pool: &PgPool, limit: i64, offset: i64) -> Result<Vec<Post>> {
    Ok(sqlx::query_file_as!(
        Post,
        "src/entities/post/queries/get_by_feed_new.sql",
        limit,
        offset
    )
    .fetch_all(pool)
    .await?)
}

pub async fn get_by_feed_popular(pool: &PgPool, limit: i64, offset: i64) -> Result<Vec<Post>> {
    Ok(sqlx::query_file_as!(
        Post,
        "src/entities/post/queries/get_by_feed_popular.sql",
        limit,
        offset
    )
    .fetch_all(pool)
    .await?)
}

pub async fn get_by_username(pool: &PgPool, username: &str) -> Result<Vec<Post>> {
    Ok(sqlx::query_file_as!(
        Post,
        "src/entities/post/queries/get_by_username.sql",
        username
    )
    .fetch_all(pool)
    .await?)
}

pub async fn get_liked_by_username(_pool: &PgPool, _username: &str) -> Result<Vec<Post>> {
    return Ok(vec![]);

    // Ok(
    //     sqlx::query_file_as!(Post, "src/entities/post/queries/get_liked_by_username.sql", username)
    //         .fetch_all(pool)
    //         .await?,
    // )
}
