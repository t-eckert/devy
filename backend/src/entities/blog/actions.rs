use super::*;
use sqlx::types::Uuid;
use sqlx::PgPool;

pub async fn upsert(pool: &PgPool, blog: BlogForUpsert) -> Result<Blog> {
    let _result = sqlx::query_file_as!(
        Blog,
        "src/entities/blog/queries/upsert.sql",
        blog.name,
        blog.slug,
        blog.username,
        blog.description,
    )
    .fetch_one(pool)
    .await?;

    get_by_slug(pool, blog.slug).await
}

pub async fn get_by_id(pool: &PgPool, id: Uuid) -> Result<Blog> {
    Ok(
        sqlx::query_file_as!(Blog, "src/entities/blog/queries/get_by_id.sql", id)
            .fetch_one(pool)
            .await?,
    )
}

pub async fn get_by_slug(pool: &PgPool, slug: String) -> Result<Blog> {
    Ok(
        sqlx::query_file_as!(Blog, "src/entities/blog/queries/get_by_slug.sql", slug)
            .fetch_one(pool)
            .await?,
    )
}

pub async fn get_by_username(pool: &PgPool, username: String) -> Result<Vec<Blog>> {
    Ok(sqlx::query_file_as!(
        Blog,
        "src/entities/blog/queries/get_by_username.sql",
        username
    )
    .fetch_all(pool)
    .await?)
}

pub async fn delete_by_slug(pool: &PgPool, slug: String) -> Result<()> {
    sqlx::query_file!("src/entities/blog/queries/delete_by_slug.sql", slug)
        .execute(pool)
        .await?;

    Ok(())
}
