use super::Result;
use crate::store::Store;
use lib::posts::{Like, LikeRepository};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewLike {
    pub profile_id: Uuid,
    pub post_id: Uuid,
}

pub struct LikesController;

impl LikesController {
    /// Create a new like.
    pub async fn insert(store: &Store, like: NewLike) -> Result<Like> {
        LikeRepository::insert(&store.db_conn, like.profile_id, like.post_id).await?;
        let like = LikeRepository::get(&store.db_conn, like.profile_id, like.post_id).await?;
        Ok(like)
    }

    /// Get all likes by a user by their username.
    pub async fn get_by_username(store: &Store, username: &String) -> Result<Vec<Like>> {
        Ok(LikeRepository::get_by_username(&store.db_conn, username).await?)
    }

    /// Delete a like.
    pub async fn delete(store: &Store, profile_id: Uuid, post_id: Uuid) -> Result<()> {
        Ok(LikeRepository::delete(&store.db_conn, profile_id, post_id).await?)
    }
}
