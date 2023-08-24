use rocket_db_pools::Connection;

use crate::db::DB;

use super::model::Blog;
use crate::entities::{Post, Profile};
use crate::tables;

pub struct BlogController {}

impl BlogController {
    pub async fn upsert(mut db: Connection<DB>, blog: Blog) -> Option<Blog> {
        let inserted = sqlx::query_file!(
            "queries/blog_upsert.sql",
            blog.name,
            blog.slug,
            blog.username,
            blog.description
        )
        .fetch_all(&mut *db)
        .await;

        match inserted {
            Ok(_) => Self::get_by_slug(db, blog.slug).await,
            Err(e) => {
                println!("Error: {}", e);
                None
            }
        }
    }

    pub async fn get_by_slug(mut db: Connection<DB>, slug: String) -> Option<Blog> {
        let blog = sqlx::query_file_as!(Blog, "queries/blog_get_by_slug.sql", slug)
            .fetch_one(&mut *db)
            .await;

        match blog {
            Ok(blog) => Some(blog),
            Err(e) => {
                println!("Error: {}", e);
                None
            }
        }
    }
}
