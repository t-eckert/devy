use crate::db::{Database, Result};
use crate::entities::Blog;
use uuid::Uuid;
use super::Relation;

pub async fn upsert(
    db: &Database,
    profile_id: Uuid,
    name: &str,
    slug: &str,
    description: Option<String>,
) -> Result<Blog> {
    Ok(sqlx::query_file_as!(
        Relation,
        "src/db/blog/queries/upsert.sql",
        profile_id,
        name.to_string(),
        slug.to_string(),
        description,
    )
    .fetch_one(db)
    .await?
    .try_into()?)
}

pub async fn get_by_id(db: &Database, id: Uuid) -> Result<Blog> {
    Ok(
        sqlx::query_file_as!(Blog, "src/db/blog/queries/get_by_id.sql", id)
            .fetch_one(db)
            .await?,
    )
}

pub async fn get_by_slug(db: &Database, slug: String) -> Result<Blog> {
    Ok(
        sqlx::query_file_as!(Blog, "src/db/blog/queries/get_by_slug.sql", slug)
            .fetch_one(db)
            .await?,
    )
}

pub async fn get_by_username(db: &Database, username: String) -> Result<Vec<Blog>> {
    Ok(
        sqlx::query_file_as!(Blog, "src/db/blog/queries/get_by_username.sql", username)
            .fetch_all(db)
            .await?,
    )
}

pub async fn delete_by_slug(db: &Database, slug: String) -> Result<()> {
    sqlx::query_file!("src/db/blog/queries/delete_by_slug.sql", slug)
        .execute(db)
        .await?;

    Ok(())
}
