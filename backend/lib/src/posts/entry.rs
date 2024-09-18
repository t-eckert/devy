use super::Result;
use crate::date::Date;
use crate::db::DBConn;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Entry is similar to a post, but omits the post body itself to save space.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Entry {
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
}

pub struct EntryRepository;

impl EntryRepository {
    pub async fn get_drafts_by_profile_id(
        db_conn: &DBConn,
        profile_id: Uuid,
    ) -> Result<Vec<Entry>> {
        Ok(sqlx::query_file_as!(
            Entry,
            "queries/get_entries_where_is_draft_by_profile_id.sql",
            profile_id
        )
        .fetch_all(db_conn)
        .await?)
    }

    pub async fn get_by_blog_slug_and_post_slug(
        db_conn: &DBConn,
        blog_slug: &str,
        post_slug: &str,
    ) -> Result<Entry> {
        Ok(sqlx::query_file_as!(
            Entry,
            "queries/get_entry_by_blog_slug_and_post_slug.sql",
            blog_slug,
            post_slug
        )
        .fetch_one(db_conn)
        .await?)
    }

    pub async fn get_by_blog_slug(db_conn: &DBConn, blog_slug: &str) -> Result<Vec<Entry>> {
        Ok(
            sqlx::query_file_as!(Entry, "queries/get_entries_by_blog_slug.sql", blog_slug)
                .fetch_all(db_conn)
                .await?,
        )
    }

    pub async fn get_by_username(db_conn: &DBConn, username: &str) -> Result<Vec<Entry>> {
        Ok(
            sqlx::query_file_as!(Entry, "queries/get_entries_by_username.sql", username)
                .fetch_all(db_conn)
                .await?,
        )
    }
}
