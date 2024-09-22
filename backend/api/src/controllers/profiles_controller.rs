use super::Result;
use crate::store::Store;
use lib::profiles::{Profile, ProfileRepository};
use uuid::Uuid;

pub struct ProfilesController;

pub struct NewProfile {
    user_id: Uuid,
    display_name: Option<String>,
    avatar_url: Option<String>,
}

impl ProfilesController {
    pub async fn insert(store: &Store, new_profile: NewProfile) -> Result<Profile> {
        let id = ProfileRepository::insert(
            &store.db_conn,
            new_profile.user_id,
            new_profile.display_name,
            new_profile.avatar_url,
            None,
            None,
        )
        .await?;

        Ok(ProfileRepository::get(&store.db_conn, id).await?)
    }

    pub async fn get_all(store: &Store) -> Result<Vec<Profile>> {
        Ok(vec![])
    }

    pub async fn get_by_id(store: &Store, id: Uuid) -> Result<Profile> {
        Ok(ProfileRepository::get(&store.db_conn, id).await?)
    }

    pub async fn get_by_username(store: &Store, username: String) -> Result<Profile> {
        Ok(ProfileRepository::get_by_username(&store.db_conn, &username).await?)
    }
}
