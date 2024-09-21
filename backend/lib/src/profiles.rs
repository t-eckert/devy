use crate::date::Date;
use crate::db;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Profile {
    pub id: Uuid,
    pub user_id: Uuid,

    pub display_name: Option<String>,
    pub avatar_url: Option<String>,
    pub bio: Option<String>,
    pub website: Option<String>,

    pub created_at: Date,
    pub updated_at: Date,
}

pub struct ProfileRepository;

impl ProfileRepository {
    pub async fn insert(
        db_conn: &db::Conn,
        user_id: Uuid,
        display_name: Option<String>,
        avatar_url: Option<String>,
        bio: Option<String>,
        website: Option<String>,
    ) -> db::Result<db::Id> {
        unimplemented!();
    }

    pub async fn update(db_conn: &db::Conn, profile: Profile) -> db::Result<db::Id> {
        unimplemented!();
    }

    pub async fn get(db_conn: &db::Conn, id: Uuid) -> db::Result<Profile> {
        unimplemented!();
    }

    pub async fn get_by_username(db_conn: &db::Conn, username: String) -> db::Result<Profile> {
        unimplemented!();
    }

    pub async fn delete(db_conn: &db::Conn, id: Uuid) -> db::Result<()> {
        unimplemented!();
    }
}
