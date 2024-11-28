use crate::db;
use crate::posts::Entry;
use uuid::Uuid;

pub struct EntryRepository;

impl EntryRepository {
    /// Get all entries using a blog slug.
    pub async fn get_by_blog_slug(db_conn: &db::Conn, blog_slug: &str) -> db::Result<Vec<Entry>> {
        Ok(
            sqlx::query_file_as!(Entry, "queries/get_entries_by_blog_slug.sql", blog_slug)
                .fetch_all(db_conn)
                .await?,
        )
    }

    /// Get an entry using a blog and a post slug.
    pub async fn get_by_blog_slug_and_post_slug(
        db_conn: &db::Conn,
        blog_slug: &str,
        post_slug: &str,
    ) -> db::Result<Entry> {
        Ok(sqlx::query_file_as!(
            Entry,
            "queries/get_entry_by_blog_slug_and_post_slug.sql",
            blog_slug,
            post_slug
        )
        .fetch_one(db_conn)
        .await?)
    }

    /// Get all entries using a username.
    pub async fn get_by_username(db_conn: &db::Conn, username: &str) -> db::Result<Vec<Entry>> {
        Ok(
            sqlx::query_file_as!(Entry, "queries/get_entries_by_username.sql", username)
                .fetch_all(db_conn)
                .await?,
        )
    }

    /// Get all entries liked by a profile using a profile ID.
    pub async fn get_liked_by_profile_id(
        db_conn: &db::Conn,
        profile_id: Uuid,
    ) -> db::Result<Vec<Entry>> {
        Ok(sqlx::query_file_as!(
            Entry,
            "queries/get_entries_liked_by_profile_id.sql",
            profile_id
        )
        .fetch_all(db_conn)
        .await?)
    }

    /// Get all entries bookmarked by a profile using a profile ID.
    pub async fn get_bookmarked_by_profile_id(
        db_conn: &db::Conn,
        profile_id: Uuid,
    ) -> db::Result<Vec<Entry>> {
        Ok(sqlx::query_file_as!(
            Entry,
            "queries/get_entries_bookmarked_by_profile_id.sql",
            profile_id
        )
        .fetch_all(db_conn)
        .await?)
    }

    /// Get all draft entries using a profile ID.
    pub async fn get_drafts_by_profile_id(
        db_conn: &db::Conn,
        profile_id: Uuid,
    ) -> db::Result<Vec<Entry>> {
        Ok(sqlx::query_file_as!(
            Entry,
            "queries/get_entries_where_is_draft_by_profile_id.sql",
            profile_id
        )
        .fetch_all(db_conn)
        .await?)
    }
}
