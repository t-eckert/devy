use super::Result;
use crate::store::Store;
use lib::posts::Feed;
use lib::repositories::FeedRepository;
use uuid::Uuid;

pub struct FeedsController;

impl FeedsController {
    /// Get a feed with the most recent posts.
    pub async fn get_recent(store: &Store, page: u32) -> Result<Feed> {
        Ok(FeedRepository::get_recent(&store.db_conn, page).await?)
    }

    /// Get a feed with the most popular posts.
    pub async fn get_popular(store: &Store, page: u32) -> Result<Feed> {
        Ok(FeedRepository::get_popular(&store.db_conn, page).await?)
    }

    /// Get a feed with the posts of the profiles the user follows.
    pub async fn get_following(store: &Store, profile_id: Uuid, page: u32) -> Result<Feed> {
        Ok(FeedRepository::get_following(&store.db_conn, profile_id, page).await?)
    }
}
