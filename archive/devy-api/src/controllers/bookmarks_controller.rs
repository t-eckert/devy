use super::Result;
use crate::store::Store;
use lib::posts::Bookmark;
use lib::repositories::BookmarkRepository;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewBookmark {
    pub profile_id: Uuid,
    pub post_id: Uuid,
}

/// Controller for managing bookmarks.
pub struct BookmarksController;

impl BookmarksController {
    /// Create a new bookmark.
    pub async fn insert(store: &Store, bookmark: NewBookmark) -> Result<Bookmark> {
        BookmarkRepository::insert(&store.db_conn, bookmark.profile_id, bookmark.post_id).await?;
        let bookmark =
            BookmarkRepository::get(&store.db_conn, bookmark.profile_id, bookmark.post_id).await?;
        Ok(bookmark)
    }

    /// Get all bookmarks by a user by their username.
    pub async fn get_by_username(store: &Store, username: &String) -> Result<Vec<Bookmark>> {
        Ok(BookmarkRepository::get_by_username(&store.db_conn, username).await?)
    }

    /// Delete a bookmark.
    pub async fn delete(store: &Store, profile_id: Uuid, post_id: Uuid) -> Result<()> {
        Ok(BookmarkRepository::delete(&store.db_conn, profile_id, post_id).await?)
    }
}
