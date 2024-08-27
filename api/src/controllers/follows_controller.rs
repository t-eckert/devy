use super::Result;
use crate::store::Store;
use lib::blogs::Follow;
use lib::db::follow;
use uuid::Uuid;

pub struct FollowsController;

impl FollowsController {
    pub async fn insert(store: &Store, follow: Follow) -> Result<()> {
        Ok(follow::insert(&store.db_conn, follow).await?)
    }

    pub async fn get_by_profile_id(store: &Store, profile_id: Uuid) -> Result<Vec<Follow>> {
        Ok(follow::get_by_profile_id(&store.db_conn, profile_id).await?)
    }

    pub async fn delete(store: &Store, follow: Follow) -> Result<()> {
        Ok(follow::delete(&store.db_conn, follow).await?)
    }
}
