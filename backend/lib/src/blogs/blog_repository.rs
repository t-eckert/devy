use super::Blog;
use crate::date::Date;
use crate::db;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

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

    pub async fn get_by_upload_id(db_conn: &db::Conn, upload_id: Uuid) -> db::Result<Blog> {
        Ok(
            sqlx::query_file_as!(Blog, "queries/get_blog_by_upload_id.sql", upload_id)
                .fetch_one(db_conn)
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
