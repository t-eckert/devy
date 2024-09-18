use crate::date::Date;
use crate::db;
use crate::db::DBConn;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Post represents a blog post.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Post {
    /// The unique identifier for the post.
    pub id: Uuid,
    /// The slug of the post.
    pub slug: String,

    /// The slug of the blog that the post belongs to.
    pub blog_slug: String,
    /// The name of the blog that the post belongs to.
    pub blog_name: Option<String>,

    /// The slug of the profile of the author of the post.
    pub author_slug: Option<String>,
    /// The name of the author of the post.
    pub author_name: Option<String>,

    // The date the post was created.
    pub created_at: Date,
    // The date the post was last updated.
    pub updated_at: Date,

    /// The title of the post.
    pub title: String,
    /// The number of likes the post has.
    pub like_count: Option<i32>,
    /// Whether the post is a draft.
    pub is_draft: bool,

    /// The body of the post.
    pub body: String,
}

pub async fn get_by_blog_slug_and_post_slug(
    db_conn: &DBConn,
    blog_slug: &String,
    post_slug: &String,
) -> Result<Post, anyhow::Error> {
    Ok(sqlx::query_file_as!(
        Post,
        "queries/get_post_by_blog_slug_and_post_slug.sql",
        blog_slug,
        post_slug
    )
    .fetch_one(db_conn)
    .await?)
}

pub struct PostRepository;

impl PostRepository {
    pub async fn insert(
        db_conn: &DBConn,
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
    pub async fn update(db_conn: &DBConn, post: &Post) -> db::Result<Uuid> {
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

    pub async fn get_by_blog_slug(db_conn: &DBConn, blog_slug: &String) -> db::Result<Vec<Post>> {
        Ok(
            sqlx::query_file_as!(Post, "queries/get_posts_by_blog_slug.sql", blog_slug)
                .fetch_all(db_conn)
                .await?,
        )
    }

    pub async fn get_by_blog_slug_and_post_slug(
        db_conn: &DBConn,
        blog_slug: &String,
        post_slug: &String,
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

    pub async fn delete(db_conn: &DBConn, id: Uuid) -> db::Result<()> {
        let _ = sqlx::query_file!("queries/delete_post.sql", id)
            .execute(db_conn)
            .await?;

        Ok(())
    }
}
