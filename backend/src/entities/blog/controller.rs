use rocket_db_pools::Connection;

use crate::db::DB;

use super::model::Blog;
use crate::entities::{Post, Profile};
use crate::tables;

pub struct BlogController {}

impl BlogController {
    pub async fn upsert(mut db: Connection<DB>, blog: Blog) -> Option<Blog> {
        let inserted = sqlx::query!(
            r#"
            INSERT INTO "blog" ("profile_id", "name", "slug", "description")
            VALUES (
                ( 
                    SELECT id AS profile_id FROM "profile" 
                    WHERE user_id=(SELECT id from "user" WHERE username=$3)
                ),
                $1, $2, $4
            )
            "#,
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
        let blog = sqlx::query_as!(
            Blog,
            r#"
            SELECT 
                name, slug,
                to_char(blog.created_at, 'YYYY-MM-DDThh:mm:ss.sss') AS created_at,
                to_char(blog.updated_at, 'YYYY-MM-DDThh:mm:ss.sss') AS updated_at,
                username, display_name, description
            FROM "blog" LEFT JOIN (
                SELECT profile.id, display_name, username
                FROM "profile" LEFT JOIN "user"
                ON user_id="user".id
            ) AS "profile" ON profile_id="profile".id
            WHERE slug=$1;
            "#,
            slug
        )
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
