use super::Result;
use crate::store::Store;
use lib::posts::{Entry, EntryRepository};
use uuid::Uuid;

pub struct EntriesController;

impl EntriesController {
    pub async fn get_drafts_by_profile_id(store: &Store, profile_id: Uuid) -> Result<Vec<Entry>> {
        Ok(EntryRepository::get_drafts_by_profile_id(&store.db_conn, profile_id).await?)
    }

    pub async fn get_by_blog_slug_and_post_slug(
        store: &Store,
        blog_slug: &str,
        post_slug: &str,
    ) -> Result<Entry> {
        Ok(
            EntryRepository::get_by_blog_slug_and_post_slug(&store.db_conn, blog_slug, post_slug)
                .await?,
        )
    }

    pub async fn get_by_blog_slug(store: &Store, blog_slug: &str) -> Result<Vec<Entry>> {
        Ok(EntryRepository::get_by_blog_slug(&store.db_conn, blog_slug).await?)
    }

    pub async fn get_by_username(store: &Store, username: &str) -> Result<Vec<Entry>> {
        Ok(EntryRepository::get_by_username(&store.db_conn, username).await?)
    }
}
