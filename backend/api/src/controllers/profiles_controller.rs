use super::Result;
use crate::db::profile;
use crate::store::Store;
use lib::entities::Profile;
use uuid::Uuid;

pub struct ProfilesController;

pub struct NewProfile {
    user_id: Uuid,
    display_name: String,
    avatar_url: Option<String>,
}

impl ProfilesController {
    pub async fn insert(store: &Store, new_profile: NewProfile) -> Result<Profile> {
        Ok(profile::upsert(
            &store.db_conn,
            new_profile.user_id,
            new_profile.display_name,
            new_profile.avatar_url,
        )
        .await?)
    }

    pub async fn get_all(store: &Store) -> Result<Vec<Profile>> {
        Ok(vec![])
    }

    pub async fn get_by_id(store: &Store, id: Uuid) -> Result<Profile> {
        Ok(profile::get_by_id(&store.db_conn, id).await?)
    }

    pub async fn get_by_username(store: &Store, username: String) -> Result<Profile> {
        Ok(profile::get_by_username(&store.db_conn, username).await?)
    }
}
