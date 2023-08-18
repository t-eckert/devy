use rocket_db_pools::Connection;

use crate::db::DB;

use super::model::Blog;
use crate::tables;

pub struct BlogController {}

impl BlogController {
    pub async fn upsert(mut db: Connection<DB>, blog: Blog) -> Option<Blog> {
        let blog = sqlx::query_as!(
            tables::Blog,
            "INSERT INTO blog (profile_id, name, slug) VALUES ($1, $2, $3) RETURNING *",
            blog.profile_id,
            blog.name,
            blog.slug
        )
        .fetch_one(&mut *db)
        .await;

        match blog {
            Ok(blog) => Some(Blog {
                id: Some(blog.id),
                profile_id: blog.profile_id,
                name: blog.name,
                slug: blog.slug,
            }),
            Err(e) => {
                println!("Error: {}", e);
                None
            }
        }
    }

    pub async fn get_by_slug(mut db: Connection<DB>, slug: String) -> Option<Blog> {
        let blog = sqlx::query_as!(tables::Blog, "SELECT * FROM blog WHERE slug = $1", slug)
            .fetch_one(&mut *db)
            .await;

        match blog {
            Ok(blog) => Some(Blog {
                id: Some(blog.id),
                profile_id: blog.profile_id,
                name: blog.name,
                slug: blog.slug,
            }),
            Err(e) => {
                println!("Error: {}", e);
                None
            }
        }
    }
}
