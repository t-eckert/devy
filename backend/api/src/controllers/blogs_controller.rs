use super::Result;
use crate::{db::blog, store::Store};
use lib::entities::Blog;
use uuid::Uuid;

pub struct BlogsController;

/// NewBlog is the data required to create a new blog.
pub struct NewBlog {
    profile_id: Uuid,
    name: String,
    slug: String,
    description: Option<String>,
}

impl BlogsController {
    /// Insert a new blog into the database.
    pub async fn insert(store: &Store, new_blog: NewBlog) -> Result<Blog> {
        Ok(blog::upsert(
            &store.db_conn,
            new_blog.profile_id,
            &new_blog.name,
            &new_blog.slug,
            new_blog.description,
        )
        .await?)
    }

    /// Get a blog by its ID.
    pub async fn get_by_id(store: &Store, id: Uuid) -> Result<Blog> {
        Ok(blog::get_by_id(&store.db_conn, id).await?)
    }

    /// Get a blog by its slug.
    pub async fn get_by_slug(store: &Store, slug: &String) -> Result<Blog> {
        Ok(blog::get_by_slug(&store.db_conn, slug).await?)
    }

    /// Delete a blog by its slug.
    pub async fn delete_by_slug(store: &Store, slug: &String) -> Result<()> {
        Ok(blog::delete_by_slug(&store.db_conn, slug.to_string()).await?)
    }
}

#[cfg(test)]
mod test {
    use crate::db::DBConn;
    use crate::test::test_store;

    use super::*;

    #[sqlx::test(migrator = "lib::db::MIGRATOR")]
    async fn test_insert_will_not_work_without_existence_of_profile_reference(db_conn: DBConn) {
        let store = test_store::new_test_store(test_store::Options { db_conn });

        let new_blog = NewBlog {
            profile_id: Uuid::new_v4(),
            name: "Test Blog".to_string(),
            slug: "test-blog".to_string(),
            description: Some("This is a test blog".to_string()),
        };

        // TODO: Check the error type, not just that it errors.
        assert!(BlogsController::insert(&store, new_blog).await.is_err());
    }
}
