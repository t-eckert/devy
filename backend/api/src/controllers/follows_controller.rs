use super::Result;
use crate::store::Store;
use lib::blogs::{Follow, FollowRepository};
use uuid::Uuid;

pub struct FollowsController;

impl FollowsController {
    pub async fn insert(store: &Store, follow: Follow) -> Result<()> {
        Ok(FollowRepository::insert(&store.db_conn, follow).await?)
    }

    pub async fn get_by_profile_id(store: &Store, profile_id: Uuid) -> Result<Vec<Follow>> {
        Ok(FollowRepository::get_by_profile_id(&store.db_conn, profile_id).await?)
    }

    pub async fn delete(store: &Store, follow: Follow) -> Result<()> {
        Ok(FollowRepository::delete(&store.db_conn, follow).await?)
    }
}
