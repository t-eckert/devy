use super::Result;
use crate::store::Store;
use lib::posts::Collection;
use lib::repositories::CollectionRepository;
use uuid::Uuid;

pub struct CollectionsController;

impl CollectionsController {
    /// Get a collection with the posts liked by the user.
    pub async fn get_likes(store: &Store, profile_id: Uuid, page: u32) -> Result<Collection> {
        Ok(CollectionRepository::get_liked(&store.db_conn, profile_id, page).await?)
    }

    /// Get a collection with the posts bookmarked by the user.
    pub async fn get_bookmarks(store: &Store, profile_id: Uuid, page: u32) -> Result<Collection> {
        Ok(CollectionRepository::get_bookmarked(&store.db_conn, profile_id, page).await?)
    }
}
