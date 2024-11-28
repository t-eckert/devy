use super::Result;
use crate::store::Store;
use lib::blogs::Blog;
use lib::repositories::BlogRepository;
use uuid::Uuid;

pub struct BlogsController;

/// NewBlog is the data required to create a new blog.
#[allow(dead_code)]
pub struct NewBlog {
    profile_id: Uuid,
    name: String,
    slug: String,
    description: Option<String>,
}

impl BlogsController {
    /// Insert a new blog into the database.
    #[allow(dead_code)]
    pub async fn insert(store: &Store, new_blog: NewBlog) -> Result<Blog> {
        let id = BlogRepository::insert(
            &store.db_conn,
            new_blog.profile_id,
            &new_blog.name,
            &new_blog.slug,
            new_blog.description.as_deref(),
        )
        .await?;

        let blog = BlogRepository::get(&store.db_conn, id).await?;

        Ok(blog)
    }

    /// Get a blog by its ID.
    #[allow(dead_code)]
    pub async fn get_by_id(store: &Store, id: Uuid) -> Result<Blog> {
        Ok(BlogRepository::get(&store.db_conn, id).await?)
    }

    /// Get a blog by its slug.
    pub async fn get_by_slug(store: &Store, slug: &String) -> Result<Blog> {
        Ok(BlogRepository::get_by_slug(&store.db_conn, slug).await?)
    }

    /// Get blogs by a username.
    pub async fn get_by_username(store: &Store, username: &String) -> Result<Vec<Blog>> {
        Ok(BlogRepository::get_by_username(&store.db_conn, username).await?)
    }

    /// Delete a blog by its slug.
    pub async fn delete_by_slug(store: &Store, slug: &String) -> Result<()> {
        Ok(BlogRepository::delete_by_slug(&store.db_conn, slug).await?)
    }
}
