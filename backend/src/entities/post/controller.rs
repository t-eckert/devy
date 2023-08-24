use rocket_db_pools::Connection;

use crate::db::DB;

use super::model::Post;

pub struct PostController {}

impl PostController {
    pub async fn get_by_id(db: Connection<DB>, id: String) -> Option<Post> {
        unimplemented!()
    }

    pub async fn get_by_blog_slug_and_post_slug(
        mut db: Connection<DB>,
        blog: String,
        post: String,
    ) -> Option<Post> {
        sqlx::query_file_as!(
            Post,
            "queries/post_get_by_blog_slug_and_post_slug.sql",
            blog,
            post
        )
        .fetch_one(&mut *db)
        .await
        .ok()
    }

    pub async fn get_by_blog_slug(mut db: Connection<DB>, blog_slug: String) -> Option<Vec<Post>> {
        sqlx::query_file_as!(Post, "queries/post_get_by_blog_slug.sql", blog_slug)
            .fetch_all(&mut *db)
            .await
            .ok()
    }

    pub async fn get_by_feed(db: Connection<DB>, feed_id: String) -> Option<Vec<Post>> {
        match feed_id.as_str() {
            "new" => Self::get_by_feed_new(db).await,
            _ => None,
        }
    }

    async fn get_by_feed_new(mut db: Connection<DB>) -> Option<Vec<Post>> {
        sqlx::query_file_as!(Post, "queries/post_get_by_feed_new.sql")
            .fetch_all(&mut *db)
            .await
            .ok()
    }
}
