
use crate::db::{DBConn, Result};
use crate::entities::Blog;
use uuid::Uuid;

pub async fn upsert(
    db_conn: &DBConn,
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
    .fetch_one(db_conn)
    .await?
    .try_into()?)
}

pub async fn get_by_id(db_conn: &DBConn, id: Uuid) -> Result<Blog> {
    Ok(
        sqlx::query_file_as!(Blog, "src/db/blog/queries/get_by_id.sql", id)
            .fetch_one(db_conn)
            .await?,
    )
}

pub async fn get_by_slug(db_conn: &DBConn, slug: &String) -> Result<Blog> {
    Ok(
        sqlx::query_file_as!(Blog, "src/db/blog/queries/get_by_slug.sql", slug)
            .fetch_one(db_conn)
            .await?,
    )
}

pub async fn get_by_username(db_conn: &DBConn, username: String) -> Result<Vec<Blog>> {
    Ok(
        sqlx::query_file_as!(Blog, "src/db/blog/queries/get_by_username.sql", username)
            .fetch_all(db_conn)
            .await?,
    )
}

pub async fn delete_by_slug(db_conn: &DBConn, slug: String) -> Result<()> {
    sqlx::query_file!("src/db/blog/queries/delete_by_slug.sql", slug)
        .execute(db_conn)
        .await?;

    Ok(())
}


use crate::db::{Error,};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Relation {
    pub id: Option<Uuid>,
    pub profile_id: Option<Uuid>,
    pub user_id: Option<Uuid>,

    pub name: Option<String>,
    pub slug: Option<String>,
    pub description: Option<String>,

    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

impl TryFrom<Relation> for Blog {
    type Error = Error;

    fn try_from(r: Relation) -> Result<Blog> {
        Ok(Blog{
            id: r.id.ok_or(Error::missing_field("id"))?,
            profile_id: r.profile_id.ok_or(Error::missing_field("profile_id"))?,
            user_id: r.user_id.ok_or(Error::missing_field("user_id"))?,

            name: r.name.ok_or(Error::missing_field("name"))?,
            slug: r.slug.ok_or(Error::missing_field("slug"))?,
            description: r.description,

            created_at: r.created_at,
            updated_at: r.updated_at,
        })
    }
}
