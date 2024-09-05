use super::Result;
use crate::store::Store;
use lib::posts::{Post, PostRepository};

pub struct PostsController;

impl PostsController {
    pub async fn get_by_blog_slug(store: &Store, blog_slug: &String) -> Result<Vec<Post>> {
        Ok(PostRepository::get_by_blog_slug(&store.db_conn, blog_slug)
            .await
            .map_err(|_| super::Error::Generic("failure".to_string()))?)
    }

    pub async fn get_by_blog_slug_and_post_slug(
        store: &Store,
        blog_slug: &String,
        post_slug: &String,
    ) -> Result<Post> {
        Ok(
            PostRepository::get_by_blog_slug_and_post_slug(&store.db_conn, blog_slug, post_slug)
                .await
                .map_err(|_| super::Error::Generic("get failed".to_string()))?,
        )
    }
}
