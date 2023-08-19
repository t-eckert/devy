use rocket_db_pools::Connection;

use crate::db::DB;

use super::model::Blog;
use crate::entities::{Post, Profile};
use crate::tables;

pub struct BlogController {}

impl BlogController {
    pub async fn upsert(mut db: Connection<DB>, blog: Blog) -> Option<Blog> {
        let blog = sqlx::query_as!(
            tables::Blog,
            r#"
            INSERT INTO "blog" ( name, slug) 
            VALUES ($1, $2) 
            RETURNING *;
            "#,
            // blog.profile.id,
            blog.name,
            blog.slug
        )
        .fetch_one(&mut *db)
        .await;

        let author = Profile {
            id: 1,
            name: "Dorthea".to_string(),
            slug: "dorthea".to_string(),
        };
        match blog {
            Ok(blog) => Some(Blog {
                name: blog.name,
                slug: blog.slug,
                profile: author,
                posts: vec![],
            }),
            Err(e) => {
                println!("Error: {}", e);
                None
            }
        }
    }

    pub async fn get_by_slug(mut db: Connection<DB>, slug: String) -> Option<Blog> {
        let blog = sqlx::query_as!(
            tables::Blog,
            r#"
            SELECT * FROM "blog" 
            WHERE slug = $1
            "#,
            slug
        )
        .fetch_one(&mut *db)
        .await;

        let post_query_response = sqlx::query_as!(
            tables::Post,
            r#"
            SELECT * FROM "post" 
            WHERE blog_id = $1
            "#,
            blog.as_ref().unwrap().id
        )
        .fetch_all(&mut *db)
        .await;

        let author = Profile {
            id: 1,
            name: "Dorthea".to_string(),
            slug: "dorthea".to_string(),
        };

        let posts: Vec<Post> = match post_query_response {
            Ok(posts) => posts,
            Err(e) => {
                println!("Error: {}", e);
                vec![]
            }
        }
        .iter()
        .map(|post| Post {
            id: post.id,
            slug: post.slug.clone(),
            title: post.title.clone(),
            author: author.clone(),
            content: post.body.clone(),
            created_at: post.created_at.to_string(),
            updated_at: post.updated_at.to_string(),
            tags: vec![],
            likes: 3,
        })
        .collect();

        match blog {
            Ok(blog) => Some(Blog {
                name: blog.name,
                slug: blog.slug,
                profile: author,
                posts,
            }),
            Err(e) => {
                println!("Error: {}", e);
                None
            }
        }
    }
}
