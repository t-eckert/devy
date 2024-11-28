use crate::db;
use crate::posts::Post;
use uuid::Uuid;

pub struct PostRepository;

impl PostRepository {
    pub async fn insert(
        db_conn: &db::Conn,
        blog_id: Uuid,
        title: &String,
        slug: &String,
        body: &String,
        is_draft: bool,
    ) -> db::Result<Uuid> {
        let id = sqlx::query_file_as!(
            db::Id,
            "queries/insert_post.sql",
            blog_id,
            title,
            slug,
            body,
            is_draft
        )
        .fetch_one(db_conn)
        .await?
        .id;

        Ok(id)
    }

    /// Update a post in the database.
    /// Updates the post with the given id to modify slug, title, is_draft, and body.
    pub async fn update(db_conn: &db::Conn, post: &Post) -> db::Result<Uuid> {
        let _ = sqlx::query_file!(
            "queries/update_post.sql",
            post.id,
            post.slug,
            post.title,
            post.is_draft,
            post.body
        )
        .execute(db_conn)
        .await?;

        Ok(post.id)
    }

    pub async fn get_by_blog_slug(db_conn: &db::Conn, blog_slug: &String) -> db::Result<Vec<Post>> {
        Ok(
            sqlx::query_file_as!(Post, "queries/get_posts_by_blog_slug.sql", blog_slug)
                .fetch_all(db_conn)
                .await?,
        )
    }

    pub async fn get_by_blog_slug_and_post_slug(
        db_conn: &db::Conn,
        blog_slug: &str,
        post_slug: &str,
    ) -> db::Result<Post> {
        Ok(sqlx::query_file_as!(
            Post,
            "queries/get_post_by_blog_slug_and_post_slug.sql",
            blog_slug,
            post_slug
        )
        .fetch_one(db_conn)
        .await?)
    }

    pub async fn delete(db_conn: &db::Conn, id: Uuid) -> db::Result<()> {
        let _ = sqlx::query_file!("queries/delete_post.sql", id)
            .execute(db_conn)
            .await?;

        Ok(())
    }
}
