use crate::date::Date;
use crate::db;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Blog {
    pub id: Uuid,
    pub profile_id: Uuid,
    pub user_id: Uuid,

    pub author_username: String,
    pub author_display_name: Option<String>,

    pub name: String,
    pub slug: String,
    pub description: Option<String>,

    pub created_at: Date,
    pub updated_at: Date,
}

pub struct BlogRepository;

impl BlogRepository {
    pub async fn insert(
        db_conn: &db::Conn,
        profile_id: Uuid,
        name: &str,
        slug: &str,
        description: Option<&str>,
    ) -> db::Result<Uuid> {
        Ok(sqlx::query_file_as!(
            db::Id,
            "queries/insert_blog.sql",
            profile_id,
            name,
            slug,
            description
        )
        .fetch_one(db_conn)
        .await?
        .id)
    }

    pub async fn get(db_conn: &db::Conn, id: Uuid) -> db::Result<Blog> {
        Ok(sqlx::query_file_as!(Blog, "queries/get_blog.sql", id)
            .fetch_one(db_conn)
            .await?)
    }

    pub async fn get_by_slug(db_conn: &db::Conn, slug: &String) -> db::Result<Blog> {
        Ok(
            sqlx::query_file_as!(Blog, "queries/get_blog_by_slug.sql", slug)
                .fetch_one(db_conn)
                .await?,
        )
    }

    pub async fn get_by_username(db_conn: &db::Conn, username: &String) -> db::Result<Vec<Blog>> {
        Ok(
            sqlx::query_file_as!(Blog, "queries/get_blogs_by_username.sql", username)
                .fetch_all(db_conn)
                .await?,
        )
    }

    pub async fn delete_by_slug(db_conn: &db::Conn, slug: &String) -> db::Result<()> {
        let _ = sqlx::query_file!("queries/delete_blog_by_slug.sql", slug)
            .execute(db_conn)
            .await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[sqlx::test]
    async fn test_insert_blog(db_conn: db::Conn) {}
}
