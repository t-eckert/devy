use lib::{entities::Feed, db::feed};
use super::Result;
use uuid::Uuid;
use crate::store::Store;

pub struct FeedsController;

impl FeedsController {
    /// Get a feed with the most recent posts.
    pub async fn get_recent(store: &Store) -> Result<Feed> {
        Ok(feed::get_recent(&store.db_conn).await?)
    }

    /// Get a feed with the most popular posts.
    pub async fn get_popular(store: &Store) -> Result<Feed> {
        Ok(feed::get_popular(&store.db_conn).await?)
    }

    /// Get a feed with the posts of the profiles the user follows.
    pub async fn get_following(store: &Store, profile_id: Uuid) -> Result<Feed> {
        Ok(feed::get_following(&store.db_conn, profile_id).await?)
    }
}
