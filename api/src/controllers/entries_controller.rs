use super::Result;
use crate::store::Store;
use lib::posts::{Entry, EntryRepository};
use uuid::Uuid;

pub struct EntriesController;

impl EntriesController {
    pub async fn get_drafts_by_profile_id(store: &Store, profile_id: Uuid) -> Result<Vec<Entry>> {
        Ok(EntryRepository::get_drafts_by_profile_id(&store.db_conn, profile_id).await?)
    }
}
