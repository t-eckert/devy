use super::Result;
use crate::store::Store;
use lib::collection::Collection;
use uuid::Uuid;

pub struct CollectionsController;

impl CollectionsController {
    /// Get a collection with the posts liked by the user.
    pub async fn get_likes(store: &Store, profile_id: Uuid) -> Result<Collection> {
        Ok(Collection { entries: vec![] })
    }

    /// Get a collection with the posts bookmarked by the user.
    pub async fn get_bookmarks(store: &Store, profile_id: Uuid) -> Result<Collection> {
        Ok(Collection { entries: vec![] })
    }
}
