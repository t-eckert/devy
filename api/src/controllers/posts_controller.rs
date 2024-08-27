use super::Result;
use crate::store::Store;
use lib::db::post;
use lib::entities::Post;

pub struct PostsController;

impl PostsController {
    pub async fn get_by_blog_slug(store: &Store, blog_slug: &String) -> Result<Vec<Post>> {
        Ok(post::get_by_blog_slug(&store.db_conn, blog_slug).await?)
    }
}
